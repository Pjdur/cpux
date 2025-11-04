use crate::widget::{Positionable, Widget};
use image::RgbaImage;

pub struct Column {
    pub children: Vec<Box<dyn Widget>>,
    pub spacing: i32,
    pub start_x: i32,
    pub start_y: i32,
}

impl Widget for Column {
    fn draw(&mut self, image: &mut RgbaImage) {
        let mut y = self.start_y;
        for child in &mut self.children {
            if let Some(text) = child.as_any_mut().downcast_mut::<crate::widget::text::Text>() {
                text.set_position(self.start_x, y);
            } else if let Some(button) = child.as_any_mut().downcast_mut::<crate::widget::button::Button>() {
                button.set_position(self.start_x, y);
            }
            child.draw(image);
            y += self.spacing;
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
