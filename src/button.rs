use crate::widget::Widget;
use image::Rgba;
use imageproc::rect::Rect;
use rusttype::{Font, Scale};
use std::any::Any;

pub struct Button {
    pub id: Option<String>,
    pub label: String,
    pub rect: Rect,
    pub font: Font<'static>,
    pub scale: Scale,
    pub text_color: Rgba<u8>,
    pub bg_color: Rgba<u8>,
}

// todo: document this later
impl Button {
    pub fn new(label: impl Into<String>, font: Font<'static>) -> Self {
        let label_str = label.into();
        let scale = Scale { x: 24.0, y: 24.0 };

        let v_metrics = font.v_metrics(scale);
        let glyphs: Vec<_> = font
            .layout(&label_str, scale, rusttype::point(0.0, v_metrics.ascent))
            .collect();

        let text_width = if glyphs.is_empty() {
            0
        } else {
            let min_x = glyphs
                .first()
                .unwrap()
                .pixel_bounding_box()
                .map_or(0, |b| b.min.x);
            let max_x = glyphs
                .last()
                .unwrap()
                .pixel_bounding_box()
                .map_or(0, |b| b.max.x);
            (max_x - min_x) as u32
        };

        // Define your desired padding constants
        let padding_x = 24; // 12 pixels padding on left and right
        let auto_width = text_width + padding_x;
        let auto_height = 40; // Clean fixed height standard for buttons

        Self {
            id: None,
            label: label_str,
            rect: Rect::at(0, 0).of_size(auto_width, auto_height),
            font,
            scale,
            text_color: Rgba([255, 255, 255, 255]),
            bg_color: Rgba([0, 0, 255, 255]),
        }
    }

    pub fn with_id(mut self, id: impl Into<String>) -> Self {
        self.id = Some(id.into());
        self
    }

    /// Manual size override
    /// Example usage:
    /// ```no_run
    /// use cpux::button::Button;
    /// use cpux::load_font();
    /// 
    /// let font = load_font("assets/arial.ttf");
    /// let button = Button::new("Click Me", font.clone())
    ///     .with_size(150, 50)
    ///     .with_id("add_button");
    /// ```
    pub fn with_size(mut self, width: u32, height: u32) -> Self {
        self.rect = Rect::at(self.rect.left(), self.rect.top()).of_size(width, height);
        self
    }
}

impl Widget for Button {
    fn set_position(&mut self, x: i32, y: i32) {
        self.rect = Rect::at(x, y).of_size(self.rect.width(), self.rect.height());
    }

    fn draw(&mut self, framebuffer: &mut image::RgbaImage) {
        imageproc::drawing::draw_filled_rect_mut(framebuffer, self.rect, self.bg_color);

        let text_x = self.rect.left() + 12;
        let text_y = self.rect.top() + 8;

        imageproc::drawing::draw_text_mut(
            framebuffer,
            self.text_color,
            text_x,
            text_y,
            self.scale,
            &self.font,
            &self.label,
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
