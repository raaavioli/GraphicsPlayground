mod shader;
mod data;
mod gl_buffers;
mod window;
mod texture;
mod camera;
mod event;

pub use crate::resources::{Resources};
pub use shader::{ShaderProgram};
pub use gl_buffers::*;
pub use window::Window;
pub use texture::Texture;
pub use event::{EventState, KeyCode};
use camera::Camera;

use std::rc::Rc;
use std::mem::size_of;
use std::path::Path;

use nalgebra::{Vector3};

use glutin::{dpi::{PhysicalPosition}, event::{ElementState, Event, WindowEvent}};
use glutin::event_loop::{ControlFlow, EventLoop};

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct PosColorUV {
    #[location = 0]
    position: data::Float32_32_32,
    #[location = 1]
    color: data::Float32_32_32_32,
    #[location = 2]
    uv: data::Float32_32,
}

#[derive(VertexAttribPointers, Copy, Clone, Debug)]
#[repr(C, packed)]
struct PosUV {
    #[location = 0]
    position: data::Float32_32_32,
    #[location = 1]
    uv: data::Float32_32,
}


pub fn run() -> Result<(), failure::Error> {
    let width = 1920;
    let height = 1080;
    let events_loop = EventLoop::new();
    let window = Window::from_size(width, height, &events_loop)?;
    let gl= window.get_gl_handle();
    let mut event_state = EventState::new();

    unsafe {
        gl.Enable(gl::BLEND);
        gl.BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
    }
    
    let mut camera = Camera::new(*Vector3::z_axis() * 10., Vector3::zeros(), width as f32 / height as f32, 45.0, 0.01, 1000.0, true);

    let resource = Resources::from_relative_exe_path(Path::new("assets")).unwrap();
    let shader_program = ShaderProgram::from_resource(Rc::clone(&gl), &resource, "shaders/test")?;
    let square_program = ShaderProgram::from_resource(Rc::clone(&gl), &resource, "shaders/square_texture")?;
    
    let vertices = [
        PosColorUV { position: (-0.5, -0.5, 0.0).into(), color: (1.0, 0.0, 0.0, 1.0).into(), uv: (0.0, 0.0).into()},
        PosColorUV { position: (0.5, -0.5, 0.0).into(), color: (1.0, 1.0, 0.0, 1.0).into(), uv: (1.0, 0.0).into()},
        PosColorUV { position: (-0.5, 0.5, 0.0).into(), color: (0.0, 1.0, 0.0, 1.0).into(), uv: (0.0, 1.0).into()},
        PosColorUV { position: (0.5, 0.5, 0.0).into(), color: (0.0, 1.0, 1.0, 1.0).into(), uv: (1.0, 1.0).into()},
    ];

    let frame_vertices = [
        PosUV { position: (-0.95, -0.95, 0.0).into(), uv: (0.0, 0.0).into()},
        PosUV { position: (0.95, -0.95, 0.0).into(), uv: (1.0, 0.0).into()},
        PosUV { position: (-0.95, 0.95, 0.0).into(), uv: (0.0, 1.0).into()},
        PosUV { position: (0.95, 0.95, 0.0).into(), uv: (1.0, 1.0).into()},
    ];

    let indices: [u8; 6] = [
        1, 0, 2, 2, 3, 1,
    ];

    // Smiley image square setup
    let v_buffer = ArrayBuffer::new(Rc::clone(&gl));
    v_buffer.bind();
    v_buffer.buffer_static_draw(&vertices);
    v_buffer.unbind();
    
    let i_buffer= ElementArrayBuffer::new(Rc::clone(&gl));
    i_buffer.bind();
    i_buffer.buffer_static_draw(&indices);
    i_buffer.unbind();
    
    let v_array = VertexArray::new(Rc::clone(&gl));
    v_array.bind();
    v_buffer.bind();
    i_buffer.bind();
    PosColorUV::vertex_attrib_pointers(&gl);
    v_array.unbind();

    let texture = Texture::from_resource(Rc::clone(&gl), &resource, "smiley.png")?;

    // Frame buffer + buffers setup
    let frame_buffer = FrameBuffer::new(Rc::clone(&gl), width, height);
    let frame_v_buffer = ArrayBuffer::new(Rc::clone(&gl));
    frame_v_buffer.bind();
    frame_v_buffer.buffer_static_draw(&frame_vertices);
    frame_v_buffer.unbind();
    
    let frame_v_array = VertexArray::new(Rc::clone(&gl));
    frame_v_array.bind();
    frame_v_buffer.bind();
    i_buffer.bind();
    PosUV::vertex_attrib_pointers(&gl);
    frame_v_array.unbind();

    let perspective_loc = shader_program.get_uniform_location("Perspective").unwrap();
    let view_loc = shader_program.get_uniform_location("View").unwrap();
    let mut left_pressed: bool = false;
    let mut prev_mouse: PhysicalPosition<f64> = PhysicalPosition::new(0., 0.);
    let default_mouse = prev_mouse;

    events_loop.run(move |event, _, control_flow| {
        #[allow(deprecated)]
        match event {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::Resized(physical_size) => {
                    let w = physical_size.width;
                    let h = physical_size.height;
                    window.resize(w, h);
                    camera.set_aspect_ratio(w, h);
                    frame_buffer.resize(w, h);
                },
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { device_id: _, input, is_synthetic: _} => {
                    event_state.process_event(input.virtual_keycode.unwrap().into(), input.state);
                },
                #[allow(deprecated)]
                WindowEvent::MouseInput { device_id: _, button, state, modifiers: _} => {
                    event_state.process_event(button.into(), state);
                    if let KeyCode::MouseLeft = button.into() {
                        if state == ElementState::Pressed {
                            left_pressed = true;
                        } else {
                            prev_mouse = default_mouse;
                            left_pressed = false;
                        }
                    }
                },
                WindowEvent::CursorMoved { device_id: _, position, modifiers: _ } => {
                    if left_pressed {
                        if prev_mouse == default_mouse {
                            prev_mouse = position;
                        } else {
                            let dx_yaw = ((position.x - prev_mouse.x) / 100.) as f32;
                            let dy_pitch = ((position.y - prev_mouse.y) / 100.) as f32;
                            camera.rotate_camera(Vector3::new(dx_yaw, dy_pitch, 0.));
                            prev_mouse = position;
                        }
                    }
                },
                _ => (),
            },
            Event::RedrawRequested(_) => {
                if event_state.is_pressed(&KeyCode::A) {
                    camera.move_camera(-*Vector3::x_axis(), 0.1);
                }
                if event_state.is_pressed(&KeyCode::D) {
                    camera.move_camera(*Vector3::x_axis(), 0.1);
                }
                if event_state.is_pressed(&KeyCode::W) {
                    camera.move_camera(-*Vector3::z_axis(), 0.1);
                }
                if event_state.is_pressed(&KeyCode::S) {
                    camera.move_camera(*Vector3::z_axis(), 0.1);
                }

                window.set_clear_color(Vector3::new(0.8, 0.8, 0.8));
                v_array.bind();
                texture.bind();
                shader_program.bind();
                shader_program.bind_uniform_mat4(view_loc, &camera.get_view_matrix());
                shader_program.bind_uniform_mat4(perspective_loc, &camera.get_projection_matrix());
                frame_buffer.bind();
                unsafe {
                    gl.Clear(gl::COLOR_BUFFER_BIT);
                    gl.DrawElements(gl::TRIANGLES, indices.len() as i32, gl::UNSIGNED_BYTE, std::ptr::null());
                }
                frame_buffer.unbind();

                window.set_clear_color(Vector3::new(0.3, 0.3, 0.3));
                frame_v_array.bind();
                frame_buffer.bind_texture();
                square_program.bind();
                unsafe {
                    gl.Clear(gl::COLOR_BUFFER_BIT);
                    gl.DrawElements(gl::TRIANGLES, indices.len() as i32, gl::UNSIGNED_BYTE, std::ptr::null());
                }        

                window.swap_buffers();
                window.request_redraw();
            },
            _ => (),
        }
    });
}