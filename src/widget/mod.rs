pub mod text;
pub mod button;

use image::RgbaImage;
use std::any::Any;

pub trait Widget {
    fn draw(&self, image: &mut RgbaImage);
    fn as_any_mut(&mut self) -> &mut dyn Any;
}
