pub trait Geometry {
    type ID: Clone;
    fn size(&self) -> usize;
    fn distance(&self, id1: Self::ID, id2: Self::ID) -> f32;
    fn neighbours(&self, id: Self::ID) -> Vec<Self::ID>;
}