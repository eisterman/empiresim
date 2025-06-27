
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GeoID(pub usize);

pub trait Geometry {
    fn size(&self) -> usize;
    fn distance(&self, id1: GeoID, id2: GeoID) -> f32;
    fn neighbours(&self, id: GeoID) -> Vec<GeoID>;
}