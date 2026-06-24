use crate::widget::Widget;
use image::{Rgba, RgbaImage};
use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{ElementState, Event, MouseButton, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowBuilder},
};

pub struct App {
    pub pixels: Pixels,
    pub window: Window,
    pub event_loop: EventLoop<()>,
    pub framebuffer: RgbaImage,
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
        let framebuffer = RgbaImage::new(width, height);

        Self {
            pixels,
            window,
            event_loop,
            framebuffer,
        }
    }

    pub fn run(mut self, mut widgets: Vec<Box<dyn Widget>>) {
        let mut cursor_pos = (0, 0);

        self.event_loop.run(move |event, _, control_flow| {
            // 1. Tell winit to sleep and WAIT for an event instead of running constantly
            *control_flow = ControlFlow::Wait;

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                    WindowEvent::Resized(size) => {
                        self.pixels.resize_surface(size.width, size.height);
                        self.window.request_redraw(); // Redraw because window size changed
                    }
                    WindowEvent::CursorMoved { position, .. } => {
                        if let Ok(mapped_pos) = self
                            .pixels
                            .window_pos_to_pixel((position.x as f32, position.y as f32))
                        {
                            cursor_pos = (mapped_pos.0 as i32, mapped_pos.1 as i32);

                            // Optional: If you implement hover states later,
                            // you would request a redraw here!
                        }
                    }
                    WindowEvent::MouseInput { state, button, .. } => {
                        if state == ElementState::Pressed && button == MouseButton::Left {
                            for widget in &mut widgets {
                                widget.handle_click(cursor_pos.0, cursor_pos.1);
                            }

                            self.window.request_redraw();
                        }
                    }
                    _ => {}
                },
                Event::RedrawRequested(_) => {
                    self.framebuffer
                        .pixels_mut()
                        .for_each(|p| *p = image::Rgba([255, 255, 255, 255]));

                    // Draw widgets
                    for widget in &mut widgets {
                        widget.draw(&mut self.framebuffer);
                    }

                    // Render to window
                    self.pixels
                        .frame_mut()
                        .copy_from_slice(&self.framebuffer.clone().into_raw());
                    self.pixels.render().unwrap();
                }
                _ => {}
            }
        });
    }
}
