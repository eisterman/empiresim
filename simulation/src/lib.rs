use crate::geometry::{GeoID, Geometry};

mod geometry;
mod rect_geom;
mod rldrawable;

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
