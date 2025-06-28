use rand::Rng;
use raylib::prelude::*;
use simulation::geometry::Geometry;
use simulation::rect_geom::RectGeometry;
use simulation::conway_sim::{draw_gol_rect, ConwaySimulation};
/*
 * Il piano della muerte e' finire tutto questo in 5 giorni.
 * L'obiettivo e' avere una prima mappa esagonale su cui far spannare la mia simulazione.
 * Una domanda interessante e' qual e' il migliore approccio per iniziare a lavorare al problema?
 * La soluzione migliore che mi viene in mente e' iniziare con una mappa QUALUNQUE e scrivere l'algoritmo
 * a parte, classizzato, in modo che possa funzionare con qualunque mappa tramite una classe topologica.
 */

// Map data
// 0,0 at lower xy value, xcells,ycells at higher xy value
// TODO: Rimuovere queste costanti, e trovare il modo per la camera di prendere
//  gli xstart/xend ystart/yend dalla geometria, o comunque calcolati al first boot.
const MAP_XCENTER: f32 = 0.0;
const MAP_YCENTER: f32 = 0.0;
const XCELLS: i32 = 300;
const YCELLS: i32 = 200;
const XCELSIZE: f32 = 100.0;
const YCELSIZE: f32 = 100.0;

const XSTART: f32 = MAP_XCENTER - (XCELLS as f32) * XCELSIZE / 2.0;
const YSTART: f32 = MAP_YCENTER - (YCELLS as f32) * YCELSIZE / 2.0;

const XEND: f32 = XSTART + (XCELLS as f32) * XCELSIZE;
const YEND: f32 = YSTART + (YCELLS as f32) * YCELSIZE;

const BASECAMERASPEED: f32 = 5.0;

fn my_camera_update(camera: &mut Camera2D, rl: &mut RaylibHandle) {
    let speed = BASECAMERASPEED / camera.zoom;
    let translation = Vector2::new(
        (rl.is_key_down(KeyboardKey::KEY_D) || rl.is_key_down(KeyboardKey::KEY_RIGHT)) as i32 as f32 * speed -
            (rl.is_key_down(KeyboardKey::KEY_A) || rl.is_key_down(KeyboardKey::KEY_LEFT)) as i32 as f32 * speed,
        (rl.is_key_down(KeyboardKey::KEY_S) || rl.is_key_down(KeyboardKey::KEY_DOWN)) as i32 as f32 * speed -
            (rl.is_key_down(KeyboardKey::KEY_W) || rl.is_key_down(KeyboardKey::KEY_UP)) as i32 as f32 * speed,
    );

    camera.target += translation;
    // Box
    if camera.target.x < XSTART { camera.target.x = XSTART; }
    else if camera.target.x > XEND { camera.target.x = XEND; }
    if camera.target.y < YSTART { camera.target.y = YSTART; }
    else if camera.target.y > YEND { camera.target.y = YEND; }

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

    let mut camera = Camera2D {
        offset: Vector2::new(SCREEN_WIDTH as f32 / 2.0, SCREEN_HEIGHT as f32 / 2.0),
        target: Vector2::zero(),
        rotation: 0.0,
        zoom: 0.04,
    };

    // Init Simulation
    let geometry = RectGeometry::new(
        Vector2::new(0.0, 0.0),
        300,
        200,
        Vector2::new(100.0, 100.0)
    );
    let mut sim = ConwaySimulation::new(&geometry, &[3], &[2,3]);
    let mut rng = rand::rng();
    for i in 0..geometry.size() {
        if let Some(state) = sim.get_mut(i) {
            state.val = if rng.random::<f32>() < 0.3 { 1 } else { 0 };
        }
    }

    rl.set_target_fps(10);

    while !rl.window_should_close() {
        // Update
        //----------------------------------------------------------------------------------
        my_camera_update(&mut camera, &mut rl);
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
            // draw_2d_map(&mut d2d);
            draw_gol_rect(&mut d2d, &sim);
        }

        d.draw_fps(10, 10);
        // Step?
        sim.step();
    }
    // De-Initialization
    //--------------------------------------------------------------------------------------
}