use rand::Rng;
use raylib::prelude::*;
use simulation::hex_conway_sim::HexConwaySimulation;
use simulation::hex_geom::HexGeometry;

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
    let mut sim = HexConwaySimulation::new(&geo, &[2], &[3,5]); //no B3/S2,3 but B2/S3,5
    let mut rng = rand::rng();
    for row in sim.states.iter_mut() {
        for s in row.iter_mut() {
            *s = if rng.random::<f32>() < 0.3 { 1 } else { 0 };
        }
    }

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