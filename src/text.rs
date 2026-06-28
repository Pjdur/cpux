use crate::widget::Widget;
use image::Rgba;
use image::RgbaImage;
use rusttype::{Font, Scale};
use std::any::Any;

/// A simple text label widget designed to render string contents.
pub struct Text {
    /// The unique identifier used to find and update this label.
    pub id: Option<String>,
    /// The string text to display on the screen.
    pub content: String,
    /// Absolute (X, Y) rendering position.
    pub position: (i32, i32),
    /// Foreground text color (defaults to solid black).
    pub color: Rgba<u8>,
    /// The font size/scale metrics.
    pub scale: Scale,
    /// The TrueType font used to render text glyphs.
    pub font: Font<'static>,
}

impl Text {
    /// Creates a new text label with a default size of 24.0.
    pub fn new(content: impl Into<String>, font: Font<'static>) -> Self {
        Self {
            id: None,
            content: content.into(),
            position: (0, 0),
            color: Rgba([0, 0, 0, 255]),
            scale: Scale { x: 24.0, y: 24.0 },
            font,
        }
    }

    /// Assigns an identifier tag to this text widget.
    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Overrides the font size scale uniform factor.
    pub fn with_size(mut self, size: f32) -> Self {
        self.scale = Scale { x: size, y: size };
        self
    }
}

impl Widget for Text {
    fn set_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }

    fn draw(&mut self, framebuffer: &mut RgbaImage) {
        imageproc::drawing::draw_text_mut(
            framebuffer,
            self.color,
            self.position.0,
            self.position.1,
            self.scale,
            &self.font,
            &self.content,
        );
    }

    fn size(&self) -> (i32, i32) {
        // Approximate width sizing using text length multiplied by average char radius
        ((self.content.len() as i32 * (self.scale.x * 0.5) as i32), self.scale.y as i32)
    }

    fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}