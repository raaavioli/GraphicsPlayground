
use std::rc::Rc;

use glfw::{Action, Context, Key, WindowEvent};
use glm::*;

use gl::types::*;

pub struct Window {
    gl: Rc<gl::Gl>,
    glfw_window: std::cell::RefCell<glfw::Window>,
    event_receiver: ::std::sync::mpsc::Receiver<(f64, WindowEvent)>,
}

impl Window {
    pub fn from_size(width: usize, height: usize) -> Result<Window, failure::Error> {
        let glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

        let (mut glfw_window, event_receiver) = glfw
        .create_window(
            width as u32,
            height as u32,
            "Playground",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");
        
        glfw_window.set_key_polling(true);
        glfw_window.make_current();
        
        let gl = Rc::new(gl::Gl::load_with(|s| {
            glfw_window.get_proc_address(s) as *const _
        }));

        Ok(Window {
            gl,
            glfw_window: glfw_window.into(),
            event_receiver,
        })
    }

    pub fn get_gl_handle(&self) -> Rc<gl::Gl> {
        Rc::clone(&self.gl)
    }

    pub fn set_viewport (&self, window_size: Vector2<GLsizei>){
        unsafe {
            self.gl.Viewport(0, 0, window_size.x, window_size.y);
        }
    }
    
    pub fn set_clear_color(&self, color: Vector3<f32>) {
        unsafe {
            self.gl.ClearColor(color.x, color.y, color.z, 1.0);
        }
    }

    pub fn should_close(&self) -> bool {
        self.glfw_window.borrow_mut().should_close()
    }
    
    pub fn swap_buffers(&self) {
        self.glfw_window.borrow_mut().swap_buffers();
    }

    pub fn handle_events(&self) {
        self.glfw_window.borrow_mut().glfw.poll_events();
        for (_, event) in glfw::flush_messages(&self.event_receiver) {
            handle_window_event(&mut self.glfw_window.borrow_mut(), event);
        }
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => window.set_should_close(true),
        _ => {}
    }
}


