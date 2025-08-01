use noise::{NoiseFn, self, Seedable};
use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Vector2};
use rand::Rng;
use raylib::prelude::*;
use simulation::hex_geom::{HexGeometry, OffsetCoord};

pub struct HexSimulation<'a> {
    geo: &'a HexGeometry,
    pub states: Vec<Vec<STATE>>, // [y:0-rows][x:0-cols]. External is the row, internal is col/cell
    birth: Vec<u8>,
    stay: Vec<u8>,
}

// STATE:
type STATE = u8;
const SEA: STATE = 0;
const EARTH: STATE = 1;
type EMPIRE = u8;
fn empire(s: STATE) -> Option<EMPIRE> { if s >= 2 { Some(s) } else { None } }

impl<'a> HexSimulation<'a> {
    pub fn new(geo: &'a HexGeometry, birth: &[u8], stay: &[u8]) -> Self {
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
            birth: birth.to_vec(),
            stay: stay.to_vec(),
        }
    }

    pub fn step(&mut self) {
        let prev_state = self.states.clone();
        for (y, row) in self.states.iter_mut().enumerate() {
            for (x, state) in row.iter_mut().enumerate() {
                if state != &EARTH { continue }
                let neighbours = self.geo.neighbours(OffsetCoord{x: x as isize,y: y as isize}.axial());
                // Sea cells nearby AND Greatest neighbour value
                let nearby = neighbours.iter().fold((0_u8, 0_u8), |acc, axcord| {
                    let o = axcord.offset();
                    let acc_sea = if prev_state[o.y as usize][o.x as usize] == SEA { acc.0+1 } else { acc.0 };
                    let acc_great = if prev_state[o.y as usize][o.x as usize] > acc.1 { prev_state[o.y as usize][o.x as usize] } else { acc.1 };
                    (acc_sea, acc_great)
                });
                if nearby.0 != 0 {
                    *state = 2;
                } else if nearby.1 >= 2 {
                    *state = nearby.1 + 1;
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
                        let cidx = 255 - x.saturating_mul(10);
                        Color::new(cidx, 0, 0, 255)
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
}

struct CameraSettings{
    basespeed: f32,
    start: Vector2,
    end: Vector2,
}

//noinspection DuplicatedCode
fn my_camera_update(camera: &mut Camera2D, rl: &mut RaylibHandle, s: &CameraSettings) {
    let speed = s.basespeed / camera.zoom;
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
    // Uses log scaling to provide consistent zoom speed
    camera.zoom = f32::exp(camera.zoom.ln() + mouse_wheel * 0.1);
    if camera.zoom > 3.0 { camera.zoom = 3.0; }
    else if camera.zoom < 0.04 { camera.zoom = 0.04; }
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
    let mut sim = HexSimulation::new(&geo, &[2], &[3,5]); //no B3/S2,3 but B2/S3,5
    let mut rng = rand::rng();
    // for row in sim.states.iter_mut() {
    //     for s in row.iter_mut() {
    //         *s = if rng.random::<f32>() < 0.3 { 1 } else { 0 };
    //     }
    // }

    let rect = geo.rect();

    let mut camera = Camera2D {
        offset: Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0),
        target: Vector2{x: rect.x+0.5*rect.width, y: rect.y+0.5*rect.height},
        rotation: 0.0,
        zoom: 0.1,
    };

    let camera_settings = CameraSettings{
        basespeed: 40.0,
        start: Vector2{x: rect.x, y: rect.y},
        end: Vector2{x: rect.x+rect.width, y: rect.y+rect.height},
    };

    rl.set_target_fps(10);

    while !rl.window_should_close() {
        // Update
        //----------------------------------------------------------------------------------
        my_camera_update(&mut camera, &mut rl, &camera_settings);
        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            for row in sim.states.iter_mut() {
                for s in row.iter_mut() {
                    *s = if rng.random::<f32>() < 0.3 { 1 } else { 0 };
                }
            }
        }

        // Draw
        //----------------------------------------------------------------------------------
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        // 2D Draw
        {
            let mut d2d = d.begin_mode2D(camera);
            // DRAW AXIS
            // d2d.draw_line(0, 0, 100, 0, Color::RED);
            // d2d.draw_line(0, 0, 0, 100, Color::BLUE);
            // d2d.draw_rectangle_rec(Rectangle{x:100.0, y:0.0, width:5.0, height:5.0}, Color::RED);
            // d2d.draw_rectangle_rec(Rectangle{x:0.0, y:100.0, width:5.0, height:5.0}, Color::BLUE);
            // END DRAW AXIS
            sim.draw(&mut d2d);
        }

        d.draw_fps(10, 10);
        // Step?
        sim.step();
    }
    // De-Initialization
    //--------------------------------------------------------------------------------------
}