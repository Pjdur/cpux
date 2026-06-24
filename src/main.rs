mod app;
mod button;
mod layouts;
mod text;
mod widget;

use app::App;
use button::Button;
use image::Rgba;
use imageproc::rect::Rect;
use layouts::Column;
use rusttype::{Font, Scale};
use text::Text;

fn load_font(path: &str) -> Font<'static> {
    let data = std::fs::read(path).expect("Failed to read font");
    Font::try_from_vec(data).expect("Invalid font data")
}

fn main() {
    env_logger::init();

    let font = load_font("C:/Windows/Fonts/arial.ttf");

    let text1 = Text {
        content: "Hello, GUI!".to_string(),
        position: (0, 0), // Managed by parent Row layout now
        color: Rgba([0, 0, 0, 255]),
        scale: Scale { x: 32.0, y: 32.0 },
        font: font.clone(),
    };

    let text2 = Text {
        content: "Welcome to the GUI!".to_string(),
        position: (0, 0),
        color: Rgba([0, 0, 0, 255]),
        scale: Scale { x: 32.0, y: 32.0 },
        font: font.clone(),
    };

    let button = Button {
        label: "Click Me".to_string(),
        rect: Rect::at(0, 0).of_size(120, 40),
        font: font.clone(),
        scale: Scale { x: 24.0, y: 24.0 },
        text_color: Rgba([255, 255, 255, 255]),
        bg_color: Rgba([0, 0, 255, 255]),
        on_click: Some(Box::new(|| {
            println!("Button clicked successfully!");
        })),
    };

    let column = Column {
        children: vec![Box::new(text1), Box::new(text2), Box::new(button)],
        spacing: 30, // 30px gap looks great vertically
        position: (50, 50),
    };

    let app = App::new("Simplified GUI Framework", 600, 400);
    app.run(vec![Box::new(column)]);
}
