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
    let mut vao: u32 = 0;
    // let mut ebo: u32 = 0;
    let shader;
    let mut camera;
    let mut texture1: u32 = 0;
    let mut texture2: u32 = 0;
    type Vertex = [f32; 5];
    // type TriIndexes = [u32; 3];
    // const VERTICES: [Vertex; 4] = [
    //     [ 0.5,  0.5, 0.0, 1.0, 0.0, 0.0, 1.0, 1.0],
    //     [ 0.5, -0.5, 0.0, 0.0, 1.0, 0.0, 1.0, 0.0],
    //     [-0.5, -0.5, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
    //     [-0.5,  0.5, 0.0, 1.0, 0.0, 1.0, 0.0, 1.0]
    // ];

    const VERTICES: [Vertex; 36] = [
        [-0.5, -0.5, -0.5,  0.0, 0.0],
        [ 0.5, -0.5, -0.5,  1.0, 0.0],
        [ 0.5,  0.5, -0.5,  1.0, 1.0],
        [ 0.5,  0.5, -0.5,  1.0, 1.0],
        [-0.5,  0.5, -0.5,  0.0, 1.0],
        [-0.5, -0.5, -0.5,  0.0, 0.0],
    
        [-0.5, -0.5,  0.5,  0.0, 0.0],
        [ 0.5, -0.5,  0.5,  1.0, 0.0],
        [ 0.5,  0.5,  0.5,  1.0, 1.0],
        [ 0.5,  0.5,  0.5,  1.0, 1.0],
        [-0.5,  0.5,  0.5,  0.0, 1.0],
        [-0.5, -0.5,  0.5,  0.0, 0.0],
    
        [-0.5,  0.5,  0.5,  1.0, 0.0],
        [-0.5,  0.5, -0.5,  1.0, 1.0],
        [-0.5, -0.5, -0.5,  0.0, 1.0],
        [-0.5, -0.5, -0.5,  0.0, 1.0],
        [-0.5, -0.5,  0.5,  0.0, 0.0],
        [-0.5,  0.5,  0.5,  1.0, 0.0],
    
        [ 0.5,  0.5,  0.5,  1.0, 0.0],
        [ 0.5,  0.5, -0.5,  1.0, 1.0],
        [ 0.5, -0.5, -0.5,  0.0, 1.0],
        [ 0.5, -0.5, -0.5,  0.0, 1.0],
        [ 0.5, -0.5,  0.5,  0.0, 0.0],
        [ 0.5,  0.5,  0.5,  1.0, 0.0],
    
        [-0.5, -0.5, -0.5,  0.0, 1.0],
        [ 0.5, -0.5, -0.5,  1.0, 1.0],
        [ 0.5, -0.5,  0.5,  1.0, 0.0],
        [ 0.5, -0.5,  0.5,  1.0, 0.0],
        [-0.5, -0.5,  0.5,  0.0, 0.0],
        [-0.5, -0.5, -0.5,  0.0, 1.0],
    
        [-0.5,  0.5, -0.5,  0.0, 1.0],
        [ 0.5,  0.5, -0.5,  1.0, 1.0],
        [ 0.5,  0.5,  0.5,  1.0, 0.0],
        [ 0.5,  0.5,  0.5,  1.0, 0.0],
        [-0.5,  0.5,  0.5,  0.0, 0.0],
        [-0.5,  0.5, -0.5,  0.0, 1.0],
    ];

    let cube_positions: [Vec3; 10] = [
        Vec3{ x:  0.0, y:  0.0, z:  0.0},
        Vec3{ x:  2.0, y:  5.0, z: -15.0},
        Vec3{ x: -1.5, y: -2.2, z: -2.5},
        Vec3{ x: -3.8, y: -2.0, z: -12.3},
        Vec3{ x:  2.4, y: -0.4, z: -3.5},
        Vec3{ x: -1.7, y:  3.0, z: -7.5},
        Vec3{ x:  1.3, y: -2.0, z: -2.5},
        Vec3{ x:  1.5, y:  2.0, z: -2.5},
        Vec3{ x:  1.5, y:  0.2, z: -1.5},
        Vec3{ x: -1.3, y:  1.0, z: -1.5}
    ];

    // const INDICES: [TriIndexes; 2] = [
    //     [0, 1, 3], 
    //     [1, 2, 3]
    // ];

    window.set_key_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);
    window.set_framebuffer_size_polling(true);
    window.make_current();

    unsafe {
        gl::Enable(gl::DEPTH_TEST);

        shader = Shader::new("src/shaders/vertex.glsl", "src/shaders/fragment.glsl");
        camera = Camera::new();
        // Set clear color
        gl::ClearColor(0.2, 0.3, 0.3, 1.0);

        // Generate vertex array object
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::BindVertexArray(vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER, 
            size_of_val(&VERTICES) as isize, 
            VERTICES.as_ptr().cast(), 
            gl::STATIC_DRAW
        );

        // Vertex positions
        gl::VertexAttribPointer(
            0, 
            3, 
            gl::FLOAT, 
            gl::FALSE, 
            (5 * std::mem::size_of::<f32>()) as gl::types::GLint, 
            0 as *const _,
        );
        gl::EnableVertexAttribArray(0);
        // Texture coordinates
        gl::VertexAttribPointer(
            1, 
            2, 
            gl::FLOAT, 
            gl::FALSE, 
            (5 * std::mem::size_of::<f32>()) as gl::types::GLint, 
            (3 * std::mem::size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);

        // Texture 1
        gl::GenTextures(1, &mut texture1);
        gl::BindTexture(gl::TEXTURE_2D, texture1);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);

        let image = match stb_image::image::load("assets/container.jpeg") {
            stb_image::image::LoadResult::ImageU8(value) => value,
            _ => panic!("Failed to load texture"),
        };
        gl::TexImage2D(
            gl::TEXTURE_2D, 
            0, 
            gl::RGB as i32, 
            image.width as i32, 
            image.height as i32, 
            0, 
            gl::RGB, 
            gl::UNSIGNED_BYTE, 
            image.data.as_ptr().cast(),
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        // Texture 2
        gl::GenTextures(1, &mut texture2);
        gl::BindTexture(gl::TEXTURE_2D, texture2);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);	
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR_MIPMAP_LINEAR as i32);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
        stb_image::stb_image::bindgen::stbi_set_flip_vertically_on_load(1);
        let image = match stb_image::image::load("assets/awesomeface.png") {
            stb_image::image::LoadResult::ImageU8(value) => value,
            _ => panic!("Failed to load texture"),
        };

        gl::TexImage2D(
            gl::TEXTURE_2D, 
            0, 
            gl::RGB as i32, 
            image.width as i32, 
            image.height as i32, 
            0, 
            gl::RGBA, 
            gl::UNSIGNED_BYTE, 
            image.data.as_ptr().cast(),
        );
        gl::GenerateMipmap(gl::TEXTURE_2D);

        shader.use_shader();
        shader.set_int("texture1", 0);
        shader.set_int("texture2", 1);

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
            gl::ClearColor(0.2, 0.3, 0.3, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, texture1);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, texture2);

            shader.use_shader();

            let view = camera.get_view_matrix();
            let projection = ultraviolet::projection::perspective_gl(
                camera.zoom.to_radians(),
                800.0 / 600.0,
                0.1, 
                100.0);
            
            shader.set_mat_4("projection", projection);
            shader.set_mat_4("view", view);

            gl::BindVertexArray(vao);

            for i in 0..10 {
                let mut time_value = glfw::ffi::glfwGetTime();
                if i % 2 == 0 {
                    time_value = 0.0;
                }
                let cube_position = cube_positions[i];
                let angle: f32 = 20.0 * i as f32;
                let model = Mat4::from_translation(cube_position)
                    * Mat4::from_rotation_x(3.0)
                    * Mat4::from_rotation_y((1.0 + i as f32) * 0.8 + time_value as f32)
                    * Mat4::from_rotation_z(angle * (1.0 + i as f32));

                shader.set_mat_4("model", model);
                gl::DrawArrays(gl::TRIANGLES, 0, 36);
            }
        }
        window.swap_buffers();
    }
    unsafe {
        gl::DeleteVertexArrays(1, &mut vao);
        gl::DeleteBuffers(1, &mut vbo);
    }
}

fn handle_window_event(window: &mut glfw::Window, event: glfw::WindowEvent, camera: &mut Camera, last_x_pos: &mut f32, last_y_pos: &mut f32, first_mouse_event: &mut bool) {
    println!("{:?}", event);
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
