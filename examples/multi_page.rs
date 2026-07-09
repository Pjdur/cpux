use cpux::app::{App, AppContext};
use cpux::layouts::Column;
use cpux::text::Text;
use cpux::button::Button;
use cpux::load_font;
use rusttype::Font;

// 1. Keep track of what page the user is looking at
enum WizardPage {
    Welcome,
    License,
    Installing,
}

fn main() {
    let font = load_font("C:\\Windows\\Fonts\\Arial.ttf");
    let mut app = App::new("CPUX Setup Wizard", 600, 450);
    
    // Track current state
    let mut current_page = WizardPage::Welcome;

    // Set up the initial screen layout
    app.set_root(build_welcome_page(font.clone()));

    // 2. The Main Interaction Loop
    app.run(move |ui: &mut AppContext| {
        match current_page {
            WizardPage::Welcome => {
                if ui.was_clicked("next_to_license") {
                    current_page = WizardPage::License;
                    // Swap the layout tree completely!
                    ui.widgets.clear(); // Clear old root elements manually
                    // Or call a utility function to replace the root if needed.
                    // Because set_root is on App, and inside the closure we have AppContext,
                    // we can modify ui.widgets directly or clear and add:
                    let new_root = build_license_page(font.clone());
                    ui.widgets.push(Box::new(new_root));
                }
            }
            WizardPage::License => {
                if ui.was_clicked("next_to_install") {
                    current_page = WizardPage::Installing;
                    ui.widgets.clear();
                    let new_root = build_install_page(font.clone());
                    ui.widgets.push(Box::new(new_root));
                }
            }
            WizardPage::Installing => {
                if ui.was_clicked("finish") {
                    std::process::exit(0); // Exit app cleanly
                }
            }
        }
    });
}

// 3. Separate structural factories to keep code incredibly clean:

fn build_welcome_page(font: Font<'static>) -> Column {
    Column::new()
        .at(50, 50)
        .with_spacing(20)
        .add(Text::new("Welcome to the Installer!", font.clone()).with_size(32.0))
        .add(Text::new("This wizard will guide you through setting up CPUX.", font.clone()))
        .add(Button::new("Next >", font.clone()).with_id("next_to_license"))
}

fn build_license_page(font: Font<'static>) -> Column {
    Column::new()
        .at(50, 50)
        .with_spacing(20)
        .add(Text::new("License Agreement", font.clone()).with_size(32.0))
        .add(Text::new("Do you promise to write neat code?", font.clone()))
        .add(Button::new("I Accept & Next >", font.clone()).with_id("next_to_install"))
}

fn build_install_page(font: Font<'static>) -> Column {
    Column::new()
        .at(50, 50)
        .with_spacing(20)
        .add(Text::new("Installation Complete!", font.clone()).with_size(32.0))
        .add(Text::new("CPUX has been successfully configured.", font.clone()))
        .add(Button::new("Finish", font.clone()).with_id("finish"))
}