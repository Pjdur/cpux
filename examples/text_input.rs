use cpux::app::{App, AppContext, TypedEvent};
use cpux::button::Button;
use cpux::layouts::Column;
use cpux::text::Text;
use cpux::text_input::TextInput;
use cpux::load_font;

fn main() {
    // 1. Load the font and set up the app
    let font = load_font("assets/arial.ttf");
    let mut app = App::new("CPUX Text Input Test", 600, 400);

    // 2. Create the widgets
    let title = Text::new("Enter your name:", font.clone())
        .with_size(28.0);

    let input_field = TextInput::new(font.clone())
        .with_id("name_input")
        .with_size(300, 40);

    let submit_btn = Button::new("Greet Me", font.clone())
        .with_id("submit_btn")
        .with_size(150, 40);

    let output_label = Text::new("Waiting for input...", font.clone())
        .with_id("greeting_output")
        .with_size(24.0);

    // 3. Arrange them in a column
    let layout = Column::new()
        .at(50, 50)
        .with_spacing(20)
        .add(title)
        .add(input_field)
        .add(submit_btn)
        .add(output_label);

    app.set_root(layout);

    app.run(move |ui: &mut AppContext| {
        if ui.clicked_id.is_some() {
            ui.active_focus = ui.clicked_id.clone();
        }

        // If something is focused, and a key was typed, route the key to that widget
        if let Some(focus_id) = &ui.active_focus.clone() {
            if let Some(event) = ui.last_typed_char {
                // Try to find the focused widget specifically as a TextInput
                if let Some(input_widget) = ui.find_widget_mut::<TextInput>(focus_id) {
                    match event {
                        TypedEvent::Char(c) => {
                            input_widget.content.push(c);
                        }
                        TypedEvent::Backspace => {
                            input_widget.content.pop();
                        }
                    }
                }
            }
        }
        
        if ui.was_clicked("submit_btn") {
            // 1. Read the text from the input field
            let mut name_entered = String::new();
            if let Some(input_widget) = ui.find_widget_mut::<TextInput>("name_input") {
                name_entered = input_widget.content.clone();
            }

            // 2. Write the text to the output label
            if let Some(output_widget) = ui.find_widget_mut::<Text>("greeting_output") {
                if name_entered.trim().is_empty() {
                    output_widget.content = "Please enter a valid name!".to_string();
                } else {
                    output_widget.content = format!("Hello, {}! Welcome to CPUX.", name_entered);
                }
            }
        }
    });
}