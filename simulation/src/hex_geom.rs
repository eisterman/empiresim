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

#[derive(Clone, Copy, Debug, Hash)]
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
    minmax: MinMaxAxial,
}

impl HexGeometry {
    pub fn new(origin: Vector2, xhexs: usize, yhexs: usize, size: f32) -> Self {
        // Preload minmax and cache it
        let lower_left = Self::offset_to_axial(OffsetCoord{x: 0, y: yhexs as isize -1});
        let lower_right = Self::offset_to_axial(OffsetCoord{x: xhexs as isize - 1, y: yhexs as isize -1});
        let minmax = MinMaxAxial{ min_q: lower_left.q, max_q: lower_right.q, min_r: 0, max_r: lower_right.r};
        Self{origin, xhexs, yhexs, size, minmax}
    }

    // pub fn seq_to_offset(&self, id: SeqID) -> OffsetCoord {
    //     // Center at the middle so we move the coords in negative by half the nx and ny
    //     OffsetCoord{ x: (id.0 % self.xhexs) as isize - self.xhexs as isize/2, y: (id.0 / self.yhexs) as isize - self.yhexs as isize/2 }
    // }

    pub fn offset_to_axial(o: OffsetCoord) -> AxialCoord {
        // how to identify the cube coordinates center? nx = xhexs//2 ; ny = yhexs//2
        // NOTE: this algorithm uses the offset center as center
        let parity = o.y & 1; // This gives 0 and 1 even for negative numbers!
        let q = o.x - (o.y - parity) / 2;
        let r = o.x;
        AxialCoord{q, r}
    }

    pub fn distance(&self, a: AxialCoord, b: AxialCoord) -> f32 {
        let vec = AxialCoord{ q: a.q - b.q, r: a.r - b.r };
        (vec.q.abs() + (vec.q + vec.r).abs() + vec.r.abs()) as f32 / 2.0
    }

    pub fn neighbours(&self, a: AxialCoord) -> Vec<AxialCoord> {
        // Directions 0 = DX and then increasing going in anticlockwise
        const HEXDIRS: [(isize, isize); 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];
        HEXDIRS.into_iter().map(|(q,r)| AxialCoord{q: q+a.q, r: r+a.r}).filter(|a|{
            self.minmax.min_q <= a.q && a.q < self.minmax.max_q &&
                self.minmax.min_r <= a.r && a.r < self.minmax.max_r
        }).collect()
    }

    pub fn minmax_axial(&self) -> MinMaxAxial {
        self.minmax.clone()
    }

    // W = sqrt(3)*size and
    pub fn hex_width(&self) -> f32 {
        f32::sqrt(3.0)*self.size
    }

    // H = 2*size
    pub fn hex_height(&self) -> f32 {
        2.0*self.size
    }

    pub fn rect(&self) -> Rectangle {
        let w = self.hex_width();
        let h = self.hex_height();
        Rectangle{
            x: self.origin.x,
            y: self.origin.y,
            width: if self.yhexs >= 2 { 0.5*w } else { 0.0 } + (self.xhexs as f32)*w,
            height: h + if self.yhexs >= 2 { 0.75*h*(self.yhexs-1) as f32 } else { 0.0 }
        }
    }
}

#[derive(Clone)]
pub struct MinMaxAxial {
    pub min_q: isize,
    pub max_q: isize,
    pub min_r: isize, // = 0
    pub max_r: isize,
}