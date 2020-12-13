use gl::types::*;
use std::ffi::CString;
use std::ptr::*;

use crate::resources::{Resources, Error};

pub struct Texture {
    gl: std::rc::Rc<gl::Gl>,
    id: gl::types::GLuint,
}

impl Texture {
    pub fn from_resource(gl: std::rc::Rc<gl::Gl>, res: &Resources, name: &str) -> Result<Self, Error> {
        let texture_file = res.load_cstring(name)?;
        //GL CREATE TEXTURE ETC ETC.
    }
}