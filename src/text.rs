use crate::widget::Widget;
use image::{Rgba, RgbaImage};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

pub struct Text {
    pub content: String,
    pub position: (i32, i32),
    pub color: Rgba<u8>,
    pub scale: Scale,
    pub font: Font<'static>,
}

impl Widget for Text {
    fn set_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }

    fn draw(&mut self, image: &mut RgbaImage) {
        draw_text_mut(
            image,
            self.color,
            self.position.0,
            self.position.1,
            self.scale,
            &self.font,
            &self.content,
        );
    }

    fn size(&self) -> (i32, i32) {
        // Approximate width based on character count and scale width
        let width = self.content.len() as f32 * (self.scale.x * 0.6); 
        let height = self.scale.y;
        (width as i32, height as i32)
    }
}