use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Vector2};
use crate::hex_geom::{AxialCoord, HexGeometry, MinMaxAxial};

pub struct HexConwaySimulation<'a> {
    geo: &'a HexGeometry,
    pub states: Vec<Vec<u8>>,  // [q(traslato di -min_q)][r]
    birth: Vec<u8>,
    stay: Vec<u8>,
    minmax: MinMaxAxial,
}

impl<'a> HexConwaySimulation<'a> {
    pub fn new(geo: &'a HexGeometry, birth: &[u8], stay: &[u8]) -> Self {
        let minmax = geo.minmax_axial();
        let states = vec![vec![0; (minmax.max_r - minmax.min_r) as usize]; (minmax.max_q - minmax.min_q) as usize];
        HexConwaySimulation{
            geo,
            states,
            birth: birth.to_vec(),
            stay: stay.to_vec(),
            minmax
        }
    }

    pub fn get_axial<'b>(states: &'b Vec<Vec<u8>>, a: &AxialCoord, minmax: &MinMaxAxial) -> &'b u8 {
        states.get((a.q-minmax.min_q) as usize).unwrap().get(a.r as usize).unwrap()
    }

    pub fn axial_to_statecoords(&self, q: isize, r: isize) -> (usize, usize) {
        // TODO: do I still need this?
        let x = q - self.minmax.min_q;
        let y = r;
        (x as usize, y as usize)
    }

    pub fn step(&mut self) {
        // TODO: HOW THE FUCK DO I ITERATE ON AN HEXAGONAL GRID, FIGLIODELLAMMERDA
        let prev_state = self.states.clone();
        // Coordinate valide da min_q a max_q e da min_r a max_r!
        for (offq, qline) in self.states.iter_mut().enumerate() {
            let q = offq as isize + self.minmax.min_q;
            for (r, state) in qline.iter_mut().enumerate() {
                let r = r as isize;
                let neighbours = self.geo.neighbours(AxialCoord{q, r});
                let alives = neighbours.iter().fold(0_u8, |acc, axcord| {
                    if Self::get_axial(&prev_state, axcord, &self.minmax).clone() > 0 { acc+1 } else { acc }
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
        // TODO: not using all the parameters!
        // W = sqrt(3)*size and H = 2*size
        let w = f32::sqrt(3.0)*self.geo.size;
        let h = 2.0*self.geo.size;
        for (offq, qline) in self.states.iter().enumerate() {
            // let q = offq as isize + self.minmax.min_q;
            // offq from 0 to max_q - min_q, so analogue to ny
            let row_y_center = 0.5*h + (0.75*h)*offq as f32;
            let row_offset = if offq % 2 == 0 { 0.5*w } else { w };
            for (r, state) in qline.iter().enumerate() {
                let r = r as isize;
                // r from 0 (min_r) to max_r, so analogue to nx
                let col_x_center = row_offset + r as f32*w;
                let color = if *state > 0 { Color::WHITE } else { Color::BLACK };
                d.draw_poly(Vector2{x: col_x_center, y: row_y_center}, 6, self.geo.size, 90.0, color);
                d.draw_poly_lines(Vector2{x: col_x_center, y: row_y_center}, 6, self.geo.size, 90.0, Color::GRAY)
            }
        }
        let rect = self.geo.rect();
        d.draw_rectangle_lines_ex(rect, 1.0, Color::GRAY)
    }
}
