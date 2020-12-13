
use gl::types::*;

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Float32_32_32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Float32_32_32 {
    pub fn new(x: f32, y: f32, z: f32) ->  Self {
        Self{ x, y, z }
    }

    pub unsafe fn vertex_attrib_pointer(gl: &gl::Gl, location: usize, stride: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            3,
            gl::FLOAT,
            gl::FALSE,
            stride as GLsizei,
            offset as *const GLvoid,
        );
    }
}

impl From<(f32, f32, f32)> for Float32_32_32 {
    fn from(other: (f32, f32, f32)) -> Self {
        Float32_32_32::new(other.0, other.1, other.2)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Float32_32_32_32 {
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub w: f32,
}

impl Float32_32_32_32 {
    pub fn new(x: f32, y: f32, z: f32, w: f32) ->  Self {
        Self{ x, y, z, w }
    }

    pub unsafe fn vertex_attrib_pointer(gl: &gl::Gl, location: usize, stride: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            4,
            gl::FLOAT,
            gl::FALSE,
            stride as GLsizei,
            offset as *const GLvoid,
        );
    }
}

impl From<(f32, f32, f32, f32)> for Float32_32_32_32 {
    fn from(other: (f32, f32, f32, f32)) -> Self {
        Float32_32_32_32::new(other.0, other.1, other.2, other.3)
    }
}

#[derive(Copy, Clone, Debug)]
#[repr(C, packed)]
pub struct Float32_32 {
    pub x: f32,
    pub y: f32,
}

impl Float32_32 {
    pub fn new(x: f32, y: f32) ->  Self {
        Self{ x, y }
    }

    pub unsafe fn vertex_attrib_pointer(gl: &gl::Gl, location: usize, stride: usize, offset: usize) {
        gl.EnableVertexAttribArray(location as gl::types::GLuint);
        gl.VertexAttribPointer(
            location as gl::types::GLuint,
            2,
            gl::FLOAT,
            gl::FALSE,
            stride as GLsizei,
            offset as *const GLvoid,
        );
    }
}

impl From<(f32, f32)> for Float32_32 {
    fn from(other: (f32, f32)) -> Self {
        Float32_32::new(other.0, other.1)
    }
}