//! cpux is a retained-mode GUI framework.
//! It is meant to be simple to use and lightweight,
//! using very little CPU when idle.
//! 
//! # Example 
//! _It might seem a bit complex, but the crate is quite
//! well documented_
//! ```no_run
//! use cpux::app::{App, AppContext};
//! use cpux::button::Button;
//! use cpux::layouts::Column;
//! use cpux::text::Text;
//! use cpux::load_font;
//! 
//! fn main() {
//!     let font = load_font("assets/arial.ttf");
//! 
//!     let mut app = App::new("cpux counter example", 600, 400);
//!     let mut counter = 0;
//! 
//!     let text1 = Text::new("Hello, GUI!", font.clone()).with_size(32.0);
//!     let text2 = Text::new("Count: 0", font.clone())
//!         .with_id("counter_text");
//!     let button = Button::new("Click Me", font.clone())
//!         .with_id("add_button");
//! 
//! 
//!     let column = Column::new()
//!         .at(50, 50)
//!         .with_spacing(25)
//!         .add(text1)
//!         .add(text2)
//!         .add(button);
//! 
//!     app.set_root(column);
//! 
//!     app.run(move |ui: &mut AppContext| {
//!         if ui.was_clicked("add_button") {
//!             counter += 1;
//!             if let Some(text_widget) = ui.find_widget_mut::<Text>("counter_text") {
//!                 text_widget.content = format!("Count: {}", counter);
//!             }
//!        }
//!     });
//! }
//! ```
//! 

pub mod app;
pub mod button;
pub mod layouts;
pub mod text;
pub mod widget;
pub mod text_input;

/// Helper utility to load raw TrueType Font (`.ttf`) files into vector memory.
/// 
/// # Example
/// ```no_run
/// use cpux::load_font;
/// let font = load_font("assets/arial.ttf");
/// ```
pub fn load_font(path: &str) -> rusttype::Font<'static> {
    let font_data = std::fs::read(path).expect("Failed to read font file");
    rusttype::Font::try_from_vec(font_data).expect("Error constructing Font")
}