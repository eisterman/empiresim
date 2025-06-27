use raylib::prelude::*;
use crate::geometry::*;
use crate::rldrawable::RLDrawable;

struct InnerNCells {x: usize, y: usize}
impl InnerNCells {
    fn vec2(&self) -> Vector2 {
        Vector2::new(self.x as f32, self.y as f32)
    }
}

pub struct RectGeometry {
    geocenter: Vector2,
    cells: InnerNCells,
    celsize: Vector2,
}

impl RectGeometry {
    pub fn new(geocenter: Vector2, xcells: usize, ycells: usize, celsize: Vector2) -> Self {
        Self { geocenter, cells: InnerNCells{x: xcells, y: ycells}, celsize }
    }
    
    fn start(&self) -> Vector2 {
        self.geocenter - self.cells.vec2() * self.celsize / 2.0
    }

    fn cellcenter(&self, nx: usize, ny: usize) -> Vector2 {
        let n = Vector2::new(nx as f32, ny as f32);
        self.start() + (n + 0.5) * self.celsize
    }

    fn cell2id(&self, nx: usize, ny: usize) -> GeoID {
        GeoID(ny * self.cells.x + nx)
    }

    fn id2cell(&self, id: GeoID) -> (usize, usize) {
        (id.0 % self.cells.x, id.0 / self.cells.x)
    }

    fn id2centercoord(&self, id: GeoID) -> Vector2 {
        let (nx, ny) = self.id2cell(id);
        self.cellcenter(nx, ny)
    }

}

impl Geometry for RectGeometry {
    fn size(&self) -> usize {
        self.cells.x * self.cells.y
    }

    fn distance(&self, id1: GeoID, id2: GeoID) -> f32 {
        self.id2centercoord(id1).distance_to(self.id2centercoord(id2))
    }

    fn neighbours(&self, id: GeoID) -> Vec<GeoID> {
        let (x, y) = self.id2cell(id);
        let mut result: Vec<GeoID> = Vec::with_capacity(4);
        if x > 0 {
            result.push(self.cell2id(x - 1, y));
        }
        if x < self.cells.x - 1 {
            result.push(self.cell2id(x + 1, y));
        }
        if y > 0 {
            result.push(self.cell2id(x, y - 1));
        }
        if y < self.cells.y - 1 {
            result.push(self.cell2id(x, y + 1));
        }
        result
    }
}

impl RLDrawable for RectGeometry {
    fn draw(&self, rl: &mut RaylibHandle) {
        todo!()
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
        assert_eq!(id, GeoID(23));
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
        assert_eq!(neighbours.len(), 4);
        assert!(neighbours.contains(&geom.cell2id(1, 3)));
        assert!(neighbours.contains(&geom.cell2id(3, 3)));
        assert!(neighbours.contains(&geom.cell2id(2, 2)));
        assert!(neighbours.contains(&geom.cell2id(2, 4)));
    }

    #[test]
    fn test_neighbours_edge() {
        let geom = setup();
        let id = geom.cell2id(0, 0);
        let neighbours = geom.neighbours(id);
        assert_eq!(neighbours.len(), 2);
        assert!(neighbours.contains(&geom.cell2id(1, 0)));
        assert!(neighbours.contains(&geom.cell2id(0, 1)));
    }

    #[test]
    fn test_neighbours_vertex() {
        let geom = setup();
        let id = geom.cell2id(0, 0);
        let neighbours = geom.neighbours(id);
        assert_eq!(neighbours.len(), 2);
        assert!(neighbours.contains(&geom.cell2id(1, 0)));
        assert!(neighbours.contains(&geom.cell2id(0, 1)));

        let id = geom.cell2id(4, 0);
        let neighbours = geom.neighbours(id);
        assert_eq!(neighbours.len(), 2);
        assert!(neighbours.contains(&geom.cell2id(3, 0)));
        assert!(neighbours.contains(&geom.cell2id(4, 1)));

        let id = geom.cell2id(0, 7);
        let neighbours = geom.neighbours(id);
        assert_eq!(neighbours.len(), 2);
        assert!(neighbours.contains(&geom.cell2id(1, 7)));
        assert!(neighbours.contains(&geom.cell2id(0, 6)));

        let id = geom.cell2id(4, 7);
        let neighbours = geom.neighbours(id);
        assert_eq!(neighbours.len(), 2);
        assert!(neighbours.contains(&geom.cell2id(3, 7)));
        assert!(neighbours.contains(&geom.cell2id(4, 6)));
    }
}