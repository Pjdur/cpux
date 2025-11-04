use crate::widget::Widget;
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::any::Any;

pub struct Text {
    pub content: String,
    pub position: (u32, u32),
    pub color: Rgba<u8>,
    pub scale: Scale,
    pub font: Font<'static>,
}

impl Widget for Text {
    fn draw(&self, image: &mut RgbaImage) {
        draw_text_mut(
            image,
            self.color,
            self.position.0 as i32,
            self.position.1 as i32,
            self.scale,
            &self.font,
            &self.content,
        );
    }
    fn as_any_mut(&mut self) -> &mut dyn Any { self }
}
