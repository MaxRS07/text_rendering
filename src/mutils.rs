use crate::ui_element::*;
use ttf_parser::*;
use vek::*;
pub fn enccol(r: u8, g: u8, b: u8) -> u32 {
    let (r, g, b) = (r as u32, g as u32, b as u32);
    (r << 16) | (g << 8) | b
}
pub trait RectCorners {
    fn tr(&self) -> Vec2<i16>;
    fn tl(&self) -> Vec2<i16>;
    fn br(&self) -> Vec2<i16>;
    fn bl(&self) -> Vec2<i16>;
}
impl RectCorners for ttf_parser::Rect {
    fn tr(&self) -> Vec2<i16> {
        return Vec2::new(self.x_max, self.y_max);
    }
    fn tl(&self) -> Vec2<i16> {
        return Vec2::new(self.x_min, self.y_max);
    }
    fn br(&self) -> Vec2<i16> {
        return Vec2::new(self.x_max, self.y_min);
    }
    fn bl(&self) -> Vec2<i16> {
        return Vec2::new(self.x_min, self.y_min);
    }
}
#[derive(Debug)]
pub enum PointType {
    Move,
    Line,
    Quad,
    Curve,
}
#[derive(Debug)]
pub struct OutlinePoint {
    pub position: Vec<f32>,
    pub point_type: PointType,
}
impl OutlinePoint {
    pub fn new(position: Vec<f32>, point_type: PointType) -> Self {
        OutlinePoint {
            position,
            point_type,
        }
    }
}
pub struct Builder {
    pub points: Vec<OutlinePoint>,
}
impl Builder {
    pub fn new() -> Self {
        return Builder { points: vec![] };
    }
}
impl ttf_parser::OutlineBuilder for Builder {
    fn move_to(&mut self, x: f32, y: f32) {
        self.points
            .push(OutlinePoint::new(vec![x, y], PointType::Move))
    }
    fn line_to(&mut self, x: f32, y: f32) {
        self.points
            .push(OutlinePoint::new(vec![x, y], PointType::Line))
    }
    fn quad_to(&mut self, x1: f32, y1: f32, x: f32, y: f32) {
        self.points
            .push(OutlinePoint::new(vec![x1, y1, x, y], PointType::Quad))
    }
    fn curve_to(&mut self, x1: f32, y1: f32, x2: f32, y2: f32, x: f32, y: f32) {
        self.points.push(OutlinePoint::new(
            vec![x1, y1, x2, y2, x, y],
            PointType::Curve,
        ))
    }
    fn close(&mut self) {
        //write!(&mut self.0, "Z ").unwrap();
    }
}
pub trait Position {
    fn pos_from_xy(&mut self, x: u32, y: u32, width: usize) -> &mut u32;
}
impl Position for Vec<u32> {
    fn pos_from_xy(&mut self, x: u32, y: u32, width: usize) -> &mut u32 {
        let len = self.len() - 1;
        &mut self[(y as usize * width + x as usize).clamp(0, len)]
    }
}
