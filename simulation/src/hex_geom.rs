/*
https://www.redblobgames.com/grids/hexagons/
pointy-top orientation
“odd-r” horizontal layout
Every cell identified by Axial coordinates.
The map will be inscripted into a rectangle.
The characteristic size measure of the hex is the "Size" (center to pointy).
In Pointy-top orient we have W = sqrt(3)*size and H = 2*size

We use Offset coordinates in “odd-r” horizontal layout for drawing the cells
*/
use raylib::prelude::*;
use crate::geometry::Geometry;

// TODO: Notiamo come qui dobbiamo esporre come geometria il SeqID che viene utilizzato per accedere
//  alla memoria... ma noi idealmente dobbiamo offrire dalla Geometria un sistema di coordinate utile
//  alla simulazione. Il problema e' che non posso offrire contemporaneamente un sistema di coordinate
//  non lineare e un modo lineare proprio di usarlo per accedere alla memoria.
//  C'e da ricontrollare il design dell'interfaccia generica... again.

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SeqID(pub usize);

pub struct OffsetCoord {
    pub x: isize,
    pub y: isize,
}

// CubeCoord is a Vector3 with x = q, y = r, z = s

pub struct AxialCoord {
    pub q: isize,
    pub r: isize
}

impl AxialCoord {
    pub fn s(&self) -> isize {
        -self.q-self.r
    }

    pub fn cube(&self) -> Vector3 {
        Vector3::new(self.q as f32, self.r as f32, self.s() as f32)
    }
}

pub struct HexGeometry {
    pub origin: Vector2,
    pub xhexs: usize,
    pub yhexs: usize,
    pub size: f32,
}

impl HexGeometry {
    pub fn new(origin: Vector2, xhexs: usize, yhexs: usize, size: f32) -> Self {
        Self{origin, xhexs, yhexs, size}
    }

    pub fn seq_to_offset(&self, id: SeqID) -> OffsetCoord {
        // Center at the middle so we move the coords in negative by half the nx and ny
        OffsetCoord{ x: (id.0 % self.xhexs) as isize - self.xhexs as isize/2, y: (id.0 / self.yhexs) as isize - self.yhexs as isize/2 }
    }

    pub fn seq_to_axial(&self, id: SeqID) -> AxialCoord {
        // how to identify the cube coordinates center? nx = xhexs//2 ; ny = yhexs//2
        // TODO: how to convert from offset coord to cube !?
        let o = self.seq_to_offset(id);
        // NOTE: this algorithm uses the offset center as center
        let parity = o.y & 1; // This gives 0 and 1 even for negative numbers!
        let q = o.x - (o.y - parity) / 2;
        let r = o.x;
        AxialCoord{q, r}
    }
}

impl Geometry for HexGeometry {
    type ID = SeqID;

    fn size(&self) -> usize {
        self.xhexs*self.yhexs
    }

    fn distance(&self, id1: Self::ID, id2: Self::ID) -> f32 {
        todo!()
    }

    fn neighbours(&self, id: Self::ID) -> Vec<Self::ID> {
        todo!()
    }
}