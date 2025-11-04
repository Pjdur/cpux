mod app;
mod font;
mod render;
mod widget;

use app::App;
use font::load_font;
use image::Rgba;
use rusttype::Scale;
use widget::text::Text;
use imageproc::rect::Rect;

fn main() {
    env_logger::init();

    let font = load_font("C:/Windows/Fonts/arial.ttf");
    let text1 = Text {
        content: "Hello, GUI!".to_string(),
        position: (50, 80),
        color: Rgba([0, 0, 0, 255]),
        scale: Scale { x: 32.0, y: 32.0 },
        font: font.clone(),
    };

    let text2 = Text {
        content: "Welcome to the GUI!".to_string(),
        position: (50, 120),
        color: Rgba([0, 0, 0, 255]),
        scale: Scale { x: 32.0, y: 32.0 },
        font: font.clone(),
    };

    use widget::button::Button;

    let button = Button {
        label: "Click Me".to_string(),
        rect: Rect::at(50, 160).of_size(120, 40),
        font: font.clone(),
        scale: Scale { x: 24.0, y: 24.0 },
        text_color: Rgba([255, 255, 255, 255]),
        bg_color: Rgba([0, 0, 255, 255]),
        on_click: Some(Box::new(|| {
            println!("Button clicked!");
        })),
    };

    let app = App::new("Minimal GUI", 400, 200);
    app.run(vec![Box::new(text1), Box::new(text2), Box::new(button)]);
}
