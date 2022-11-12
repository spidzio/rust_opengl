extern crate glfw;
extern crate gl_loader;
extern crate gl;

use glfw::{Action, Context, Key};
use core::mem::size_of_val;
use learn_opengl::shader::Shader;
use learn_opengl::camera::{Camera, CameraMovement};
use ultraviolet::mat::Mat4;
use ultraviolet::vec::Vec3;

fn main() {
    let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(4, 1));
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(glfw::OpenGlProfileHint::Core));

    let (mut window, events) = glfw.create_window(800, 600, "Hello this is window", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");
    
    load_gl_symbol();

    let mut vbo: u32 = 0;
    let mut cube_vao: u32 = 0;
    let mut light_cube_vao: u32 = 0;

    let lighting_shader;
    let light_cube_shader;
    let mut camera;
    type Vertex = [f32; 3];

    const VERTICES: [Vertex; 36] = [
        [-0.5, -0.5, -0.5 ],
        [ 0.5, -0.5, -0.5 ],
        [ 0.5,  0.5, -0.5 ],
        [ 0.5,  0.5, -0.5 ],
        [-0.5,  0.5, -0.5 ],
        [-0.5, -0.5, -0.5 ],
    
        [-0.5, -0.5,  0.5 ],
        [ 0.5, -0.5,  0.5 ],
        [ 0.5,  0.5,  0.5 ],
        [ 0.5,  0.5,  0.5 ],
        [-0.5,  0.5,  0.5 ],
        [-0.5, -0.5,  0.5 ],
    
        [-0.5,  0.5,  0.5 ],
        [-0.5,  0.5, -0.5 ],
        [-0.5, -0.5, -0.5 ],
        [-0.5, -0.5, -0.5 ],
        [-0.5, -0.5,  0.5 ],
        [-0.5,  0.5,  0.5 ],
    
        [ 0.5,  0.5,  0.5 ],
        [ 0.5,  0.5, -0.5 ],
        [ 0.5, -0.5, -0.5 ],
        [ 0.5, -0.5, -0.5 ],
        [ 0.5, -0.5,  0.5 ],
        [ 0.5,  0.5,  0.5 ],
    
        [-0.5, -0.5, -0.5 ],
        [ 0.5, -0.5, -0.5 ],
        [ 0.5, -0.5,  0.5 ],
        [ 0.5, -0.5,  0.5 ],
        [-0.5, -0.5,  0.5 ],
        [-0.5, -0.5, -0.5 ],
    
        [-0.5,  0.5, -0.5 ],
        [ 0.5,  0.5, -0.5 ],
        [ 0.5,  0.5,  0.5 ],
        [ 0.5,  0.5,  0.5 ],
        [-0.5,  0.5,  0.5 ],
        [-0.5,  0.5, -0.5 ],
    ];

    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.set_framebuffer_size_polling(true);
    window.make_current();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);

        lighting_shader = Shader::new("src/shaders/vertex_color.glsl", "src/shaders/fragment_color.glsl");
        light_cube_shader = Shader::new("src/shaders/vertex_light_cube.glsl", "src/shaders/fragment_light_cube.glsl");
        camera = Camera::new();
        // Set clear color
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);

        // Generate vertex array object
        gl::GenVertexArrays(1, &mut cube_vao);
        gl::GenBuffers(1, &mut vbo);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, size_of_val(&VERTICES) as isize, VERTICES.as_ptr().cast(), gl::STATIC_DRAW);

        gl::BindVertexArray(cube_vao);

        // Vertex positions
        gl::VertexAttribPointer(
            0, 
            3, 
            gl::FLOAT, 
            gl::FALSE, 
            (3 * std::mem::size_of::<f32>()) as gl::types::GLint, 
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);

        gl::GenVertexArrays(1, &mut light_cube_vao);
        gl::BindVertexArray(light_cube_vao);

        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);

        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, (3 * std::mem::size_of::<f32>()) as gl::types::GLint, 0 as *const _);
        gl::EnableVertexAttribArray(0);
    }

    let mut last_frame: f32 = 0.0;
    let mut current_frame: f32;
    let mut delta_time: f32;
    let mut last_x_pos: f32 = 0.0;
    let mut last_y_pos: f32 = 0.0;
    let mut first_mouse_event = true;

    while !window.should_close() {
        unsafe {
            current_frame = glfw::ffi::glfwGetTime() as f32;
        }
        delta_time = current_frame.clone() - last_frame;
        last_frame = current_frame.clone();
        camera.process_keyboard_movement(&delta_time);


        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            handle_window_event(&mut window, event, &mut camera, &mut last_x_pos, &mut last_y_pos, &mut first_mouse_event);
        }

        unsafe {
            gl::ClearColor(0.1, 0.1, 0.1, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            lighting_shader.use_shader();
            lighting_shader.set_vec_3("objectColor", Vec3{ x: 1.0, y: 0.5, z: 0.31 });
            lighting_shader.set_vec_3("lightColor",  Vec3{ x: 1.0, y: 1.0, z: 1.0 });

            let view = camera.get_view_matrix();
            let projection = ultraviolet::projection::perspective_gl(
                camera.zoom.to_radians(),
                800.0 / 600.0,
                0.1, 
                100.0);
            
            lighting_shader.set_mat_4("projection", projection);
            lighting_shader.set_mat_4("view", view);

            let model = Mat4::from_scale(1.0);
            lighting_shader.set_mat_4("model", model);

            gl::BindVertexArray(cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);

            
            light_cube_shader.use_shader();
            light_cube_shader.set_mat_4("projection", projection);
            light_cube_shader.set_mat_4("view", view);

            let mut model = Mat4::from_scale(0.2);
            model.translate(&Vec3{x: 1.2, y: 1.0, z: 2.0});
            light_cube_shader.set_mat_4("model", model);
            
            gl::BindVertexArray(light_cube_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 36);
        }
        window.swap_buffers();
    }
    unsafe {
        gl::DeleteVertexArrays(1, &mut light_cube_vao);
        gl::DeleteVertexArrays(1, &mut cube_vao);
        gl::DeleteBuffers(1, &mut vbo);
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent, camera: &mut Camera, last_x_pos: &mut f32, last_y_pos: &mut f32, first_mouse_event: &mut bool) {
    // println!("{:?}", event);
    match event {
        glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
            window.set_should_close(true)
        },
        glfw::WindowEvent::Key(Key::W, _, Action::Press, _) => {
            camera.start_movement(CameraMovement::Forward)
        },
        glfw::WindowEvent::Key(Key::S, _, Action::Press, _) => {
            camera.start_movement(CameraMovement::Backword)
        },
        glfw::WindowEvent::Key(Key::A, _, Action::Press, _) => {
            camera.start_movement(CameraMovement::Left)
        },
        glfw::WindowEvent::Key(Key::D, _, Action::Press, _) => {
            camera.start_movement(CameraMovement::Right)
        },
        glfw::WindowEvent::Key(Key::W, _, Action::Release, _) => {
            camera.stop_movement(CameraMovement::Forward)
        },
        glfw::WindowEvent::Key(Key::S, _, Action::Release, _) => {
            camera.stop_movement(CameraMovement::Backword)
        },
        glfw::WindowEvent::Key(Key::A, _, Action::Release, _) => {
            camera.stop_movement(CameraMovement::Left)
        },
        glfw::WindowEvent::Key(Key::D, _, Action::Release, _) => {
            camera.stop_movement(CameraMovement::Right)
        },
        glfw::WindowEvent::Scroll(_, y_offset) => {
            camera.process_mouse_scroll(y_offset as f32);
        },
        glfw::WindowEvent::CursorPos(x_pos, y_pos) => {
            let new_x_pos = x_pos as f32;
            let new_y_pos = y_pos as f32;

            if *first_mouse_event {
                *last_x_pos = new_x_pos.clone();
                *last_y_pos = new_y_pos.clone();
                *first_mouse_event = false;
            }
            let x_offset = new_x_pos - *last_x_pos;
            let y_offset = *last_y_pos - new_y_pos;

            *last_x_pos = new_x_pos.clone();
            *last_y_pos = new_y_pos.clone();

            camera.process_mouse_movement(x_offset, y_offset);
        },
        glfw::WindowEvent::FramebufferSize(width, height) => {
            unsafe {
                gl::Viewport(0, 0, width, height);
            }
        }
        _ => {},
    }
}

fn load_gl_symbol() {
    gl_loader::init_gl();
    gl::load_with(|symbol| gl_loader::get_proc_address(symbol) as *const _);
}
