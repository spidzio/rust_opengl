extern crate glfw;
extern crate gl_loader;
extern crate gl;

use glfw::{Action, Context, Key};
use core::mem::size_of_val;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(800, 600, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    
    load_gl_symbol();

    let mut vbo: u32 = 0;
    type Vertex = [f32; 3];
    const VERTICES: [Vertex; 3] = [
        [-0.5, -0.5, 0.0], 
        [0.5, -0.5, 0.0], 
        [0.0, 0.5, 0.0]
    ];

    const VERTEX_SHADER: &str = r#"#version 410 core
layout (location = 0) in vec3 pos;
void main() {
    gl_Position = vec4(pos.x, pos.y, pos.z, 1.0);
}
"#;

    const FRAGMENT_SHADER: &str = r#"#version 410 core
out vec4 FragColor;
    
void main()
{
    FragColor = vec4(1.0f, 0.5f, 0.2f, 1.0f);
} 
"#;


    window.set_key_polling(true);
    window.make_current();

    if gl::Viewport::is_loaded() {
        unsafe {
            // Set viewport
            gl::Viewport(0, 0, 800, 600);

            // Set clear color
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);

            // Generate vertex buffer object (VBO)
            gl::GenBuffers(1, &mut vbo);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(
                gl::ARRAY_BUFFER, 
                size_of_val(&VERTICES) as isize, 
                VERTICES.as_ptr().cast(), 
                gl::STATIC_DRAW
            );

            // Create and compile vertex shader
            let vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            gl::ShaderSource(
                vertex_shader,
                1,
                &(VERTEX_SHADER.as_bytes().as_ptr().cast()),
                &(VERTEX_SHADER.len().try_into().unwrap()),
            );
            gl::CompileShader(vertex_shader);

            let mut success: i32 = 1;
            gl::GetShaderiv(vertex_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(vertex_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));
            } else {
                println!("Vertex shader compiled");
            }

            // Create and compile fragment shader
            let fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            gl::ShaderSource(
                fragment_shader, 
                1, 
                &(FRAGMENT_SHADER.as_bytes().as_ptr().cast()),
                &(FRAGMENT_SHADER.len().try_into().unwrap()),
            );
            gl::CompileShader(fragment_shader);

            gl::GetShaderiv(fragment_shader, gl::COMPILE_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
            } else {
                println!("Fragment shader compiled");
            }

            // Link shader program
            let shader_program = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragment_shader);
            gl::LinkProgram(shader_program);

            gl::GetShaderiv(shader_program, gl::LINK_STATUS, &mut success);
            if success == 0 {
                let mut v: Vec<u8> = Vec::with_capacity(1024);
                let mut log_len = 0_i32;
                gl::GetShaderInfoLog(fragment_shader, 1024, &mut log_len, v.as_mut_ptr().cast());
                v.set_len(log_len.try_into().unwrap());
                panic!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));
            } else {
                println!("Shader program linked");
            }

            // Use shader program
            gl::UseProgram(shader_program);
            gl::DeleteShader(vertex_shader);
            gl::DeleteShader(fragment_shader);


            gl::VertexAttribPointer(
                0, 
                3, 
                gl::FLOAT, 
                gl::FALSE, 
                (3 * std::mem::size_of::<f32>()) as gl::types::GLint, 
                std::ptr::null(),
            );
            gl::EnableVertexAttribArray(0);  
        }
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.swap_buffers();
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent) {
    println!("{:?}", event);
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        },
        _ => {},
    }
}

fn load_gl_symbol() {
    gl_loader::init_gl();
    gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);
}
