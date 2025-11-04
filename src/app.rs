use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};
use image::RgbaImage;
use crate::widget::Widget;
use crate::widget::button::Button;

pub struct App {
    pub pixels: Pixels,
    pub window: Window,
    pub event_loop: EventLoop<()>,
    pub width: u32,
    pub height: u32,
}

impl App {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let event_loop = EventLoop::new();
        let window = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(winit::dpi::LogicalSize::new(width as f64, height as f64))
            .build(&event_loop)
            .unwrap();

        let surface = SurfaceTexture::new(width, height, &window);
        let pixels = Pixels::new(width, height, surface).unwrap();

        Self {
            pixels,
            window,
            event_loop,
            width,
            height,
        }
    }

    pub fn run(mut self, mut widgets: Vec<Box<dyn Widget>>) {
        let mut cursor_pos = (0, 0);

        self.event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(size) => {
                        self.pixels.resize_surface(size.width, size.height);
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        cursor_pos = (position.x as i32, position.y as i32);
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        if state == ElementState::Pressed && button == MouseButton::Left {
                            handle_click(cursor_pos.0, cursor_pos.1, &mut widgets);
                        }
                    }
                    _ => {}
                },
                Event::RedrawRequested(_) => {
                    let mut image = RgbaImage::new(self.width, self.height);
                    for pixel in image.pixels_mut() {
                        *pixel = image::Rgba([255, 255, 255, 255]);
                    }

                    for widget in &widgets {
                        widget.draw(&mut image);
                    }

                    self.pixels.frame_mut().copy_from_slice(&image.into_raw());
                    self.pixels.render().unwrap();
                }
                Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                _ => {}
            }
        });
    }
}

use winit::event::{MouseButton, ElementState};

fn handle_click(x: i32, y: i32, widgets: &mut [Box<dyn Widget>]) {
    for widget in widgets {
        if let Some(button) = widget.as_any_mut().downcast_mut::<Button>() {
            let r = button.rect;
            if x >= r.left() && x <= r.right() && y >= r.top() && y <= r.bottom() {
                if let Some(callback) = &mut button.on_click {
                    callback();
                }
            }
        }
    }
}
