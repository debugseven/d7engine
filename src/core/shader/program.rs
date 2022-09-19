use crate::prelude::*;
use gl::types::*;

// the ShaderProgram will be the 
// compiled vertex and fragment shader
// it holds an id which is generated by opengl
// the id points to a location on the graphics card
pub struct Program {
    pub id: GLuint,
}

impl Program {
    // creates a ShaderProgram from the 2 
    // shader types
    pub fn new(vertex_shader: &Shader, fragment_shader: &Shader) -> Result<Self, String> {
        unsafe {
            let program = Self {
                id: gl::CreateProgram(),
            };

            // attach the shaders to the program and link them together 
            // under this structs id
            gl::AttachShader(program.id, vertex_shader.id);
            gl::AttachShader(program.id, fragment_shader.id);
            gl::LinkProgram(program.id);

            // check error status
            let mut success: GLint = 0;
            gl::GetProgramiv(program.id, gl::LINK_STATUS, &mut success);

            if success != 1 {
                // get the error log message
                // and throw the error
                let mut error_log_size: GLint = 0;
                gl::GetProgramiv(program.id, gl::INFO_LOG_LENGTH, &mut error_log_size);
                let mut error_log: Vec<u8> = Vec::with_capacity(error_log_size as usize);
                gl::GetProgramInfoLog(
                    program.id,
                    error_log_size,
                    &mut error_log_size,
                    error_log.as_mut_ptr() as *mut _,
                );

                error_log.set_len(error_log_size as usize);
                match String::from_utf8(error_log) {
                    Ok(log) => return Err(log),
                    _ => return Err("Could not create String from utf8.".to_string()),
                }
            }

            Ok(program)
        }
    }

    // this will set this shaderprogram active
    // so all opengl draw functions will use this
    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    // get the location of an attribute in the vertex shader 
    pub fn get_attribute_location(&self, attribute: &str) -> Result<GLint, String> {
        unsafe {
            if let Ok(attribute) = CString::new(attribute) {
                Ok(gl::GetAttribLocation(self.id, attribute.as_ptr()))
            } else {
                Err("Could not get Attribute Location.".to_string())
            }
        }
    }

    // get the location of a uniform in the vertex shader
    pub fn get_uniform_location(&self, uniform: &str) -> Result<GLint, String> {
        unsafe {
            if let Ok(attribute) = CString::new(uniform) {
                Ok(gl::GetUniformLocation(self.id, attribute.as_ptr()))
            } else {
                Err("Could not get Uniform Location.".to_string())
            }
        }
    }
}

impl Default for Program {
    // implements an empty Program
    fn default() -> Self {
        Self {
            id: 0,
        }
    }
}

// delete the program on the graphics card
// when the ShaderProgram is dropped
impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.id);
        }
    }
}