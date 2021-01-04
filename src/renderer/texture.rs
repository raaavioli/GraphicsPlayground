use std::{ffi::c_void, rc::Rc};
use gl::types::*;

use crate::resources::{Resources, Error};

pub struct Texture {
    gl: Rc<gl::Gl>,
    id: GLuint,
    width: u32,
    height: u32,
    img_ptr: *const c_void, 
}

impl Texture {
    pub fn empty(gl: Rc<gl::Gl>, width: u32, height: u32) -> Self {
        let mut id: GLuint = 0;
        let img_ptr: *const c_void = std::ptr::null();
        unsafe {
            gl.GenTextures(1, &mut id);
            gl.BindTexture(gl::TEXTURE_2D, id);

            gl.TexImage2D(gl::TEXTURE_2D, 0, gl::RGB8 as GLint, width as i32, height as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, img_ptr);
    
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl.TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32); 
    
            gl.BindTexture(gl::TEXTURE_2D, 0);
        }

        Texture {
            gl,
            id, 
            width, 
            height,
            img_ptr,
        }
    }

    pub fn from_resource(gl: Rc<gl::Gl>, res: &Resources, name: &str) -> Result<Self, Error> {
        let mut id: GLuint = 0;
        unsafe { gl.GenTextures(1, &mut id); };
        let img = res.load_rgb_image(name)?;
        
        let texture = Texture{ gl: Rc::clone(&gl), id, width: img.width(), height: img.height(), img_ptr: img.as_ptr() as _, };
        
        texture.bind();
        unsafe {
            gl.TexImage2D(
                gl::TEXTURE_2D, 0, gl::RGB8 as GLint,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGB,
                gl::UNSIGNED_BYTE,
                img.as_ptr() as _,
            );
            gl.GenerateMipmap(gl::TEXTURE_2D);
        }      
        texture.unbind();
        Ok(texture)
    }

    pub fn bind(&self) {
        unsafe {
            self.gl.BindTexture(gl::TEXTURE_2D, self.id);
        }
    }

    pub fn unbind(&self) {
        unsafe {
            self.gl.BindTexture(gl::TEXTURE_2D, 0);
        }
    }

    pub fn resize(&self,  width: u32, height: u32) {
        self.bind();
        unsafe {
            self.gl.TexImage2D(gl::TEXTURE_2D, 0, gl::RGB8 as GLint, width as i32, height as i32, 0, gl::RGB, gl::UNSIGNED_BYTE, self.img_ptr);
        }
        self.bind();
    }

    pub fn get_id(&self) -> GLuint {
        self.id
    }

    pub fn get_size(&self) -> (u32, u32) {
        (self.width, self.height)
    }
}

impl Drop for Texture {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteTextures(1, &self.id);
        }
    }
}