//! Core of the GUI framework

use crate::widget::Widget;
use image::{Rgba, RgbaImage};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    dpi::LogicalSize,
    event::{ElementState, Event, KeyboardInput, MouseButton, VirtualKeyCode, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

/// Represents captured keyboard input
///
/// # Example
/// ```no_run
/// use cpux::app::TypedEvent;
///
/// let event = TypedEvent::Char('A');
/// match event {
///     TypedEvent::Char(c) => println!("User typed: {}", c),
///     TypedEvent::Backspace => println!("User pressed backspace"),
/// }
/// ```
/// Represents captured keyboard input
///
/// # Example
/// ```no_run
/// use cpux::app::TypedEvent;
///
/// let event = TypedEvent::Char('A');
/// match event {
///     TypedEvent::Char(c) => println!("User typed: {}", c),
///     TypedEvent::Backspace => println!("User pressed backspace"),
/// }
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum TypedEvent {
    /// A character input event
    Char(char),
    /// A text deletion event triggered by pressing the Backspace key.
    /// A text deletion event triggered by pressing the Backspace key.
    Backspace,
}

/// Contains data and context about the App,
/// like the contained widgets and the ID of a widget clicked during a frame.
pub struct AppContext {
    /// Vector containing the widgets of the App.
    pub widgets: Vec<Box<dyn Widget>>,
    /// The ID string of whichever widget was clicked during a frame.
    pub clicked_id: Option<String>,
    /// The currently focused widget
    pub active_focus: Option<String>,
    /// The last typed character event, if any, during a frame.
    pub last_typed_char: Option<TypedEvent>,
}

impl AppContext {
    /// Returns `true` if the widget with the given ID was clicked.
    /// Useful for handling events and triggering logic based on user interaction.
    ///
    /// # Example
    /// ```no_run
    /// use cpux::app::AppContext;
    /// fn main() {
    ///         // ...                                  
    ///         app.run(move |ui: &mut AppContext| {
    ///         if ui.was_clicked("add_button") {
    ///             counter += 1;
    ///             // ...
    ///         }
    ///     });
    /// }
    /// ```
    pub fn was_clicked(&self, id: &str) -> bool {
        self.clicked_id.as_deref() == Some(id)
    }

    /// Recursively scans through layouts and widgets to find a matching ID.
    ///
    /// # Example
    /// ```no_run
    /// use cpux::app::AppContext;
    /// fn main() {
    ///         // ...                                  
    ///         app.run(move |ui: &mut AppContext| {
    ///         if ui.was_clicked("add_button") {
    ///             counter += 1;
    ///             if let Some(text_widget) = ui.find_widget_mut::<Text>("counter_text") {
    ///                 text_widget.content = format!("Count: {}", counter);
    ///             }
    ///         }
    ///     });
    /// }
    /// ```
    pub fn find_widget_mut<'a, T: 'static>(&'a mut self, id: &str) -> Option<&'a mut T> {
        for widget in &mut self.widgets {
            if widget.id() == Some(id) {
                return widget.as_any_mut().downcast_mut::<T>();
            }
            if let Some(found_widget) = widget.find_child_mut(id) {
                return found_widget.as_any_mut().downcast_mut::<T>();
            }
        }
        None
    }
}

/// The main application struct that manages the OS window, framebuffer, and widget tree.
pub struct App {
    event_loop: Option<EventLoop<()>>,
    window: Window,
    pixels: Pixels,
    framebuffer: RgbaImage,
    /// The collection of top-level widgets associated with this application.
    pub widgets: Vec<Box<dyn Widget>>,
}

impl App {
    /// Spawns an OS Window with the given title and dimensions.
    /// Example
    /// ```no_run
    /// use cpux::app::App;
    /// fn main() {
    ///     let mut app = App::new("Window", 600, 400);
    /// }
    /// ```
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(LogicalSize::new(width, height))
            .build(&event_loop)
            .unwrap();

        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(width, height, surface_texture).unwrap();
        let framebuffer = RgbaImage::new(width, height);

        Self {
            event_loop: Some(event_loop),
            window,
            pixels,
            framebuffer,
            widgets: Vec::new(),
        }
    }

    /// Sets the top-level layout container, e.g. a Row or Column.
    pub fn set_root(&mut self, widget: impl Widget + 'static) {
        self.widgets = vec![Box::new(widget)];
    }

    /// Starts the main event loop.
    pub fn run<F>(mut self, mut update_logic: F)
    where
        F: 'static + FnMut(&mut AppContext),
    {
        let mut cursor_pos = (0, 0);
        let event_loop = self.event_loop.take().unwrap();

        let window = self.window;
        let mut pixels = self.pixels;
        let mut framebuffer = self.framebuffer;

        let mut context = AppContext {
            widgets: self.widgets,
            clicked_id: None,
            active_focus: None,
            last_typed_char: None,
        };

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Wait;

            // Reset frame-specific event states
            context.clicked_id = None;
            context.last_typed_char = None;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,

                    WindowEvent::Resized(size) => {
                        let _ = pixels.resize_surface(size.width, size.height);
                        let _ = pixels.resize_buffer(size.width, size.height);
                        framebuffer = RgbaImage::new(size.width, size.height);
                        window.request_redraw();
                    }

                    WindowEvent::CursorMoved { position, .. } => {
                        if let Ok(mapped_pos) =
                            pixels.window_pos_to_pixel((position.x as f32, position.y as f32))
                        {
                            cursor_pos = (mapped_pos.0 as i32, mapped_pos.1 as i32);
                        }
                    }

                    WindowEvent::MouseInput { state, button, .. } => {
                        if state == ElementState::Pressed && button == MouseButton::Left {
                            for widget in &mut context.widgets {
                                if let Some(id) = widget.handle_click(cursor_pos.0, cursor_pos.1) {
                                    context.clicked_id = Some(id);
                                    break;
                                }
                            }

                            update_logic(&mut context);
                            window.request_redraw();
                        }
                    }

                    WindowEvent::ReceivedCharacter(c) => {
                        // Ignore control characters (like backspace, which is handled below)
                        if !c.is_control() {
                            context.last_typed_char = Some(TypedEvent::Char(c));
                            update_logic(&mut context);
                            window.request_redraw();
                        }
                    }

                    WindowEvent::KeyboardInput {
                        input:
                            KeyboardInput {
                                state: ElementState::Pressed,
                                virtual_keycode: Some(VirtualKeyCode::Back),
                                ..
                            },
                        ..
                    } => {
                        context.last_typed_char = Some(TypedEvent::Backspace);
                        update_logic(&mut context);
                        window.request_redraw();
                    }

                    _ => {}
                },

                Event::RedrawRequested(_) => {
                    framebuffer
                        .pixels_mut()
                        .for_each(|p| *p = Rgba([255, 255, 255, 255]));

                    for widget in &mut context.widgets {
                        widget.draw(&mut framebuffer);
                    }

                    pixels
                        .frame_mut()
                        .copy_from_slice(&framebuffer.clone().into_raw());
                    pixels.render().unwrap();
                }
                _ => {}
            }
        });
    }
}
