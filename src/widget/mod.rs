pub mod text;
pub mod button;

use image::RgbaImage;
use std::any::Any;

pub trait Widget {
    fn draw(&mut self, image: &mut RgbaImage);
    fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub trait Positionable {
    fn set_position(&mut self, x: i32, y: i32);
}

pub trait WidgetPositionable: Widget + Positionable + Any {}
impl<T: Widget + Positionable + Any> WidgetPositionable for T {}
