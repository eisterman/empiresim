use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Vector2};
use crate::hex_geom::{HexGeometry, OffsetCoord};

// TODO: Due intervalli di max min su q e r e basta NON SONO SUFFICIENTI a determinare propriamente
//  la zona da tenere viva. C'e bisogno di un modo diverso di delimitare una zona valida.
//  Per fixare il bug dell'overdraw/oversimulating basta sostituire MinMax con un nuovo metodo
//  per sapere lo spazio delle coordinate assiali che sono valide.
//  C'e forse il rischio di dover riscrivere tutto con coordinate Offset ma speriamo di no...

pub struct HexConwaySimulation<'a> {
    geo: &'a HexGeometry,
    pub states: Vec<Vec<u8>>, // [y:0-rows][x:0-cols]. External is the row, internal is col/cell
    birth: Vec<u8>,
    stay: Vec<u8>,
}

impl<'a> HexConwaySimulation<'a> {
    pub fn new(geo: &'a HexGeometry, birth: &[u8], stay: &[u8]) -> Self {
        let states = vec![vec![0; geo.cols]; geo.rows];
        HexConwaySimulation{
            geo,
            states,
            birth: birth.to_vec(),
            stay: stay.to_vec(),
        }
    }

    pub fn step(&mut self) {
        let prev_state = self.states.clone();
        for (y, row) in self.states.iter_mut().enumerate() {
            for (x, state) in row.iter_mut().enumerate() {
                let neighbours = self.geo.neighbours(OffsetCoord{x: x as isize,y: y as isize}.axial());
                let alives = neighbours.iter().fold(0_u8, |acc, axcord| {
                    let o = axcord.offset();
                    if prev_state[o.y as usize][o.x as usize] > 0 { acc+1 } else { acc }
                });
                *state = if *state > 0 {
                    if self.stay.contains(&alives) { 1 } else { 0 }
                } else {
                    if self.birth.contains(&alives) { 1 } else { 0 }
                }
            }
        }
    }

    pub fn draw(&mut self, d: &mut RaylibDrawHandle) {
        let w = self.geo.hex_width();
        let h = self.geo.hex_height();
        for (y, row) in self.states.iter().enumerate() {
            let row_y_center = 0.5*h + (0.75*h)*y as f32;
            let row_offset = if y % 2 == 0 { 0.5*w } else { w };
            for (x, state) in row.iter().enumerate() {
                let col_x_center = row_offset + x as f32*w;
                let color = if *state > 0 { Color::WHITE } else { Color::BLACK };
                d.draw_poly(Vector2{x: col_x_center, y: row_y_center}, 6, self.geo.size, 90.0, color);
                d.draw_poly_lines(Vector2{x: col_x_center, y: row_y_center}, 6, self.geo.size, 90.0, Color::GRAY)
            }
        }
        let mut rect = self.geo.rect();
        let line_thick: f32 = 10.0;
        rect.x -= line_thick;
        rect.width += 2.0*line_thick;
        rect.y -= line_thick;
        rect.height += 2.0*line_thick;
        d.draw_rectangle_lines_ex(rect, line_thick, Color::GRAY.alpha(0.5))
    }
}
