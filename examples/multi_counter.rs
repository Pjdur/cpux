use cpux::app::{App, AppContext};
use cpux::button::Button;
use cpux::layouts::Column;
use cpux::text::Text;
use cpux::load_font;

fn main() {
    let font = load_font("../assets/arial.ttf");

    let mut app = App::new("cpux multi-counter example", 600, 400);

    // This same value used by multiple widgets.
    let mut counter = 0;

    let text1 = Text::new("Hello, GUI!", font.clone()).with_size(32.0);
    let text2 = Text::new("Count: 0", font.clone())
        .with_id("counter_text")
        .with_size(24.0);
    let button = Button::new("Click Me", font.clone())
        .with_id("add_button");
    let text3 = Text::new("Count: 0", font.clone())
        .with_id("counter_text2")
        .with_size(24.0);
    let button2 = Button::new("Another Button", font.clone())
    .with_id("add_button2");

    let column = Column::new()
        .at(50, 50)
        .with_spacing(25)
        .add(text1)
        .add(text2)
        .add(button)
        .add(text3)
        .add(button2);

    app.set_root(column);

    app.run(move |ui: &mut AppContext| {
        if ui.was_clicked("add_button") {
            counter += 1;
            if let Some(text_widget) = ui.find_widget_mut::<Text>("counter_text") {
                text_widget.content = format!("Count: {}", counter);
            }
        } else if ui.was_clicked("add_button2") {
            counter += 1;
            if let Some(text_widget) = ui.find_widget_mut::<Text>("counter_text2") {
                text_widget.content = format!("Count: {}", counter);
            }
        }
    });
}
