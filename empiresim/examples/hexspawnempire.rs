use std::time::{Duration, Instant};
use noise::{NoiseFn, self};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Vector2};
use rand::{Rng};
use raylib::prelude::*;
use simulation::hex_geom::{AxialCoord, HexGeometry, OffsetCoord};

fn empire_id_to_color(eid: u8) -> Color {
    let layer: u8 = 13;
    let mut id = eid - 2;
    let mut sat = 1.0;
    let mut val = 1.0;
    while id > layer {
        sat *= 0.8;
        val *= 0.9;
        id -= 60;
    }
    let hue = 360.0 / (layer as f32) * id as f32;
    Color::color_from_hsv(hue, sat, val)
}

#[derive(Clone, Debug)]
pub struct Empire {
    id: STATE,
    name: String,
}

pub struct HexSimulation<'a> {
    geo: &'a HexGeometry,
    pub states: Vec<Vec<STATE>>, // [y:0-rows][x:0-cols]. External is the row, internal is col/cell
    empires: [Option<Empire>; 253], // empire id is position in the vector + 2
}

// STATE:
type STATE = u8;
const SEA: STATE = 0;
const EARTH: STATE = 1;

impl<'a> HexSimulation<'a> {
    pub fn new(geo: &'a HexGeometry) -> Self {
        let mut states = vec![vec![SEA; geo.cols]; geo.rows];
        let noise = noise::Fbm::<noise::Perlin>::new(2);
        for (y, row) in states.iter_mut().enumerate() {
            for (x, state) in row.iter_mut().enumerate() {
                let lowest_max = [geo.cols, geo.rows].into_iter().min().unwrap() as f64;
                // Generally noises are made to work between -1 and 1 and output -1 to 1
                *state = if noise.get([x as f64/lowest_max,y as f64/lowest_max]) >= 0.0 { EARTH } else { SEA };
            }
        }
        HexSimulation {
            geo,
            states,
            empires: [const { None }; 253]
        }
    }

    pub fn step(&mut self) {
        println!("Step");
        let prev_state = self.states.clone();
        for (y, row) in self.states.iter_mut().enumerate() {
            for (x, state) in row.iter_mut().enumerate() {
                if state != &EARTH { continue }
                let neighbours = self.geo.neighbours(OffsetCoord{x: x as isize,y: y as isize}.axial());
                // Propagate empire to nearby free cells
                #[derive(Copy, Clone)]
                struct NEmp {
                    id: u8,
                    qty: u8,
                }
                let mut empires: [Option<NEmp>; 6] = [None; 6];
                for axcord in neighbours.iter() {
                    let o = axcord.offset();
                    match prev_state[o.y as usize][o.x as usize] {
                        SEA | EARTH => { continue }
                        empire_id => {
                            // One little-known effect of flatten is to transform nested option iterators:
                            // [None, Some(a), None, Some(b) => [a, b]
                            if let Some(empire) = empires.iter_mut().flatten().find(|nemp| { nemp.id == empire_id }) {
                                empire.qty += 1;
                            } else if let Some(empire) = empires.iter_mut().find(|e| e.is_none()) {
                                *empire = Some(NEmp{ id: empire_id, qty: 1 });
                            } else {
                                panic!("More than 6 neighbour empire at x,y {},{}", o.x, o.y);
                            }
                        }
                    }
                }
                // If several items are equal by key, the last is returned
                let chosen_empire = empires.iter().max_by_key(|ne| {
                    if let Some(ne) = ne {
                        ne.qty
                    } else { 0 }
                }).expect("Empires cannot be empty");
                if let Some(nemp) = chosen_empire {
                    *state = nemp.id;
                } else {
                    // No empire nearby
                    continue;
                }
            }
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        let w = self.geo.hex_width();
        let h = self.geo.hex_height();
        for (y, row) in self.states.iter().enumerate() {
            let row_y_center = 0.5*h + (0.75*h)*y as f32;
            let row_offset = if y % 2 == 0 { 0.5*w } else { w };
            for (x, state) in row.iter().enumerate() {
                let col_x_center = row_offset + x as f32*w;
                let color = match *state {
                    SEA => Color::CYAN,
                    EARTH => Color::DARKGRAY,
                    x => {
                        empire_id_to_color(x)
                    },
                };
                d.draw_poly(Vector2{x: col_x_center, y: row_y_center}, 6, self.geo.size, 90.0, color);
                d.draw_poly_lines(Vector2{x: col_x_center, y: row_y_center}, 6, self.geo.size, 90.0, Color::GRAY)
            }
        }
        let mut rect = self.geo.rect();
        let line_thick: f32 = 10.0;
        rect.x -= line_thick;
        rect.width += 2.0*line_thick;
        rect.y -= line_thick;
        rect.height += 2.0*line_thick;
        d.draw_rectangle_lines_ex(rect, line_thick, Color::GRAY.alpha(0.5))
    }

    fn find_free_earth(&mut self) -> Option<(usize, usize)> {
        let mut earth_cells = Vec::new();
        let mut rng = rand::rng();
        let attempts = 100;

        for _ in 0..attempts {
            let x = rng.random_range(0..self.states[0].len());
            let y = rng.random_range(0..self.states.len());
            if self.states[y][x] == EARTH {
                return Some((x, y));
            }
        }
        eprintln!("Failed montecarlo new random empire, gallback to full search");

        // Fallback to full search if random sampling fails
        for (y, row) in self.states.iter().enumerate() {
            for (x, &state) in row.iter().enumerate() {
                if state == EARTH {
                    earth_cells.push((x, y));
                }
            }
        }

        if !earth_cells.is_empty() {
            let (x, y) = earth_cells[rng.random_range(0..earth_cells.len())];
            return Some((x, y));
        }
        None
    }

    pub fn interact_new_random_empire(&mut self) {
        if let Some((x, y)) = self.find_free_earth() {
            if let Some((idx, empire)) = self.empires.iter_mut().enumerate().find(|(_idx, e)| e.is_none()) {
                let state = (idx + 2) as STATE;
                *empire = Some(Empire{ id: state, name: format!("Empire {}", state) });
                self.states[y][x] = state;
                println!("Empire generated")
            } else {
                eprintln!("No free empire slot for new empire");
            }
        } else {
            eprintln!("No free earth for new empire");
        }
    }

    pub fn get_empire_by_pos(&self, pos: Vector2) -> Option<Empire> {
        // Invert the scaling, apply origin offset and scale for the algo
        let x = (pos.x - 0.5*self.geo.hex_width()) / (self.geo.size * f32::sqrt(3.0));
        let y = (-pos.y+ 0.5*self.geo.hex_height()) / (self.geo.size * f32::sqrt(3.0)) ;
        // Cartesian to Hex - Apply Charles Chamber algo https://www.redblobgames.com/grids/hexagons/more-pixel-to-hex.html#charles-chambers
        let temp = f32::floor(x + f32::sqrt(3.0) * y + 1.0);
        let q = f32::floor((f32::floor(2.0 * x + 1.0) + temp) / 3.0) as isize;
        let r = f32::floor((temp + f32::floor(-x + f32::sqrt(3.0) * y + 1.0)) / 3.0) as isize;
        let o = AxialCoord{ q, r: -r }.offset();
        if o.x < 0 || o.x >= self.geo.cols as isize || o.y < 0 || o.y >= self.geo.rows as isize {
            println!("No state found");
            return None;
        }
        let state = self.states[o.y as usize][o.x as usize];
        if state > EARTH {
            println!("Empire with state {}", state);
            self.empires.get(state as usize-2)?.clone()
        } else {
            println!("No empire");
            None
        }
    }
}

struct CameraSettings{
    pos_speed: f32,
    start: Vector2,
    end: Vector2,
    zoom_ln_speed: f32,
    zoom_ln_min: f32,
    zoom_ln_max: f32,
}

//noinspection DuplicatedCode
fn my_camera_update(camera: &mut Camera2D, rl: &mut RaylibHandle, s: &CameraSettings) {
    let frame_dt = rl.get_frame_time();
    // Scale all speed on the effective frame DT to have uniform movement at all framerate
    let speed = s.pos_speed / camera.zoom * frame_dt;
    let translation = Vector2::new(
        (rl.is_key_down(KeyboardKey::KEY_D) || rl.is_key_down(KeyboardKey::KEY_RIGHT)) as i32 as f32 * speed -
            (rl.is_key_down(KeyboardKey::KEY_A) || rl.is_key_down(KeyboardKey::KEY_LEFT)) as i32 as f32 * speed,
        (rl.is_key_down(KeyboardKey::KEY_S) || rl.is_key_down(KeyboardKey::KEY_DOWN)) as i32 as f32 * speed -
            (rl.is_key_down(KeyboardKey::KEY_W) || rl.is_key_down(KeyboardKey::KEY_UP)) as i32 as f32 * speed,
    );

    camera.target += translation;
    // Box
    if camera.target.x < s.start.x { camera.target.x = s.start.x; }
    else if camera.target.x > s.end.x { camera.target.x = s.end.x; }
    if camera.target.y < s.start.y { camera.target.y = s.start.y; }
    else if camera.target.y > s.end.y { camera.target.y = s.end.y; }

    let mouse_wheel = rl.get_mouse_wheel_move();
    // Uses log scaling to provide a consistent zoom speed
    let mut camera_ln = camera.zoom.ln() + mouse_wheel * s.zoom_ln_speed * frame_dt;
    if camera_ln > s.zoom_ln_max {
        camera_ln = s.zoom_ln_max;
    } else if camera_ln < s.zoom_ln_min {
        camera_ln = s.zoom_ln_min;
    }
    camera.zoom = f32::exp(camera_ln);
}

#[derive(Clone)]
enum OverlayState {
    NotClicked,
    Clicked{
        when: Instant,
        mouse_pos: Vector2,
        empire_id: STATE  // TODO: Keep copy of ID into Empire and use that around
    }
}

struct Overlay {
    pub timeout: Duration,
    state: OverlayState,
}

impl Overlay {
    fn new(timeout: Duration) -> Self {
        Self{ timeout, state: OverlayState::NotClicked }
    }

    fn draw_overlay(&mut self, d: &mut RaylibDrawHandle, sim: &HexSimulation, camera: &Camera2D) {
        use OverlayState::*;
        // QUESTION: Can be split into interact + draw
        // INTERACT with state
        let now = Instant::now();
        if let Clicked{when, ..} = &self.state {
            if now - *when > self.timeout {
                self.state = NotClicked;
            }
        }
        if d.is_mouse_button_pressed(MouseButton::MOUSE_BUTTON_LEFT) {
            let mouse = d.get_mouse_position();
            let empire = sim.get_empire_by_pos(d.get_screen_to_world2D(mouse, camera));
            if let Some(e) = empire {
                println!("Clicked");
                self.state = Clicked {
                    when: now,
                    mouse_pos: mouse,
                    empire_id: e.id,
                };
            }
        }
        // DRAW based on state
        if let Clicked{ mouse_pos, empire_id, .. } = &self.state {
            if let Some(Some(e)) = sim.empires.get((empire_id-2) as usize) {
                let font_size = 10;
                let w = d.measure_text(e.name.as_str(), font_size)+10;
                let h = font_size*2;
                let x = mouse_pos.x.round() as i32;
                let y = mouse_pos.y.round() as i32;
                d.draw_rectangle(x, y-h, w, h, Color::RAYWHITE);
                d.draw_rectangle_lines(x, y-h, w, h, Color::BLACK);
                d.draw_text(e.name.as_str(), x+5, y-h+font_size/2, font_size, Color::BLACK);
            } else {
                eprintln!("Invalid overlay state?");
            }
        }
    }
}


fn main() {
    // Initialization
    //--------------------------------------------------------------------------------------
    const SCREEN_WIDTH: i32 = 1600;
    const SCREEN_HEIGHT: i32 = 800;

    // Raylib Init
    let (mut rl, thread) = init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("SQUALONE SQUALOTTO QUANTO E' BELLO")
        .build();

    // Init Simulation
    let geo = HexGeometry::new(
        Vector2{x: 0.0, y: 0.0},
        200,
        100,
        50.0
    );
    let mut sim = HexSimulation::new(&geo);

    let rect = geo.rect();

    let mut camera = Camera2D {
        offset: Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0),
        target: Vector2{x: rect.x+0.5*rect.width, y: rect.y+0.5*rect.height},
        rotation: 0.0,
        zoom: f32::exp(-2.4),
    };

    let camera_settings = CameraSettings{
        pos_speed: 300.0,
        start: Vector2{x: rect.x, y: rect.y},
        end: Vector2{x: rect.x+rect.width, y: rect.y+rect.height},
        zoom_ln_speed: 6.0,
        zoom_ln_min: -3.0,
        zoom_ln_max: 1.0,
    };

    // GUI State - Overlay
    let mut overlay = Overlay::new(Duration::from_secs(3));

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        // Update
        //----------------------------------------------------------------------------------
        my_camera_update(&mut camera, &mut rl, &camera_settings);
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            sim.interact_new_random_empire();
            // for row in sim.states.iter_mut() {
            //     for s in row.iter_mut() {
            //         *s = if rng.random::<f32>() < 0.3 { 1 } else { 0 };
            //     }
            // }
        }
        if rl.is_key_down(KeyboardKey::KEY_BACKSPACE) {
            sim.step();
        }

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        // 2D Draw
        {
            let mut d2d = d.begin_mode2D(camera);
            sim.draw(&mut d2d);
        }

        d.draw_fps(10, 10);

        // Overlay
        overlay.draw_overlay(&mut d, &sim, &camera);

        // Step
        // sim.step();
    }
    // De-Initialization
    //--------------------------------------------------------------------------------------
}