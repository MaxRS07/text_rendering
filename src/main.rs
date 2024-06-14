mod buffer;
mod mutils;
mod ui_element;
mod window;

use std::{fs::File, io::Read};
use ttf_parser::*;

use buffer::*;
use mutils::*;
use ui_element::TextBox;
use window::*;
fn main() {
    let path = "src/resources/Arial.ttf".to_string();
    let mut font_file = File::open(path).unwrap();
    let mut font_data = Vec::new();
    font_file.read_to_end(&mut font_data).unwrap();
    let face = Face::parse(&font_data, 0).unwrap();

    let width: usize = 900;
    let height: usize = 900;
    let mut view = View::new(width, height);
    let pos = vek::Vec2::new(0u32, 0u32);
    let mut textbox = TextBox::new(pos, width, height, "Hello".to_string(), face);
    view.add_element(&mut textbox);
    run_window(width, height, &mut view);
}
