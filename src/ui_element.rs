use crate::buffer::*;
use crate::mutils::*;
use std::any::{Any, TypeId};
use ttf_parser::*;
use vek::*;

pub trait UIElement: Any + Clone {
    fn draw_self(&self, buffer: &mut View);
    fn get_text(&mut self) -> &mut String;
}

#[derive(Clone)]
pub struct TextBox<'a> {
    pub position: Vec2<u32>,
    pub width: usize,
    pub height: usize,
    pub text: String,
    pub face: Face<'a>,
}
impl<'a> TextBox<'a> {
    pub fn new(
        position: Vec2<u32>,
        width: usize,
        height: usize,
        text: String,
        face: Face<'a>,
    ) -> Self {
        return TextBox {
            position,
            width,
            height,
            text,
            face,
        };
    }
    pub fn set_text(&mut self, text: String) {
        self.text = text;
    }
}
impl UIElement for TextBox<'_> {
    fn get_text(&mut self) -> &mut String {
        return &mut self.text;
    }
    fn draw_self(&self, buffer: &mut View) {
        let mut start_pos = Vec2::new(100f32, 400f32);
        let mut draw_off_x = 0f32;
        let ppi = 224f32;
        let font_size = 10f32;
        let line_spacing = 50;
        let line_height = self.face.ascender() - self.face.descender() + line_spacing;
        let half_line_height = line_height as f32 / 2f32;
        for char_to_draw in self.text.chars() {
            let glyph_id = self.face.glyph_index(char_to_draw).unwrap();
            let mut builder = Builder::new();
            let bbox: Option<ttf_parser::Rect> = self.face.outline_glyph(glyph_id, &mut builder);

            let px_size = self.face.glyph_hor_advance(glyph_id).unwrap() as f32 * font_size * ppi
                / (72f32 * self.face.units_per_em() as f32);

            if bbox == None {
                draw_off_x += self.face.glyph_hor_advance(glyph_id).unwrap() as f32 / px_size;
                continue;
            }
            let off = Vec2::new(start_pos.x + draw_off_x, start_pos.y);
            let mut pen: Vec2<f32> = Vec2::new(0f32, 0f32);
            for i in 0..builder.points.len() {
                let point = &builder.points[i];
                let next = Vec2::new(
                    point.position[0] / px_size + off.x,
                    -point.position[1] / px_size + off.y,
                );
                match point.point_type {
                    PointType::Move => {
                        pen = next;
                    }
                    PointType::Line => {
                        buffer.draw_line(pen, next);
                        pen = next;
                    }
                    PointType::Quad => {
                        let vec1 = Vec2::new(
                            point.position[0] / px_size + off.x,
                            -point.position[1] / px_size + off.y,
                        );
                        let vec = Vec2::new(
                            point.position[2] / px_size + off.x,
                            -point.position[3] / px_size + off.y,
                        );

                        buffer.draw_line(pen, vec);
                        pen = vec;
                    }
                    PointType::Curve => {
                        let vec2 = Vec2::new(
                            point.position[0] / px_size + off.x,
                            -point.position[1] / px_size + off.y,
                        );
                        let vec1 = Vec2::new(
                            point.position[2] / px_size + off.x,
                            -point.position[3] / px_size + off.y,
                        );
                        let vec = Vec2::new(
                            point.position[4] / px_size + off.x,
                            -point.position[5] / px_size + off.y,
                        );

                        buffer.draw_line(pen, vec);
                        pen = vec;
                        buffer.draw_line(pen, vec1);
                        pen = vec1;
                        buffer.draw_line(pen, vec2);
                        pen = vec2
                    }
                }
            }
            draw_off_x += self.face.glyph_hor_advance(glyph_id).unwrap() as f32 / px_size;
        }
        buffer.draw_line(
            start_pos + draw_off_x * Vec2::unit_x() - half_line_height * Vec2::unit_y(),
            start_pos + draw_off_x * Vec2::unit_x() + half_line_height * Vec2::unit_y(),
        );
    }
}
