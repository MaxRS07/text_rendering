use crate::mutils::*;
use crate::ui_element::*;
use core::fmt;
use std::clone;
use std::ops::Deref;
use std::{fs::File, io::Read};
use ttf_parser::*;
use vek::{vec2, Vec2};

#[derive(Clone)]
pub struct View<'a> {
    pub width: usize,
    pub height: usize,
    pub elements: Vec<&'a mut dyn UIElement>,
    pub buffer: Vec<u32>,
}

impl<'a> View<'a> {
    pub fn new(width: usize, height: usize) -> Self {
        View {
            width,
            height,
            elements: vec![],
            buffer: vec![0; width * height],
        }
    }
    pub fn add_element(&mut self, element: &'a mut dyn UIElement) {
        self.elements.push(element);
    }
    pub fn draw(&mut self) {
        let elements = self.clone().elements;
        for element in elements {
            element.draw_self(self);
        }
    }
    pub fn draw_line(&mut self, line_start: Vec2<f32>, line_end: Vec2<f32>) {
        let dist = line_start.distance(line_end);
        let dir = (line_end - line_start).normalized();
        let mut pen = line_start;
        for _ in 0..dist as u32 {
            pen += dir;
            *self
                .buffer
                .pos_from_xy(pen.x as u32, pen.y as u32, self.width) = enccol(255, 255, 255);
        }
    }
    fn draw_panel(&mut self, rect: vek::Aabr<u32>) {
        let x = rect.min.x;
        let y = rect.min.y;
        let x1 = rect.max.x;
    }
}
impl Clone for dyn UIElement {}
