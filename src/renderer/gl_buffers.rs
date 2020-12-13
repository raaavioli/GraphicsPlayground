use std::rc::Rc;
use gl::types::*;
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
    gl: gl::Gl,
    vao: gl::types::GLuint,
}

impl VertexArray {
    pub fn new(gl: &gl::Gl) -> VertexArray {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl.GenVertexArrays(1, &mut vao);
        }

        VertexArray {
            gl: gl.clone(),
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