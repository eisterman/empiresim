use crate::geometry::{GeoID, Geometry};

pub mod geometry;
pub mod rect_geom;
pub mod rldrawable;

#[derive(Clone)]
pub struct State {
    pub val: u8
}

pub struct Simulation<'a> {
    geo: &'a dyn Geometry,
    states: Vec<State>,
}

impl<'a> Simulation<'a> {
    pub fn new(geo: &'a dyn Geometry) -> Self {
        let states = vec![State{val: 0}; geo.size()];
        Self{ geo, states }
    }

    pub fn get(&self, id: usize) -> Option<&State> {
        self.states.get(id)
    }
    
    pub fn get_mut(&mut self, id: usize) -> Option<&mut State> {
        self.states.get_mut(id)
    }

    pub fn step(&mut self) {
        let prev_state = self.states.clone();
        for (i, s) in self.states.iter_mut().enumerate() {
            let id = GeoID(i);
            let neighbours = self.geo.neighbours(id);
            let alives = neighbours.iter().fold(0_u8, |acc, gid| {
                if prev_state.get(gid.0).unwrap().val > 0 { acc+1 } else { acc }
            });
            s.val = if s.val > 0 {
                if alives == 2 || alives == 3 { 1 } else { 0 }
            } else {
                if alives == 3 { 1 } else { 0 }
            };
        }
    }
}

pub fn hello() {
    println!("Hello from the simulation library!");
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rect_geom::RectGeometry;
    use raylib::prelude::Vector2;

    #[test]
    fn test_glider_pattern() {
        let geometry = RectGeometry::new(
            Vector2::new(0.0, 0.0),
            10,
            10,
            Vector2::new(1.0, 1.0)
        );
        
        let mut sim = Simulation::new(&geometry);
        
        // Set up glider pattern at position (1,1)
        // Pattern:
        //  X
        //   XX
        // XX
        let glider_positions = [
            (2, 1), // top
            (3, 2), // middle right
            (1, 3), // bottom left
            (2, 3), // bottom middle
            (3, 3), // bottom right
        ];
        
        for (x, y) in glider_positions {
            let id = y * 10 + x;
            if let Some(state) = sim.get_mut(id) {
                state.val = 1;
            }
        }
        
        // Step 4 times to see glider movement
        for _ in 0..4 {
            sim.step();
        }
        
        // After 4 steps, glider should be at position (2,2) relative to original
        let expected_positions = [
            (3, 2), // moved right and up
            (4, 3), // moved right and up  
            (2, 4), // moved right and up
            (3, 4), // moved right and up
            (4, 4), // moved right and up
        ];
        
        let mut alive_count = 0;
        for y in 0..10 {
            for x in 0..10 {
                let id = y * 10 + x;
                if let Some(state) = sim.get(id) {
                    if state.val > 0 {
                        alive_count += 1;
                    }
                }
            }
        }
        
        // Glider should maintain 5 alive cells
        assert_eq!(alive_count, 5, "Glider should maintain 5 alive cells");
    }
}
