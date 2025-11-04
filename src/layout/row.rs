use crate::widget::{Positionable, Widget};
use image::RgbaImage;

pub struct Row {
    pub children: Vec<Box<dyn Widget>>,
    pub spacing: i32,
    pub start_x: i32,
    pub start_y: i32,
}

impl Widget for Row {
    fn draw(&mut self, image: &mut RgbaImage) {
        let mut x = self.start_x;
        for child in &mut self.children {
            if let Some(text) = child.as_any_mut().downcast_mut::<crate::widget::text::Text>() {
                text.set_position(x, self.start_y);
            } else if let Some(button) = child.as_any_mut().downcast_mut::<crate::widget::button::Button>() {
                button.set_position(x, self.start_y);
            }
            child.draw(image);
            x += self.spacing;
        }
    }

    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self
    }
}
