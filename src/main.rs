extern crate glfw;
extern crate gl_loader;
extern crate gl;

use glfw::{Action, Context, Key};
use core::mem::size_of_val;
use learn_opengl::shader::Shader;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(800, 600, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    
    load_gl_symbol();

    let mut vbo: u32 = 0;
    let mut vao: u32 = 0;
    let mut ebo: u32 = 0;
    let shader;
    type Vertex = [f32; 6];
    type TriIndexes = [u32; 3];
    const VERTICES: [Vertex; 4] = [
        [ 0.5,  0.5, 0.0, 1.0, 0.0, 0.0], 
        [ 0.5, -0.5, 0.0, 0.0, 1.0, 0.0], 
        [-0.5, -0.5, 0.0, 0.0, 0.0, 1.0],
        [-0.5,  0.5, 0.0, 1.0, 0.0, 1.0]
    ];
    const INDICES: [TriIndexes; 2] = [
        [0, 1, 3], 
        [1, 2, 3]
    ];

    window.set_key_polling(true);
    window.make_current();

    unsafe {
        shader = Shader::new("src/shaders/vertex.glsl", "src/shaders/fragment.glsl");
        // Set clear color
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);

        // Generate vertex array object
        gl::GenVertexArrays(1, &mut vao);
        gl::BindVertexArray(vao);

        // Generate vertex buffer object (VBO)
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            size_of_val(&VERTICES) as isize, 
            VERTICES.as_ptr().cast(), 
            gl::STATIC_DRAW
        );
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER, 
            size_of_val(&INDICES) as isize, 
            INDICES.as_ptr().cast(), 
            gl::STATIC_DRAW
        ); 

        // Vertex possition array
        gl::VertexAttribPointer(
            0, 
            3, 
            gl::FLOAT, 
            gl::FALSE, 
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, 
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);

        // Color position array
        gl::VertexAttribPointer(
            1, 
            3, 
            gl::FLOAT, 
            gl::FALSE, 
            (6 * std::mem::size_of::<f32>()) as gl::types::GLint, 
            (3 * std::mem::size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);
    }

    while !window.should_close() {
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event);
        }
        unsafe {
            let time_value = glfw::ffi::glfwGetTime();
            let green_value = ((time_value.sin() / 2.0) + 0.5) as f32;
            use std::ffi::CString;
            let uniform_name = CString::new("ourColor").expect("Convert to c-string");
            let vertex_color_location = gl::GetUniformLocation(shader.id, uniform_name.as_ptr());
            gl::Clear(gl::COLOR_BUFFER_BIT);
            shader.use_shader();
            gl::Uniform4f(vertex_color_location, 0.0, green_value, 0.0, 1.0);
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
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
