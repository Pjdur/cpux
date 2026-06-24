use image::RgbaImage;

pub trait Widget {
    fn set_position(&mut self, x: i32, y: i32);
    fn draw(&mut self, image: &mut RgbaImage);
    fn handle_click(&mut self, _x: i32, _y: i32) {} // Default empty implementation
    fn size(&self) -> (i32, i32);
}