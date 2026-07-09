//! A interactive text input widget

use crate::widget::Widget;
use image::Rgba;
use imageproc::rect::Rect;
use rusttype::{Font, Scale};
use std::any::Any;

/// A text input field that allows users to type single-line textual data.
///
/// It manages user keyboard entry (characters and backspace) when focused, 
/// and automatically applies sub-image clipping and panning when the entered 
/// text overflows the bounding box width.
/// 
/// Example usage:
/// ```no_run
/// use cpux::text_input::TextInput;
/// let input = TextInput::new(font.clone())
///     .with_id("username_field")
///     .with_size(250, 40);
/// ```
pub struct TextInput {
    /// The optional ID of the input box, essential for routing keystrokes via focus management.
    pub id: Option<String>,
    /// The current text content
    pub content: String,
    /// The rectangle defining the input box's position and dimensions.
    pub rect: Rect,
    /// The font used to render the characters inside the input area.
    pub font: Font<'static>,
    /// The scale at which to render the typed text characters.
    pub scale: Scale,
    /// The color of the input text and simulated text cursor.
    pub text_color: Rgba<u8>,
    /// The background fill color of the input field rectangle.
    pub bg_color: Rgba<u8>,
    /// The boundary stroke color used to highlight borders.
    pub border_color: Rgba<u8>,
}

impl TextInput {
    /// Creates a new text input widget with the specified font
    /// 
    /// Example:
    /// ```no_run
    /// use cpux::text_input::TextInput;
    /// let input = TextInput::new(font.clone());
    /// ```
    /// This makes an empty input field spanning a default width of 200 pixels 
    /// and a height of 40 pixels, with a light gray background and black text.
    /// 
    /// You can customize the size, colors, and ID by modifiying the properties
    /// after creating the widget: (This requires the widget to be mutable)
    /// ```no_run
    /// use cpux::text_input::TextInput;
    /// let mut input = TextInput::new(font.clone());
    /// input.text_color = Rgba([255, 0, 0, 255]); // Red text
    /// input.bg_color = Rgba([0, 0, 0, 255]); // Black background
    /// ```
    pub fn new(font: Font<'static>) -> Self {
        Self {
            id: None,
            content: String::new(),
            rect: Rect::at(0, 0).of_size(200, 40), // Default size
            font,
            scale: Scale { x: 24.0, y: 24.0 },
            text_color: Rgba([0, 0, 0, 255]),      // Black text
            bg_color: Rgba([240, 240, 240, 255]),  // Light gray background
            border_color: Rgba([150, 150, 150, 255]),
        }
    }

    /// Assigns a unique string identifier to the text input widget.
    ///
    /// Example:
    /// ```no_run
    /// use cpux::text_input::TextInput;
    /// let input = TextInput::new(font.clone())
    ///     .with_id("search_bar");
    /// ```
    /// Providing an ID string is required if you want this widget to be queryable 
    /// via `AppContext::find_widget_mut()` or to register keyboard state focus.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Overrides the default layout bounding dimensions of the input box.
    ///
    /// Example:
    /// ```no_run
    /// use cpux::text_input::TextInput;
    /// let input = TextInput::new(font.clone())
    ///     .with_size(350, 45);
    /// ```
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.rect = Rect::at(self.rect.left(), self.rect.top()).of_size(width, height);
        self
    }
}

// Internal trait implementations
impl Widget for TextInput {
    fn set_position(&mut self, x: i32, y: i32) {
        self.rect = Rect::at(x, y).of_size(self.rect.width(), self.rect.height());
    }

    fn draw(&mut self, framebuffer: &mut image::RgbaImage) {
        imageproc::drawing::draw_filled_rect_mut(framebuffer, self.rect, self.bg_color);
        
        let border_rect = Rect::at(self.rect.left(), self.rect.top()).of_size(self.rect.width(), 1);
        imageproc::drawing::draw_filled_rect_mut(framebuffer, border_rect, self.border_color);

        let inner_width = self.rect.width() - 2;
        let inner_height = self.rect.height() - 2;
        let mut interior = image::RgbaImage::from_pixel(inner_width, inner_height, self.bg_color);

        let display_text = format!("{}|", self.content);
        let (text_w, _text_h) = imageproc::drawing::text_size(self.scale, &self.font, &display_text);
        
        let padding = 8;
        let max_text_width = inner_width as i32 - (padding * 2);

        let mut text_x = padding;
        if text_w > max_text_width {
            text_x = inner_width as i32 - padding - text_w;
        }

        let text_y = 8;

        imageproc::drawing::draw_text_mut(
            &mut interior,
            self.text_color,
            text_x,
            text_y,
            self.scale,
            &self.font,
            &display_text,
        );

        image::imageops::overlay(
            framebuffer, 
            &interior, 
            self.rect.left() as i64 + 1, 
            self.rect.top() as i64 + 1
        );
    }

    fn handle_click(&mut self, x: i32, y: i32) -> Option<String> {
        let r = self.rect;
        if x >= r.left() && x <= r.right() && y >= r.top() && y <= r.bottom() {
            return self.id.clone();
        }
        None
    }

    fn size(&self) -> (i32, i32) {
        (self.rect.width() as i32, self.rect.height() as i32)
    }

    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}