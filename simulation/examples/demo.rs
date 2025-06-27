use simulation::{Simulation, rect_geom::RectGeometry};
use raylib::prelude::*;
use rand::Rng;
use std::{thread, time::Duration};
use simulation::geometry::Geometry;

fn print_grid(sim: &Simulation<RectGeometry>, width: usize, height: usize) {
    println!("\x1B[2J\x1B[1;1H"); // Clear screen and move cursor to top
    for y in 0..height {
        for x in 0..width {
            let id = y * width + x;
            if let Some(state) = sim.get(id) {
                print!("{}", if state.val > 0 { "█" } else { " " });
            }
        }
        println!();
    }
    println!();
}

fn main() {
    let width = 80;
    let height = 40;
    
    // Create geometry
    let geometry = RectGeometry::new(
        Vector2::new(0.0, 0.0),
        width,
        height,
        Vector2::new(1.0, 1.0)
    );
    
    // Create simulation
    let mut sim = Simulation::new(&geometry);
    
    // Populate with random values
    let mut rng = rand::rng();
    for i in 0..geometry.size() {
        if let Some(state) = sim.get_mut(i) {
            state.val = if rng.random::<f32>() < 0.3 { 1 } else { 0 };
        }
    }
    
    println!("Conway's Game of Life Demo - Running for 100 steps");
    println!("Initial state:");
    print_grid(&sim, width, height);
    
    // Run simulation for 100 steps
    for step in 1..=100 {
        thread::sleep(Duration::from_millis(200));
        sim.step();

        print_grid(&sim, width, height);
        println!("Step {}/100:", step);
    }
    
    println!("Simulation complete!");
}
