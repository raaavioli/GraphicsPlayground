use gl::types::*;
use std::ffi::CString;
use std::ptr::*;
use std::rc::Rc;

use crate::resources;
use crate::resources::Resources;

#[derive(Debug, Fail)]
pub enum Error {
    #[fail(display = "Failed to load resource {}", name)]
    ResourceLoad { name: String, #[cause] inner: resources::Error },
    #[fail(display = "Can not determine shader type for resource {}", name)]
    CanNotDetermineShaderTypeForResource { name: String },
    #[fail(display = "Failed to compile shader {}: {}", name, message)]
    CompileError { name: String, message: String },
    #[fail(display = "Failed to link program {}: {}", name, message)]
    LinkError { name: String, message: String },
}

pub struct ShaderProgram {
    gl: Rc<gl::Gl>,
    id: GLuint,
}

impl ShaderProgram {
    pub fn from_resource(gl: Rc<gl::Gl>, res: &Resources, name: &str) -> Result<Self, Error> {
        const POSSIBLE_EXT: [&str; 2] = [".vert", ".frag"];

        let shaders = POSSIBLE_EXT
            .iter()
            .map(|file_extension| {
                Shader::from_resource(gl, res, &format!("{}{}", name, file_extension))
            })
            .collect::<Result<Vec<Shader>, Error>>()?;

        Self::from_shaders(gl, &shaders[..], name)
    }

    fn from_shaders(gl: Rc<gl::Gl>, shaders: &[Shader], name: &str) -> Result<Self, Error> {
        let id = unsafe { gl.CreateProgram() };

        for shader in shaders {
            unsafe { gl.AttachShader(id, shader.id) }
        }

        unsafe { gl.LinkProgram(id) }

        for shader in shaders {
            unsafe { gl.DetachShader(id, shader.id) }
        }

        let mut success: gl::types::GLint = 1;
        unsafe { gl.GetProgramiv(id, gl::LINK_STATUS, &mut success) }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe { gl.GetProgramiv(id, gl::INFO_LOG_LENGTH, &mut len) }

            let error = create_blank_cstring(len as usize);

            unsafe { gl.GetProgramInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar) }

            return Err(Error::LinkError{name: name.into(), message: error.to_string_lossy().into_owned()});
        }

        Ok(Self { gl, id })
    }

    pub fn bind(&self) {
        unsafe { self.gl.UseProgram(self.id) }
    }
}

impl Drop for ShaderProgram {
    fn drop(&mut self) {
        unsafe { self.gl.DeleteProgram(self.id) }
    }
}

struct Shader {
    gl: Rc<gl::Gl>,
    id: GLuint,
}

impl Shader {
    fn from_resource(gl: Rc<gl::Gl>, res: &Resources, name: &str) -> Result<Self, Error> {
        const POSSIBLE_EXT: [(&str, gl::types::GLenum); 2] =
            [(".vert", gl::VERTEX_SHADER), (".frag", gl::FRAGMENT_SHADER)];

        let shader_type = POSSIBLE_EXT
            .iter()
            .find(|&&(file_extension, _)| name.ends_with(file_extension))
            .map(|&(_, s_type)| s_type)
            .ok_or_else(|| Error::CanNotDetermineShaderTypeForResource{ name: name.into() })?;

        let source = res
            .load_cstring(name)
            .map_err(|e| Error::ResourceLoad{name: name.into(), inner: e})?;

        Self::from_source(gl, &source, shader_type, name)
    }

    fn from_source(gl: Rc<gl::Gl>, src_code: &CString, shader_type: GLenum, name: &str) -> Result<Self, Error> {
        let id = unsafe { gl.CreateShader(shader_type) };
        unsafe {
            gl.ShaderSource(
                id,
                1, // Shader count
                &(src_code.as_ptr() as *const GLchar),
                null(),
            );
            gl.CompileShader(id);
        }
        let mut success: GLint = 1;
        unsafe {
            gl.GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
        }
        if success == 0 {
            let mut len: GLint = 0;
            unsafe {
                gl.GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_blank_cstring(len as usize);
            unsafe {
                gl.GetShaderInfoLog(id, len, null_mut(), error.as_ptr() as *mut GLchar);
            }
            return Err(Error::CompileError{name: name.into(), message: error.to_string_lossy().into_owned()});
        }

        Ok(Self { gl, id, })
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteShader(self.id);
        }
    }
}

fn create_blank_cstring(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer) }
}
