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

pub trait UniformValue {
    fn set(&self, program: u32, name: &str);
}

impl UniformValue for i32 {
    fn set(&self, program: u32, name: &str) {
        let c_str = std::ffi::CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(program, c_str.as_ptr());
            gl::Uniform1i(location, *self);
        }
    }
}

impl UniformValue for [i32] {
    fn set(&self, program: u32, name: &str) {
        let c_str = std::ffi::CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(program, c_str.as_ptr());
            gl::Uniform1iv(location, self.len() as i32, self.as_ptr());
        }
    }
}

impl UniformValue for Mat4 {
    fn set(&self, program: u32, name: &str) {
        let c_str = std::ffi::CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(program, c_str.as_ptr());
            gl::UniformMatrix4fv(location, 1, gl::FALSE, self.to_cols_array().as_ptr());
        }
    }
}

impl UniformValue for Vec4 {
    fn set(&self, program: u32, name: &str) {
        let c_str = std::ffi::CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(program, c_str.as_ptr());
            gl::Uniform4f(location, self.x, self.y, self.z, self.w);
        }
    }
}

impl UniformValue for [f32] {
    fn set(&self, program: u32, name: &str) {
        let c_str = std::ffi::CString::new(name).unwrap();
        unsafe {
            let location = gl::GetUniformLocation(program, c_str.as_ptr());
            gl::Uniform1fv(location, self.len() as i32, self.as_ptr());
        }
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

    pub fn set_uniform<T: UniformValue>(&self, name: &str, value: T) {
        value.set(self.0, name);
    }

    pub fn set_uniform_ref<T: UniformValue + ?Sized>(&self, name: &str, value: &T) {
        value.set(self.0, name);
    }

    pub fn use_program(&self) {
        unsafe {
            gl::UseProgram(self.0);
        }
    }
}
