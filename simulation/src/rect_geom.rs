use raylib::prelude::*;
use crate::geometry::*;

// TODO: Forse usare un vettore proprio castabile a raylib e' meglio per rendere la lib piu indip

pub struct InnerNCells {pub x: usize, pub y: usize}
impl InnerNCells {
    pub fn vec2(&self) -> Vector2 {
        Vector2::new(self.x as f32, self.y as f32)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RectGeoID(pub usize);

pub struct RectGeometry {
    pub geocenter: Vector2,
    pub cells: InnerNCells,
    pub celsize: Vector2,
}

impl RectGeometry {
    pub fn new(geocenter: Vector2, xcells: usize, ycells: usize, celsize: Vector2) -> Self {
        Self { geocenter, cells: InnerNCells{x: xcells, y: ycells}, celsize }
    }

    pub fn start(&self) -> Vector2 {
        self.geocenter - self.cells.vec2() * self.celsize / 2.0
    }

    pub fn cellcenter(&self, nx: usize, ny: usize) -> Vector2 {
        let n = Vector2::new(nx as f32, ny as f32);
        self.start() + (n + 0.5) * self.celsize
    }

    pub fn cell2id(&self, nx: usize, ny: usize) -> RectGeoID {
        RectGeoID(ny * self.cells.x + nx)
    }

    pub fn id2cell(&self, id: RectGeoID) -> (usize, usize) {
        (id.0 % self.cells.x, id.0 / self.cells.x)
    }

    pub fn id2centercoord(&self, id: RectGeoID) -> Vector2 {
        let (nx, ny) = self.id2cell(id);
        self.cellcenter(nx, ny)
    }

    // For Drawable Impl
    pub fn cell_rectangle(&self, nx: i32, ny: i32) -> Rectangle {
        let start = self.start();
        Rectangle {
            x: start.x + (nx as f32) * self.celsize.x,
            y: start.y + (ny as f32) * self.celsize.y,
            width: self.celsize.x,
            height: self.celsize.y,
        }
    }
}

impl Geometry for RectGeometry {
    type ID = RectGeoID;
    
    fn size(&self) -> usize {
        self.cells.x * self.cells.y
    }

    fn distance(&self, id1: Self::ID, id2: Self::ID) -> f32 {
        self.id2centercoord(id1).distance_to(self.id2centercoord(id2))
    }

    fn neighbours(&self, id: Self::ID) -> Vec<Self::ID> {
        let (x, y) = self.id2cell(id);
        let mut result: Vec<RectGeoID> = Vec::with_capacity(8);
        // All 8 directions
        for dx in -1..=1_i32 {
            for dy in -1..=1_i32 {
                if dx == 0 && dy == 0 {
                    continue; // Skip the center cell
                }
                let nx = x as i32 + dx;
                let ny = y as i32 + dy;
                if nx >= 0 && nx < self.cells.x as i32 && ny >= 0 && ny < self.cells.y as i32 {
                    result.push(self.cell2id(nx as usize, ny as usize));
                }
            }
        }
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> RectGeometry {
        RectGeometry {
            geocenter: Vector2::new(100.0, 200.0),
            cells: InnerNCells { x: 5, y: 8 },
            celsize: Vector2::new(10.0, 5.0),
        }
    }

    #[test]
    fn test_size() {
        let geom = setup();
        assert_eq!(geom.size(), 40);
    }

    #[test]
    fn test_cell2id_and_id2cell() {
        let geom = setup();
        let id = geom.cell2id(3, 4);
        assert_eq!(id, RectGeoID(23));
        let (x, y) = geom.id2cell(id);
        assert_eq!(x, 3);
        assert_eq!(y, 4);
    }

    #[test]
    fn test_id2centercoord() {
        let geom = setup();
        let id = geom.cell2id(0, 0);
        let center = geom.id2centercoord(id);
        assert_eq!(center, Vector2::new(80.0, 182.5));
    }

    #[test]
    fn test_distance() {
        let geom = setup();
        let id1 = geom.cell2id(0, 0);
        let id2 = geom.cell2id(1, 0);
        let distance = geom.distance(id1, id2);
        assert_eq!(distance, 10.0);
    }

    #[test]
    fn test_neighbours() {
        let geom = setup();
        let id = geom.cell2id(2, 3);
        let neighbours = geom.neighbours(id);
        assert_eq!(neighbours.len(), 8);
        // Orthogonal neighbors
        assert!(neighbours.contains(&geom.cell2id(1, 3)));
        assert!(neighbours.contains(&geom.cell2id(3, 3)));
        assert!(neighbours.contains(&geom.cell2id(2, 2)));
        assert!(neighbours.contains(&geom.cell2id(2, 4)));
        // Diagonal neighbors
        assert!(neighbours.contains(&geom.cell2id(1, 2)));
        assert!(neighbours.contains(&geom.cell2id(3, 2)));
        assert!(neighbours.contains(&geom.cell2id(1, 4)));
        assert!(neighbours.contains(&geom.cell2id(3, 4)));
    }

    #[test]
    fn test_neighbours_edge() {
        let geom = setup();
        let id = geom.cell2id(0, 0);
        let neighbours = geom.neighbours(id);
        assert_eq!(neighbours.len(), 3);
        assert!(neighbours.contains(&geom.cell2id(1, 0)));
        assert!(neighbours.contains(&geom.cell2id(0, 1)));
        assert!(neighbours.contains(&geom.cell2id(1, 1)));
    }

    #[test]
    fn test_neighbours_vertex() {
        let geom = setup();
        // Top-left corner (0,0)
        let id = geom.cell2id(0, 0);
        let neighbours = geom.neighbours(id);
        assert_eq!(neighbours.len(), 3);
        assert!(neighbours.contains(&geom.cell2id(1, 0)));
        assert!(neighbours.contains(&geom.cell2id(0, 1)));
        assert!(neighbours.contains(&geom.cell2id(1, 1)));

        // Top-right corner (4,0)
        let id = geom.cell2id(4, 0);
        let neighbours = geom.neighbours(id);
        assert_eq!(neighbours.len(), 3);
        assert!(neighbours.contains(&geom.cell2id(3, 0)));
        assert!(neighbours.contains(&geom.cell2id(4, 1)));
        assert!(neighbours.contains(&geom.cell2id(3, 1)));

        // Bottom-left corner (0,7)
        let id = geom.cell2id(0, 7);
        let neighbours = geom.neighbours(id);
        assert_eq!(neighbours.len(), 3);
        assert!(neighbours.contains(&geom.cell2id(1, 7)));
        assert!(neighbours.contains(&geom.cell2id(0, 6)));
        assert!(neighbours.contains(&geom.cell2id(1, 6)));

        // Bottom-right corner (4,7)
        let id = geom.cell2id(4, 7);
        let neighbours = geom.neighbours(id);
        assert_eq!(neighbours.len(), 3);
        assert!(neighbours.contains(&geom.cell2id(3, 7)));
        assert!(neighbours.contains(&geom.cell2id(4, 6)));
        assert!(neighbours.contains(&geom.cell2id(3, 6)));
    }
}