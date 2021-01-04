
use std::{rc::Rc};

use glutin::{ContextWrapper, PossiblyCurrent, dpi::PhysicalSize};
use glutin::event_loop::{EventLoop};
use glutin::window::WindowBuilder;
use nalgebra::{Vector3};

use gl::types::*;

pub struct Window {
    gl: Rc<gl::Gl>,
    window_context: ContextWrapper<PossiblyCurrent, glutin::window::Window>,
}

impl Window {
    pub fn from_size(width: u32, height: u32, events_loop: &EventLoop<()>) -> Result<Window, failure::Error> {
        let window_builder = WindowBuilder::new()
            .with_title("Playground")
            .with_inner_size(PhysicalSize::new(width, height));
        
        let window_context: ContextWrapper<PossiblyCurrent, _>;
        unsafe {
            window_context = glutin::ContextBuilder::new()
                .with_vsync(true)
                .build_windowed(window_builder, events_loop).unwrap()
                .make_current().unwrap();
        }

        let gl = Rc::new(gl::Gl::load_with(|s| {
                window_context.get_proc_address(s) as *const _
            }));

        Ok(Window {
            gl,
            window_context,
        })
    }

    pub fn get_gl_handle(&self) -> Rc<gl::Gl> {
        Rc::clone(&self.gl)
    }

    pub fn request_redraw(&self) {
        self.window_context.window().request_redraw();
    }

    pub fn resize (&self, width: u32, height: u32){
        self.window_context.resize(PhysicalSize::new(width, height));
        unsafe {
            self.gl.Viewport(0, 0, width as i32, height as i32);
        }
    }
    
    pub fn set_clear_color(&self, color: Vector3<f32>) {
        unsafe {
            self.gl.ClearColor(color.x, color.y, color.z, 1.0);
        }
    }
    
    pub fn swap_buffers(&self) {
        self.window_context.swap_buffers().unwrap();
    }
}


