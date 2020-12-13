mod shader;
mod data;
mod gl_buffers;
mod window;
mod texture;

pub use crate::resources::{Resources};
pub use self::shader::{ShaderProgram};
pub use self::gl_buffers::*;
pub use self::window::Window;


use std::rc::Rc;
use std::mem::size_of;
use std::path::Path;

use glm::*;
pub use gl::types::*;

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct Vertex {
    #[location = 0]
    position: data::Float32_32_32,
    #[location = 1]
    color: data::Float32_32_32_32,
    #[location = 2]
    uv: data::Float32_32,
}


pub fn run() -> Result<(), failure::Error> {

    let window = Window::from_size(1280, 720)?;
    let gl= window.get_gl_handle();
    //window_setup(&gl, vec3(0.5, 0.8, 1.), Vector2::new(width, height));
    enable_default_blend(Rc::clone(&gl));

    
    let resource = Resources::from_relative_exe_path(Path::new("assets")).unwrap();
    let shader_program = ShaderProgram::from_resource(Rc::clone(&gl), &resource, "shaders/test")?;
    shader_program.bind();
    
    let vertices = [
        Vertex { position: (-0.5, -0.5, 0.0).into(), color: (1.0, 0.0, 0.0, 1.0).into(), uv: (0.0, 0.0).into()},
        Vertex { position: (0.5, -0.5, 0.0).into(), color: (1.0, 1.0, 0.0, 1.0).into(), uv: (1.0, 0.0).into()},
        Vertex { position: (-0.5, 0.5, 0.0).into(), color: (0.0, 1.0, 0.0, 1.0).into(), uv: (0.0, 1.0).into()},
        Vertex { position: (0.5, 0.5, 0.0).into(), color: (0.0, 1.0, 1.0, 1.0).into(), uv: (1.0, 1.0).into()},
    ];

    let indices: [u8; 6] = [
        1, 0, 2, 2, 3, 1,
    ];

    let v_buffer = ArrayBuffer::new(Rc::clone(&gl));
    v_buffer.bind();
    v_buffer.buffer_static_draw(&vertices);
    v_buffer.unbind();
    
    let i_buffer= ElementArrayBuffer::new(Rc::clone(&gl));
    i_buffer.bind();
    i_buffer.buffer_static_draw(&indices);
    i_buffer.unbind();
    
    let v_array = VertexArray::new(&gl);
    v_array.bind();
    v_buffer.bind();
    i_buffer.bind();
    Vertex::vertex_attrib_pointers(&gl);
    v_array.unbind();

    let texture = Texture::from_resource();

    while !window.should_close() {
        window.handle_events();
        window.set_clear_color(Vector3::<f32>::new(0.3, 0.3, 0.3));
        v_array.bind();
        unsafe {
            gl.Clear(gl::COLOR_BUFFER_BIT);
            gl.DrawElements(gl::TRIANGLES, indices.len() as i32, gl::UNSIGNED_BYTE, std::ptr::null());
        }

        window.swap_buffers();
    } 
    Ok(()) 
}

fn enable_default_blend(gl: Rc<gl::Gl>) {
    unsafe {
        gl.Enable(gl::BLEND);
        gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
} 