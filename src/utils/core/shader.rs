use glam::*;

pub enum ShaderType {
    Vertex = gl::VERTEX_SHADER as isize,
    Fragment = gl::FRAGMENT_SHADER as isize,
}

pub struct Shader(u32);

impl Shader {
    pub fn new(shader_type: ShaderType, source: &str) -> Self {
        let id = unsafe {
            let shader = gl::CreateShader(shader_type as u32);
            let c_str = std::ffi::CString::new(source).unwrap();
            gl::ShaderSource(shader, 1, &c_str.as_ptr(), std::ptr::null());
            gl::CompileShader(shader);

            let mut success = 0;
            gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v = Vec::<u8>::with_capacity(1024);
                let mut log_len = 0;
                gl::GetShaderInfoLog(
                    shader,
                    v.capacity() as i32,
                    &mut log_len,
                    v.as_mut_ptr().cast(),
                );
                v.set_len(log_len as usize);
                panic!("Shader compilation failed: {}", String::from_utf8_lossy(&v));
            }
            shader
        };

        Shader(id)
    }
}

#[derive(Debug, Clone)]
pub struct ShaderProgram(u32);

impl ShaderProgram {
    pub fn new(vertex_shader_source: &str, fragment_shader_source: &str) -> Self {
        let vertex_shader = Shader::new(ShaderType::Vertex, vertex_shader_source);
        let fragment_shader = Shader::new(ShaderType::Fragment, fragment_shader_source);

        let id = unsafe {
            let program = gl::CreateProgram();
            gl::AttachShader(program, vertex_shader.0);
            gl::AttachShader(program, fragment_shader.0);
            gl::LinkProgram(program);
            gl::DetachShader(program, vertex_shader.0);
            gl::DetachShader(program, fragment_shader.0);
            program
        };

        ShaderProgram(id)
    }

    pub fn set_uniform_u32(&self, name: &str, value: u32) {
        let c_str = std::ffi::CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.0, c_str.as_ptr());
            gl::Uniform1ui(location, value);
        }
    }

    pub fn set_uniform_mat4(&self, name: &str, value: &Mat4) {
        let c_str = std::ffi::CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.0, c_str.as_ptr());
            gl::UniformMatrix4fv(location, 1, gl::FALSE, value.to_cols_array().as_ptr());
        }
    }

    pub fn set_uniform_vec3(&self, name: &str, value: &Vec3) {
        let c_str = std::ffi::CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.0, c_str.as_ptr());
            gl::Uniform3f(location, value.x, value.y, value.z);
        }
    }

    pub fn set_uniform_vec4(&self, name: &str, value: &Vec4) {
        let c_str = std::ffi::CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(self.0, c_str.as_ptr());
            gl::Uniform4f(location, value.x, value.y, value.z, value.w);
        }
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.0);
        }
    }
}
