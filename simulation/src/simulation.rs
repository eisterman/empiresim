pub trait Simulation {
    type State;
    type Geometry;
    fn get_geometry(&self) -> &Self::Geometry;
    fn get_states(&self) -> &Vec<Self::State>;
}