use crate::widget::Widget;
use image::RgbaImage;

pub struct Row {
    pub children: Vec<Box<dyn Widget>>,
    pub spacing: i32,
    pub position: (i32, i32),
}

impl Widget for Row {
    fn set_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }

    fn draw(&mut self, image: &mut RgbaImage) {
        let mut current_x = self.position.0;
        for child in &mut self.children {
            child.set_position(current_x, self.position.1);
            child.draw(image);
            
            // Advance by the actual child width + our padding spacing
            let (width, _) = child.size();
            current_x += width + self.spacing;
        }
    }

    fn handle_click(&mut self, x: i32, y: i32) {
        for child in &mut self.children {
            child.handle_click(x, y);
        }
    }

    fn size(&self) -> (i32, i32) {
        if self.children.is_empty() {
            return (0, 0);
        }
        
        let mut total_width = 0;
        let mut max_height = 0;
        
        for (i, child) in self.children.iter().enumerate() {
            let (w, h) = child.size();
            total_width += w;
            if i < self.children.len() - 1 {
                total_width += self.spacing;
            }
            if h > max_height {
                max_height = h;
            }
        }
        
        (total_width, max_height)
    }
}

pub struct Column {
    pub children: Vec<Box<dyn Widget>>,
    pub spacing: i32,
    pub position: (i32, i32),
}

impl Widget for Column {
    fn set_position(&mut self, x: i32, y: i32) {
        self.position = (x, y);
    }

    fn draw(&mut self, image: &mut RgbaImage) {
        let mut current_y = self.position.1;
        for child in &mut self.children {
            child.set_position(self.position.0, current_y);
            child.draw(image);
            
            // Advance by the actual child height + our padding spacing
            let (_, height) = child.size();
            current_y += height + self.spacing;
        }
    }

    fn handle_click(&mut self, x: i32, y: i32) {
        for child in &mut self.children {
            child.handle_click(x, y);
        }
    }

    fn size(&self) -> (i32, i32) {
        if self.children.is_empty() {
            return (0, 0);
        }
        
        let mut max_width = 0;
        let mut total_height = 0;
        
        for (i, child) in self.children.iter().enumerate() {
            let (w, h) = child.size();
            total_height += h;
            if i < self.children.len() - 1 {
                total_height += self.spacing;
            }
            if w > max_width {
                max_width = w;
            }
        }
        
        (max_width, total_height)
    }
}