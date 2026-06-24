use crate::widget::Widget;
use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_filled_rect_mut, draw_text_mut};
use imageproc::rect::Rect;
use rusttype::{Font, Scale};

pub struct Button {
    pub label: String,
    pub rect: Rect,
    pub font: Font<'static>,
    pub scale: Scale,
    pub text_color: Rgba<u8>,
    pub bg_color: Rgba<u8>,
    pub on_click: Option<Box<dyn FnMut()>>,
}

impl Widget for Button {
    fn set_position(&mut self, x: i32, y: i32) {
        self.rect = Rect::at(x, y).of_size(self.rect.width(), self.rect.height());
    }

    fn draw(&mut self, image: &mut RgbaImage) {
        draw_filled_rect_mut(image, self.rect, self.bg_color);
        draw_text_mut(
            image,
            self.text_color,
            self.rect.left() + 10,
            self.rect.top() + 10,
            self.scale,
            &self.font,
            &self.label,
        );
    }

    fn handle_click(&mut self, x: i32, y: i32) {
        let r = self.rect;
        if x >= r.left() && x <= r.right() && y >= r.top() && y <= r.bottom() {
            if let Some(callback) = &mut self.on_click {
                callback();
            }
        }
    }

    fn size(&self) -> (i32, i32) {
        (self.rect.width() as i32, self.rect.height() as i32)
    }
}
