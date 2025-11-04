use rusttype::Font;
use std::fs;

pub fn load_font(path: &str) -> Font<'static> {
    let data = fs::read(path).expect("Failed to read font");
    Font::try_from_vec(data).expect("Invalid font data")
}
