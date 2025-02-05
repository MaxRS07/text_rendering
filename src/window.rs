use std::{
    any::{Any, TypeId},
    collections::binary_heap::Iter,
    fmt::{Debug, Formatter},
};

use crate::{buffer::*, ui_element::TextBox};
use minifb::{Key, Window, WindowOptions};
use vek::*;

pub fn run_window(width: usize, height: usize, buffer: &mut View) {
    let mut backspace_counter: u32 = 1;
    let mut opts = WindowOptions::default();
    opts.scale = minifb::Scale::X1;
    let mut window = Window::new("Test - ESC to exit", width, height, opts).unwrap_or_else(|e| {
        panic!("{}", e);
    });
    // Limit to max ~60 fps update rate
    window.set_target_fps(16600);
    while window.is_open() && !window.is_key_down(Key::Escape) {
        handle_pressed_keys(&window, buffer, &mut backspace_counter);
        handle_mouse(&window, buffer);
        buffer.draw();

        window
            .update_with_buffer(&buffer.buffer, width, height)
            .unwrap();
    }
}
pub fn handle_pressed_keys(window: &Window, view: &mut View, count: &mut u32) {
    /*let mut text = view.elements[0].get_text();
    window
        .get_keys_pressed(minifb::KeyRepeat::Yes)
        .iter()
        .for_each(|key| match key {
            Key::Backspace => {
                println!("{}", *count);
                for _ in 0..(*count / 30) + 1 {
                    if text.len() > 0 {
                        text.remove(text.len() - 1);
                    }
                }
                *count += 1;
            }
            Key::Space => *text += " ",
            _ => {
                *text += &format!("{:?}", key);
                *count = 1;
            }
        });*/
}
pub fn handle_mouse(window: &Window, buffer: &mut View) {
    if window.get_mouse_down(minifb::MouseButton::Left) {
        let position = window.get_mouse_pos(minifb::MouseMode::Clamp);
        if let Some(value) = position {}
    }
    if window.get_mouse_down(minifb::MouseButton::Right) {
        let position = window.get_mouse_pos(minifb::MouseMode::Clamp);
        if let Some(value) = position {}
    }
}
