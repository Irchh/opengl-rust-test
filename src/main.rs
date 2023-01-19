#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use sdl2::event::Event;
use sdl2::video::GLProfile;

fn find_sdl_gl_driver() -> Option<u32> {
    for (index, item) in sdl2::render::drivers().enumerate() {
        if item.name == "opengl" {
            return Some(index as u32);
        }
    }
    None
}

fn main() {
    let sdl_context = sdl2::init().expect("SDL init failed!");
    let video_sub = sdl_context.video().expect("SDL video subsystem init failed!");

    let gl_attr = video_sub.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 6);

    let window = video_sub.window("Rust opengl test", 800, 600)
        .opengl()
        .build()
        .expect("Could not initialize SDL window!");

    let ctx = window.gl_create_context().expect("Could not create OpenGL context!");
    gl::load_with(|name| video_sub.gl_get_proc_address(name) as *const _);

    debug_assert_eq!(gl_attr.context_profile(), GLProfile::Core);
    debug_assert_eq!(gl_attr.context_version(), (4, 6));

    let mut event_pump = sdl_context.event_pump().expect("Could not get SDL event pump!");

    'kuukkis: loop {
        unsafe {
            let mut vao = 0;
            let mut vbo = 0;
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);

            gl::BindVertexArray(vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);



            gl::ClearColor(0.6, 0.0, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.gl_swap_window();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {
                    break 'kuukkis;
                }
                _ => {}
            }
        }
        std::thread::sleep(::std::time::Duration::new(0, 1_000_000_000u32 / 60));
    }

    println!("Hello, world!");
}
