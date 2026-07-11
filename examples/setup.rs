use cpux::app::{App, AppContext};
use cpux::button::Button;
use cpux::layouts::Column;
use cpux::text::Text;
use cpux::load_font;

fn main() {
    let font = load_font("assets/arial.ttf");
    let mut app = App::new("CPUX installer example", 600, 400);

    let mut current_pct = 0.0; 

    let header = Text::new("CPUX installer", font.clone()).with_size(36.0);
    let percentage_widget = Text::new("0%", font.clone()).with_id("percentage");
    let start_button = Button::new("Start Installer", font.clone()).with_id("start");

    let column = Column::new()
        .at(50, 50)
        .with_spacing(25)
        .add(header)
        .add(percentage_widget)
        .add(start_button);

    app.set_root(column);

    app.run(move |ui: &mut AppContext| {
        
        // If clicked, start animating
        if ui.was_clicked("start") && current_pct < 100.0 {
            ui.is_animating = true;
            if let Some(button) = ui.find_widget_mut::<Button>("start") {
                button.label = "Installing...".to_string();
            }
        }

        // This logic runs automatically while is_animating == true
        if ui.is_animating {
            current_pct += 0.5;

            if current_pct >= 100.0 {
                current_pct = 100.0;
                ui.is_animating = false;

                if let Some(button) = ui.find_widget_mut::<Button>("start") {
                    button.label = "Finished!".to_string();
                }
            }

            if let Some(tex) = ui.find_widget_mut::<Text>("percentage") {
                tex.content = format!("{}%", current_pct as u32);
            }
        }
    });
}