use gl::types::GLuint;
use std::fs;

pub struct Shader {
    pub id: GLuint,
}

impl Shader {
    pub fn new(vertex_path: &str, fragment_path: &str) -> Self {
        // Load the shaders code
        let vertex_code = fs::read_to_string(vertex_path).unwrap();
        let fragment_code = fs::read_to_string(fragment_path).unwrap();
        let shader_program;

        unsafe {
            // Create GL shaders
            shader_program = gl::CreateProgram();
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            // Compile shaders
            gl::ShaderSource(
                vertex_shader,
                1,
                &(vertex_code.as_bytes().as_ptr().cast()),
                &(vertex_code.len().try_into().unwrap()),
            );
            gl::CompileShader(vertex_shader);
            Self::verify_shader(vertex_shader, gl::COMPILE_STATUS);

            gl::ShaderSource(
                fragment_shader,
                1,
                &(fragment_code.as_bytes().as_ptr().cast()),
                &(fragment_code.len().try_into().unwrap()),
            );
            gl::CompileShader(fragment_shader);
            Self::verify_shader(fragment_shader, gl::COMPILE_STATUS);

            // Link shaderd
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);
            Self::verify_shader(shader_program, gl::LINK_STATUS);

            // Delete shaders
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);
        }

        Self {
            id: shader_program
        }
    }

    pub fn use_shader(&self) {
        unsafe {
            gl::UseProgram(self.id);
        }
    }

    fn verify_shader(shader: GLuint, verify_flag: gl::types::GLenum) {
        let mut success: i32 = 1;

        unsafe {
            gl::GetShaderiv(shader, verify_flag, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Error: {}", String::from_utf8_lossy(&v));
            }
        }
    }
}

