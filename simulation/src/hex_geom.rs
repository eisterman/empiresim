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

impl OffsetCoord {
    pub fn axial(&self) -> AxialCoord {
        // how to identify the cube coordinates center? nx = xhexs//2 ; ny = yhexs//2
        // NOTE: this algorithm uses the offset center as center
        let parity = self.y & 1; // This gives 0 and 1 even for negative numbers!
        let q = self.x - (self.y - parity) / 2;
        let r = self.y;
        AxialCoord{q, r}
    }
}

// CubeCoord is a Vector3 with x = q, y = r, z = s

#[derive(Debug)]
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

    pub fn offset(&self) -> OffsetCoord {
        // NOTE: this algorithm uses the offset center as center
        let parity = self.r & 1;
        let x = self.q + (self.r - parity) / 2;
        let y = self.r;
        OffsetCoord{ x, y}
    }
}

pub struct HexGeometry {
    pub origin: Vector2,
    pub cols: usize,
    pub rows: usize,
    pub size: f32,
}

impl HexGeometry {
    pub fn new(origin: Vector2, cols: usize, rows: usize, size: f32) -> Self {
        Self{origin, cols, rows, size}
    }

    // pub fn seq_to_offset(&self, id: SeqID) -> OffsetCoord {
    //     // Center at the middle so we move the coords in negative by half the nx and ny
    //     OffsetCoord{ x: (id.0 % self.xhexs) as isize - self.xhexs as isize/2, y: (id.0 / self.yhexs) as isize - self.yhexs as isize/2 }
    // }

    pub fn distance(&self, a: AxialCoord, b: AxialCoord) -> f32 {
        let vec = AxialCoord{ q: a.q - b.q, r: a.r - b.r };
        (vec.q.abs() + (vec.q + vec.r).abs() + vec.r.abs()) as f32 / 2.0
    }

    pub fn neighbours(&self, a: AxialCoord) -> Vec<AxialCoord> {
        // Directions 0 = DX and then increasing going in anticlockwise
        const HEXDIRS: [(isize, isize); 6] = [(1, 0), (1, -1), (0, -1), (-1, 0), (-1, 1), (0, 1)];
        HEXDIRS.into_iter().map(|(q,r)| AxialCoord{q: q+a.q, r: r+a.r}).filter(|a|{
            let o = a.offset();
            0 <= o.x && o.x < self.cols as isize &&
                0 <= o.y && o.y < self.rows as isize
        }).collect()
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
        let height = h + if self.rows > 1 { (self.rows -1) as f32*0.75*h } else { 0.0 };
        let width = (self.cols as f32)*w + if self.rows > 1 { 0.5*w } else { 0.0 };
        Rectangle{
            x: self.origin.x,
            y: self.origin.y,
            height,
            width,
            // height: if self.rows >= 2 { 0.5*w } else { 0.0 } + (self.cols as f32)*w,
            // width: h + if self.rows >= 2 { 0.75*h*(self.rows -1) as f32 } else { 0.0 },
        }
    }
}
