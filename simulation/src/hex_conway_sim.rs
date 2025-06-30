use crate::geometry::Geometry;
use crate::simulation::Simulation;

#[derive(Clone)]
pub struct State {
    pub val: u8
}

pub struct ConwaySimulation<'a, T> where T: Geometry {
    geo: &'a T,
    states: Vec<crate::conway_sim::State>,
    birth: Vec<u8>,
    stay: Vec<u8>,
}