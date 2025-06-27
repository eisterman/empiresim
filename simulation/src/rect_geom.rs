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
        vec![
            self.cell2id(x-1, y),
            self.cell2id(x+1, y),
            self.cell2id(x, y-1),
            self.cell2id(x, y+1),
        ]
    }
}

impl RLDrawable for RectGeometry {
    fn draw(&self, rl: &mut RaylibHandle) {
        todo!()
    }
}