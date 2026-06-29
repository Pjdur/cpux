//! Core widget trait, implemented by layouts and widgets.
use std::any::Any;

/// The core trait that all visual components and layouts must implement.
/// 
/// Implementing `Widget` allows a component to be positioned, drawn onto the 
/// frame buffer, and automatically managed by the layout system.
pub trait Widget {
    /// Sets the absolute (X, Y) pixel position of the widget on the canvas.
    fn set_position(&mut self, x: i32, y: i32);

    /// Draws the widget onto the provided pixel framebuffer array.
    fn draw(&mut self, image: &mut image::RgbaImage);

    /// Checks if a mouse click at (X, Y) lands inside this widget.
    /// Returns `Some(id)` if clicked, or `None` if missed.
    fn handle_click(&mut self, _x: i32, _y: i32) -> Option<String> { 
        None 
    }

    /// Returns the layout bounding box dimensions of the widget as `(width, height)`.
    fn size(&self) -> (i32, i32);

    /// Returns the optional unique string identifier assigned to this widget.
    fn id(&self) -> Option<&str> { 
        None 
    }

    /// Used by the framework to downcast a generic widget back into its 
    /// concrete type (like `Text` or `Button`) to safely mutate internal data.
    fn as_any_mut(&mut self) -> &mut dyn Any;

    /// Allows containers to look inside themselves for a specific widget ID.
    fn find_child_mut(&mut self, _id: &str) -> Option<&mut dyn Widget> { 
        None 
    }
}