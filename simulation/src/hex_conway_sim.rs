use raylib::color::Color;
use raylib::drawing::{RaylibDraw, RaylibDrawHandle};
use raylib::math::{Vector2};
use crate::hex_geom::{AxialCoord, HexGeometry, MinMaxAxial};

// TODO: Due intervalli di max min su q e r e basta NON SONO SUFFICIENTI a determinare propriamente
//  la zona da tenere viva. C'e bisogno di un modo diverso di delimitare una zona valida.
//  Per fixare il bug dell'overdraw/oversimulating basta sostituire MinMax con un nuovo metodo
//  per sapere lo spazio delle coordinate assiali che sono valide.
//  C'e forse il rischio di dover riscrivere tutto con coordinate Offset ma speriamo di no...

pub struct HexConwaySimulation<'a> {
    geo: &'a HexGeometry,
    pub states: Vec<Vec<u8>>, // [r][q(translato di -min_q)]. There is a little more data than needed
    birth: Vec<u8>,
    stay: Vec<u8>,
    minmax: MinMaxAxial,
}

impl<'a> HexConwaySimulation<'a> {
    pub fn new(geo: &'a HexGeometry, birth: &[u8], stay: &[u8]) -> Self {
        let minmax = geo.minmax_axial();
        assert_eq!(minmax.min_r, 0, "This simulation expect r to start from 0.");
        let states = vec![vec![0; (minmax.max_q - minmax.min_q) as usize]; (minmax.max_r - minmax.min_r) as usize];
        HexConwaySimulation{
            geo,
            states,
            birth: birth.to_vec(),
            stay: stay.to_vec(),
            minmax
        }
    }

    pub fn get_axial<'b>(states: &'b Vec<Vec<u8>>, a: &AxialCoord, minmax: &MinMaxAxial) -> &'b u8 {
        states.get(a.r as usize).unwrap().get((a.q-minmax.min_q) as usize).unwrap()
    }

    pub fn step(&mut self) {
        // TODO: HOW THE FUCK DO I ITERATE ON AN HEXAGONAL GRID, FIGLIODELLAMMERDA
        let prev_state = self.states.clone();
        // Coordinate valide da min_q a max_q e da min_r a max_r!
        for (r, rline) in self.states.iter_mut().enumerate() {
            let r = r as isize;
            for (offq, state) in rline.iter_mut().enumerate() {
                let q = offq as isize + self.minmax.min_q;
                if !self.minmax.validate(AxialCoord{q,r}) {continue}
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
        let w = self.geo.hex_width();
        let h = self.geo.hex_height();
        for (r, rline) in self.states.iter().enumerate() {
            let row_y_center = 0.5*h + (0.75*h)*r as f32;
            let row_offset = if r % 2 == 0 { 0.5*w } else { w };
            for (offq, state) in rline.iter().enumerate() {
                let q = offq as isize + self.minmax.min_q;
                println!("q {} r {}", q, r);
                if !self.minmax.validate(AxialCoord{q,r: r as isize}) {continue}
                // We keep offq because is from 0 to cols
                let col_x_center = row_offset + offq as f32*w;
                let color = if *state > 0 { Color::WHITE } else { Color::BLACK };
                d.draw_poly(Vector2{x: col_x_center, y: row_y_center}, 6, self.geo.size, 90.0, color);
                d.draw_poly_lines(Vector2{x: col_x_center, y: row_y_center}, 6, self.geo.size, 90.0, Color::GRAY)
            }
        }
        let rect = self.geo.rect();
        d.draw_rectangle_lines_ex(rect, 10.0, Color::GRAY)
    }
}
