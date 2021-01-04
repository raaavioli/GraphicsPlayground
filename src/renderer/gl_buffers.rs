use std::rc::Rc;
use gl::types::*;

use super::Texture;

pub struct VertexBuffer<B>
where B: BufferType, {
    gl: Rc<gl::Gl>,
    id: GLuint,
    _marker: std::marker::PhantomData<B>
}

impl<B> VertexBuffer<B> 
where B: BufferType, {
    pub fn new(gl: Rc<gl::Gl>) -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl.GenBuffers(1, &mut id);
        }

        VertexBuffer {
            gl,
            id,
            _marker: ::std::marker::PhantomData
        }
    }

    /**
    Binds the buffer

    see: glBindBuffer
    */
    pub fn bind(&self) {
        unsafe {
            self.gl.BindBuffer(B::BUFFER_TYPE, self.id);
        }
    }

    /**
    Unbinds the buffer
    */
    pub fn unbind(&self) {
        unsafe {
            self.gl.BindBuffer(B::BUFFER_TYPE, 0);
        }
    }

    /**
    Submits an array buffer for static draw to the currently bound VertexBuffer
    
    see: glBufferData
    */
    pub fn buffer_static_draw<T>(&self, data: &[T]) {
        unsafe {
            self.gl.BufferData(
                B::BUFFER_TYPE,
                (data.len() * ::std::mem::size_of::<T>()) as GLsizeiptr,
                data.as_ptr() as *const GLvoid,
                gl::STATIC_DRAW,
            );
        }
    }
}

impl<B> Drop for VertexBuffer<B> 
where B: BufferType, {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteBuffers(1, &self.id);
        }
    }
}

pub trait BufferType {
    const BUFFER_TYPE: GLuint;
}

pub struct ArrayBufferType;
impl BufferType for ArrayBufferType {
    const BUFFER_TYPE: GLuint = gl::ARRAY_BUFFER; 
}

pub struct ElementArrayBufferType;
impl BufferType for ElementArrayBufferType {
    const BUFFER_TYPE: GLuint = gl::ELEMENT_ARRAY_BUFFER; 
}

pub type ElementArrayBuffer = VertexBuffer<ElementArrayBufferType>;
pub type ArrayBuffer = VertexBuffer<ArrayBufferType>;

pub struct VertexArray {
    gl: Rc<gl::Gl>,
    vao: gl::types::GLuint,
}

impl VertexArray {
    pub fn new(gl: Rc<gl::Gl>) -> VertexArray {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
        }

        VertexArray {
            gl,
            vao,
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindVertexArray(self.vao);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindVertexArray(0);
        }
    }
}

impl Drop for VertexArray {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteVertexArrays(1, &self.vao);
        }
    }
}

pub struct FrameBuffer {
    gl: Rc<gl::Gl>,
    id: GLuint,
    texture: Texture,
}

impl FrameBuffer {
    pub fn new(gl: Rc<gl::Gl>, width: u32, height: u32) -> Self {
        let mut id: GLuint = 0;
        unsafe {
            gl.GenFramebuffers(1, &mut id);
            gl.BindFramebuffer(gl::FRAMEBUFFER, id);
        }

        let texture: Texture = Texture::empty(Rc::clone(&gl), width, height);

        unsafe {
            gl.FramebufferTexture2D(gl::FRAMEBUFFER, gl::COLOR_ATTACHMENT0, gl::TEXTURE_2D, texture.get_id(), 0);
            if gl.CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                println!("ERROR::FRAMEBUFFER:: Framebuffer is not complete!");
            }
            gl.BindFramebuffer(gl::FRAMEBUFFER, 0); 
        }

        FrameBuffer {
            gl, 
            id,
            texture,
        }
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }

    pub fn resize(&self, width: u32, height: u32) {
        self.texture.resize(width, height);
    }

    pub fn bind_texture(&self) {
        self.texture.bind();
    }
}

impl Drop for FrameBuffer {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteFramebuffers(1, &mut self.id) }
    }
}