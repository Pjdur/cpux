use cpux::app::{App, AppContext};
use cpux::button::Button;
use cpux::layouts::Column;
use cpux::text::Text;
use cpux::load_font;

fn main() {
    // Loads a font to use. In this case, it's Arial.
    let font = load_font("assets/arial.ttf");

    // Creates an app, with a title, width and height
    let mut app = App::new("cpux counter example", 600, 400);

    // Counter variable, which will be updated when the button is clicked.
    let mut counter = 0;

    // A simple text widget with content, uses the loaded font, with a size of 32.
    // By default, a text widget has a size of 24, but here it's overrided using `Text::with_size()`
    let text1 = Text::new("Hello, GUI!", font.clone()).with_size(32.0);

    // Creates a text widget to display the counter value, with an initial value of 0.
    // This text widget has an ID so it can be found and updated.
    let text2 = Text::new("Count: 0", font.clone())
        .with_id("counter_text");

    // Creates a button widget with the label "Click Me", using the loaded font.
    let button = Button::new("Click Me", font.clone())
        .with_id("add_button");

    // A column layout that contains the text and button widgets,
    // with a spacing of 25 pixels between them vertically.
    let column = Column::new()
        .at(50, 50)
        .with_spacing(25)
        .add(text1)
        .add(text2)
        .add(button);

    // Sets the root container, a Column in this example
    app.set_root(column);

    // Runs the app.
    // The app is asleep until an event happens, like a click.
    // `was_clicked` checks if a widget with the given ID was clicked.
    // if it was, `find_widget_mut` finds the widget with that ID
    // and updates its content.
    app.run(move |ui: &mut AppContext| {
        if ui.was_clicked("add_button") {
            counter += 1;
            if let Some(text_widget) = ui.find_widget_mut::<Text>("counter_text") {
                text_widget.content = format!("Count: {}", counter);
            }
        }
    });
}
