//! 3D black hole viewer: warped grid + geodesic ray-traced image.

mod camera;
mod constants;
mod gl_utils;
mod grid;
mod math;
mod scene;
mod shaders;

use glfw::{Action, Context, Key, MouseButton, WindowEvent};

use camera::Camera;
use gl_utils::create_program;
use grid::generate_grid;
use math::{camera_basis, look_at, mat4_mul, perspective};
use scene::grid_objects;
use shaders::{GRID_FRAG, GRID_VERT, QUAD_FRAG_GEODESIC, QUAD_VERT};

pub use camera::Camera as ThreeDCamera;
pub use scene::{BlackHole, GridObject};

pub fn run() {
    // Program setup
    let mut glfw = glfw::init_no_callbacks().unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    #[cfg(target_os = "macos")]
    glfw.window_hint(glfw::WindowHint::OpenGlForwardCompat(true));

    // Window setup
    let (mut window, events) = glfw
        .create_window(800, 600, "Black Hole Grid", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window");

    window.make_current();
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_scroll_polling(true);

    gl::load_with(|s| window.get_proc_address(s) as *const _);

    // -- Scene --
    let bh = BlackHole::new();
    let bh_r_s: f32 = bh.r_s;

    // -- GPU setup --
    let (program, vao, index_count, quad_program, quad_vao) = unsafe {
        let program = create_program(GRID_VERT, GRID_FRAG);
        let quad_program = create_program(QUAD_VERT, QUAD_FRAG_GEODESIC);

        // Grid geometry
        let (vertices, indices) = generate_grid(&bh, &grid_objects());
        let index_count = indices.len() as i32;

        // Allocate GPU buffers
        let (mut vao, mut vbo, mut ebo) = (0u32, 0u32, 0u32);
        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ebo);

        // Start VAO
        gl::BindVertexArray(vao);

        // Move indices and vertices data to GPU
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (vertices.len() * std::mem::size_of::<[f32; 3]>()) as isize,
            vertices.as_ptr() as *const _,
            gl::DYNAMIC_DRAW,
        );
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
        gl::BufferData(
            gl::ELEMENT_ARRAY_BUFFER,
            (indices.len() * std::mem::size_of::<u32>()) as isize,
            indices.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 12, std::ptr::null());
        gl::EnableVertexAttribArray(0);
        gl::BindVertexArray(0);

        // Fullscreen quad
        let quad_verts: [f32; 24] = [
            -1.0, 1.0, 0.0, 1.0, -1.0, -1.0, 0.0, 0.0, 1.0, -1.0, 1.0, 0.0, -1.0, 1.0, 0.0, 1.0,
            1.0, -1.0, 1.0, 0.0, 1.0, 1.0, 1.0, 1.0,
        ];

        // Allocate GPU buffers and move to GPU
        let mut quad_vao = 0u32;
        let mut quad_vbo = 0u32;
        gl::GenVertexArrays(1, &mut quad_vao);
        gl::GenBuffers(1, &mut quad_vbo);
        gl::BindVertexArray(quad_vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, quad_vbo);
        gl::BufferData(
            gl::ARRAY_BUFFER,
            (quad_verts.len() * std::mem::size_of::<f32>()) as isize,
            quad_verts.as_ptr() as *const _,
            gl::STATIC_DRAW,
        );
        gl::VertexAttribPointer(0, 2, gl::FLOAT, gl::FALSE, 16, std::ptr::null());
        gl::EnableVertexAttribArray(0);
        gl::VertexAttribPointer(
            1,
            2,
            gl::FLOAT,
            gl::FALSE,
            16,
            (2 * std::mem::size_of::<f32>()) as *const _,
        );
        gl::EnableVertexAttribArray(1);
        gl::BindVertexArray(0);

        (program, vao, index_count, quad_program, quad_vao)
    };

    // Uniform locations
    let loc_vp =
        unsafe { gl::GetUniformLocation(program, b"viewProj\0".as_ptr() as *const _) };
    let (loc_cam_pos, loc_cam_right, loc_cam_up, loc_cam_fwd, loc_thfov, loc_aspect, loc_rs) =
        unsafe {
            (
                gl::GetUniformLocation(quad_program, b"camPos\0".as_ptr() as *const _),
                gl::GetUniformLocation(quad_program, b"camRight\0".as_ptr() as *const _),
                gl::GetUniformLocation(quad_program, b"camUp\0".as_ptr() as *const _),
                gl::GetUniformLocation(quad_program, b"camForward\0".as_ptr() as *const _),
                gl::GetUniformLocation(quad_program, b"tanHalfFov\0".as_ptr() as *const _),
                gl::GetUniformLocation(quad_program, b"aspect\0".as_ptr() as *const _),
                gl::GetUniformLocation(quad_program, b"r_s\0".as_ptr() as *const _),
            )
        };

    // Camera perspectives, move to GPU
    let mut camera = Camera::new();
    let tan_hfov = (60.0_f32.to_radians() * 0.5).tan();
    let aspect = 800.0 / 600.0_f32;

    while !window.should_close() {
        // Events
        glfw.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            match event {
                // Close event
                WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                }

                // Drag event - Pressed
                WindowEvent::MouseButton(MouseButton::Button1, Action::Press, _) => {
                    camera.dragging = true;
                    let (x, y) = window.get_cursor_pos();
                    camera.last_x = x as f32;
                    camera.last_y = y as f32;
                }
                // Drag event - Released
                WindowEvent::MouseButton(MouseButton::Button1, Action::Release, _) => {
                    camera.dragging = false;
                }

                // Drag - update camera object
                WindowEvent::CursorPos(x, y) => {
                    if camera.dragging {
                        let dx = x as f32 - camera.last_x;
                        let dy = y as f32 - camera.last_y;
                        camera.azimuth += dx * 0.01;
                        camera.elevation -= dy * 0.01;
                        camera.elevation = camera
                            .elevation
                            .clamp(0.01, std::f32::consts::PI - 0.01);
                    }
                    camera.last_x = x as f32;
                    camera.last_y = y as f32;
                }
                // Scroll - update camera radius
                WindowEvent::Scroll(_, y) => {
                    camera.radius -= y as f32 * 1e9;
                    camera.radius = camera.radius.clamp(1e10, 1e13);
                }
                _ => {}
            }
        }

        let pos = camera.position();
        let (right, up, fwd) = camera_basis(pos);

        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);

            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);

            // Geodesic ray-traced image
            gl::Disable(gl::DEPTH_TEST);
            gl::UseProgram(quad_program);
            gl::Uniform3fv(loc_cam_pos, 1, pos.as_ptr());
            gl::Uniform3fv(loc_cam_right, 1, right.as_ptr());
            gl::Uniform3fv(loc_cam_up, 1, up.as_ptr());
            gl::Uniform3fv(loc_cam_fwd, 1, fwd.as_ptr());
            gl::Uniform1f(loc_thfov, tan_hfov);
            gl::Uniform1f(loc_aspect, aspect);
            gl::Uniform1f(loc_rs, bh_r_s);
            gl::BindVertexArray(quad_vao);
            gl::DrawArrays(gl::TRIANGLES, 0, 6);
            gl::BindVertexArray(0);
            gl::Enable(gl::DEPTH_TEST);

            // Warped grid overlay
            let view = look_at(pos, [0.0, 0.0, 0.0]);
            let proj = perspective(60.0_f32.to_radians(), 800.0 / 600.0, 1e9, 1e14);
            let view_proj = mat4_mul(&proj, &view);

            gl::UseProgram(program);
            gl::UniformMatrix4fv(loc_vp, 1, gl::FALSE, view_proj.as_ptr());
            gl::BindVertexArray(vao);
            gl::DrawElements(gl::LINES, index_count, gl::UNSIGNED_INT, std::ptr::null());
            gl::BindVertexArray(0);
        }

        window.swap_buffers();
    }
}
