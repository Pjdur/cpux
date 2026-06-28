use crate::widget::Widget;
use image::RgbaImage;
use std::any::Any;

/// A layout container that arranges its child widgets vertically in a single line.
/// 
/// `Column` handles the geometric calculations to distribute child items from top to
/// bottom, automatically applying the configured spacing gap between elements.
pub struct Column {
    /// The collection of box-allocated components managed by this layout.
    pub children: Vec<Box<dyn Widget>>,
    /// Pixel spacing gap inserted between adjacent widgets.
    pub spacing: i32,
    /// Absolute (X, Y) layout anchor coordinate on the parent canvas.
    pub position: (i32, i32),
}

impl Column {
    /// Creates a new Column
    /// 
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            spacing: 10,
            position: (0, 0),
        }
    }

    /// Sets the absolute starting position of the column container.
    /// Example:
    /// ```
    /// use cpux::layouts::Column;
    /// let column = Column::new()
    ///     .at(50, 50)
    ///     // ...
    /// ```
    pub fn at(mut self, x: i32, y: i32) -> Self {
        self.position = (x, y);
        self
    }

    /// Configures the pixel spacing gap to draw between adjacent items (vertically).
    /// ```
    /// use cpux::layouts::Column;
    /// let column = Column::new()
    ///     .at(50, 50)
    ///     .with_spacing(25);
    ///     // ...
    /// ```
    pub fn with_spacing(mut self, spacing: i32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Adds a widget to the Column stack in the order it was added.
    /// The first widget will be drawn at the top, and subsequent widgets 
    /// will be drawn below it.
    /// ```
    /// use cpux::layouts::Column;
    /// use cpux::text::Text;
    /// use cpux::button:Button;
    /// 
    /// let text = Text::new("Hello, GUI!", font.clone()).with_size(32.0);
    /// let button = Button::new("Click Me", font.clone())
    ///    .with_id("add_button");
    /// 
    /// let column = Column::new()
    ///     .at(50, 50)
    ///     .with_spacing(25)
    ///     .add(text)
    ///     .add(button);
    /// ```
    pub fn add(mut self, widget: impl Widget + 'static) -> Self {
        self.children.push(Box::new(widget));
        self
    }
}

// Internal methods
impl Widget for Column {
    fn set_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }

    fn draw(&mut self, framebuffer: &mut RgbaImage) {
        let mut current_y = self.position.1;
        for child in &mut self.children {
            child.set_position(self.position.0, current_y);
            child.draw(framebuffer);
            current_y += child.size().1 + self.spacing;
        }
    }

    fn handle_click(&mut self, x: i32, y: i32) -> Option<String> {
        for child in &mut self.children {
            if let Some(clicked_id) = child.handle_click(x, y) {
                return Some(clicked_id);
            }
        }
        None
    }

    fn size(&self) -> (i32, i32) {
        let mut width = 0;
        let mut height = 0;
        for child in &self.children {
            let (w, h) = child.size();
            if w > width { 
                width = w; 
            }
            height += h + self.spacing;
        }
        if !self.children.is_empty() {
            height -= self.spacing;
        }
        (width, height)
    }

    fn id(&self) -> Option<&str> {
        None
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn find_child_mut(&mut self, id: &str) -> Option<&mut dyn Widget> {
        for child in &mut self.children {
            if child.id() == Some(id) {
                return Some(child.as_mut());
            }
            if let Some(found) = child.find_child_mut(id) {
                return Some(found);
            }
        }
        None
    }
}

/// A layout container that arranges its child widgets horizontally in a single line.
/// 
/// `Row` handles the geometric calculations to distribute child items from left to
/// right, automatically applying the configured spacing gap between elements.
pub struct Row {
    /// The collection of box-allocated components managed by this layout.
    pub children: Vec<Box<dyn Widget>>,
    /// Pixel spacing gap inserted between adjacent widgets.
    pub spacing: i32,
    /// Absolute (X, Y) layout anchor coordinate on the parent canvas.
    pub position: (i32, i32),
}

impl Row {
    /// Creates a empty, default horizontal layout.
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            spacing: 10,
            position: (0, 0),
        }
    }

    /// Sets the starting position of the Row, like `Column::at()`.
    /// Example:
    /// ```
    /// use cpux::layouts::Row;
    /// let row = Row::new()
    ///     .at(50, 50)
    ///     .with_spacing(25);
    ///     // ...
    /// ```
    pub fn at(mut self, x: i32, y: i32) -> Self {
        self.position = (x, y);
        self
    }

    /// Configures the pixel spacing gap to draw between adjacent items
    /// horizontally, similar to `Column::with_spacing()`.
    /// ```
    /// use cpux::layouts::Row;
    /// let Row = Row::new()
    ///     .at(50, 50)
    ///     .with_spacing(25);
    ///     // ...
    /// ```
    pub fn with_spacing(mut self, spacing: i32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Adds a widget to the Row stack in the order it was added.
    /// The first widget will be drawn at the top, and subsequent widgets 
    /// will be drawn below it, like `Column::add()`.
    /// ```
    /// use cpux::layouts::Row;
    /// use cpux::text::Text;
    /// use cpux::button:Button;
    /// 
    /// let text = Text::new("Hello, GUI!", font.clone()).with_size(32.0);
    /// let button = Button::new("Click Me", font.clone())
    ///    .with_id("add_button");
    /// 
    /// let row = Row::new()
    ///     .at(50, 50)
    ///     .with_spacing(25)
    ///     .add(text)
    ///     .add(button);
    /// ```
    pub fn add(mut self, widget: impl Widget + 'static) -> Self {
        self.children.push(Box::new(widget));
        self
    }
}

// Internal methods
impl Widget for Row {
    fn set_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }

    fn draw(&mut self, framebuffer: &mut RgbaImage) {
        let mut current_x = self.position.0;
        for child in &mut self.children {
            child.set_position(current_x, self.position.1);
            child.draw(framebuffer);
            current_x += child.size().0 + self.spacing;
        }
    }

    fn handle_click(&mut self, x: i32, y: i32) -> Option<String> {
        for child in &mut self.children {
            if let Some(clicked_id) = child.handle_click(x, y) {
                return Some(clicked_id);
            }
        }
        None
    }

    fn size(&self) -> (i32, i32) {
        let mut width = 0;
        let mut height = 0;
        for child in &self.children {
            let (w, h) = child.size();
            if h > height { 
                height = h; 
            }
            width += w + self.spacing;
        }
        if !self.children.is_empty() {
            width -= self.spacing;
        }
        (width, height)
    }

    fn id(&self) -> Option<&str> {
        None
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn find_child_mut(&mut self, id: &str) -> Option<&mut dyn Widget> {
        for child in &mut self.children {
            if child.id() == Some(id) {
                return Some(child.as_mut());
            }
            if let Some(found) = child.find_child_mut(id) {
                return Some(found);
            }
        }
        None
    }
}