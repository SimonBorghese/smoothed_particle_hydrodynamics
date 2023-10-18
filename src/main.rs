use std::ptr;
use sdl2;
use glm;
use gl;
use imgui;
use imgui::Condition;
use imgui_sdl2;
use imgui_opengl_renderer;
use rand;
use rand::{Rng};
use sdl2::mouse::MouseButton;
use crate::ogl::Buffer;

static EPS: f32 = 16.0;

mod ogl;
#[derive(Copy)]
pub struct Particle{
    pub position: glm::Vec2,
    pub velocity: glm::Vec2,
    pub forces: glm::Vec2,
    pub press_force: glm::Vec2,
    pub visc_force: glm::Vec2,
    pub pressure: f32,
    pub rho: f32
}

impl Clone for Particle {
    fn clone(&self) -> Self {
        Particle{
            position: self.position,
            velocity: self.velocity,
            forces: self.forces,
            press_force: self.press_force,
            visc_force: self.visc_force,
            pressure: self.pressure,
            rho: self.rho,
        }
    }
}


pub struct PARAMS{
    pub pixel_size: i32,
    pub rest_dens: f32,
    pub gas_const: f32,
    pub kernel_radius: f32,
    pub kr_sq: f32,
    pub mass: f32,
    pub visc: f32,
    pub eps: f32,
    pub bound_damping: f32

}

fn main() {
    let mut pause = false;
    let mut parameters = PARAMS{
        pixel_size: 10,
        rest_dens: 300.0,
        gas_const: 2000.0,
        kernel_radius: 16.0,
        kr_sq: (16.0_f32.powf(2.0)),
        mass: 2.5,
        visc: 200.0,
        eps: 16.0,
        bound_damping: -0.5,
    };
    let mut rand_thread = rand::thread_rng();
    let mut particles: [Particle; 10000] = [Particle{
        position: glm::vec2(0.0, 0.0),
        velocity: glm::vec2(0.0, 0.0),
        forces: glm::vec2(0.0, 0.0),
        press_force: glm::vec2(0.0, 0.0),
        visc_force: glm::vec2(0.0, 0.0),
        pressure: 0.0,
        rho: 0.0,
    }; 10000];
    let mut num_particles: usize = 00;
    let mut num_placed = 0;
    for y in 1..(900/EPS as i32){
        for x in 1..((1600/2)/EPS as i32){
            if num_placed < num_particles {
                particles[num_placed] = Particle {
                    position: glm::vec2((x as i32 * (EPS-2.0) as i32) as f32 + rand_thread.gen_range(0..10) as f32, (y * EPS as i32) as f32 + 20.0),
                    velocity: glm::vec2(0.0, 0.0),
                    forces: glm::vec2(0.0, 0.0),
                    press_force: glm::vec2(0.0, 0.0),
                    visc_force: glm::vec2(0.0, 0.0),
                    pressure: 0.0,
                    rho: 0.0,
                };
                num_placed += 1;
            }
        }
    }
    println!("Num placed: {}", num_placed);


    let sdl2_ctx = sdl2::init().expect("Couldn't get SDL2 CTX");
    let sdl2_video = sdl2_ctx.video()
        .expect("Couldn't create video subsystem");


    let gl_attr = sdl2_video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 6);

    let sdl2_gl_window = sdl2_video.window("SPH-GL", 1600, 900)
        .opengl()
        .position_centered()
        .build()
        .expect("Couldn't make OpenGL Window!!!");

    gl::load_with(|name| sdl2_video.gl_get_proc_address(name) as *const _);

    let _ogl_ctx = sdl2_gl_window.gl_create_context().expect("Couldn't create GL Context");
    //sdl2_gl_window.gl_set_context_to_current().expect("Couldn't make GL Context curr");

    let mut imgui = imgui::Context::create();
    imgui.set_ini_filename(None);
    let mut imgui_sdl2 = imgui_sdl2::ImguiSdl2::new(&mut imgui, &sdl2_gl_window);
    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui,
                                                        |s| sdl2_video.gl_get_proc_address(s) as _);

    let vertices: Vec<f32> = vec![-1.0, -1.0, 0.0, 0.0, 0.0,
                                  1.0, -1.0, 0.0, 1.0, 0.0,
                                  -1.0, 1.0, 0.0, 0.0, 1.0,
                                  1.0, 1.0, 0.0, 1.0, 1.0];
    let indices: Vec<i32> = vec![0, 1, 2, 2, 3, 1];

    let mut mesh = ogl::vao::VertexArray::new();
    mesh.load_vertices(vertices, indices);
    mesh.bind();

    let shader = ogl::shader::Shader::new(String::from("vertex.glsl"),
                                          String::from("fragment.glsl"));
    shader.use_shader();

    let compute_clear = ogl::shader::Shader::new_compute(String::from("compute_clear.glsl"));

    let compute_position = ogl::shader::Shader::new_compute(String::from("compute_position.glsl"));

    let compute_density_clear = ogl::shader::Shader::new_compute(String::from("compute_density_clear.glsl"));

    let compute_density_shader = ogl::shader::Shader::new_compute(String::from("compute_density.glsl"));

    let compute_density_finish = ogl::shader::Shader::new_compute(String::from("compute_desnity_finish.glsl"));

    let compute_force_clear = ogl::shader::Shader::new_compute(String::from("compute_force_clear.glsl"));

    let compute_force = ogl::shader::Shader::new_compute(String::from("compute_force.glsl"));

    let compute_force_finish = ogl::shader::Shader::new_compute(String::from("compute_force_finish.glsl"));

    let compute = ogl::shader::Shader::new_compute(String::from("compute.glsl"));
    compute.use_shader();

    // Init UBO block
    let param_ubo = Buffer::new();
    //unsafe {
    param_ubo.data(gl::UNIFORM_BUFFER,
                   ptr::NonNull::new(&mut parameters)
                       .expect("Couldn't new")
                       .as_ptr()
                       .cast(),
                   std::mem::size_of::<PARAMS>());
    //}
    param_ubo.binding(gl::UNIFORM_BUFFER, 4);

    let width_height = (1600, 900);

    let mut gl_img: gl::types::GLuint = 0;
    unsafe{
        gl::GenTextures(1, &mut gl_img);

        gl::ActiveTexture(gl::TEXTURE0);
        gl::BindTexture(gl::TEXTURE_2D, gl_img);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S,
                          gl::CLAMP_TO_EDGE as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T,
                          gl::CLAMP_TO_EDGE as gl::types::GLint);

        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER,
                          gl::LINEAR as gl::types::GLint);
        gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER,
                          gl::LINEAR as gl::types::GLint);

        gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA32F as gl::types::GLint,
                       width_height.0, width_height.1, 0,
                       gl::RGBA, gl::FLOAT, std::ptr::null());

        gl::BindImageTexture(0, gl_img, 0, gl::FALSE, 0,
                             gl::READ_WRITE, gl::RGBA32F);


    }

    let mut event_pump = sdl2_ctx.event_pump().expect("Couldn't get event pump!");

    let timer = sdl2_ctx.timer().expect("Couldn't get timer");

    let mut last_time = timer.ticks();
    let mut fps = 0;
    let mut real_fps = 0;

    let sphere_buff = ogl::Buffer::new();
    sphere_buff.data(gl::SHADER_STORAGE_BUFFER, particles.as_ptr().cast(), std::mem::size_of::<Particle>() * num_particles);
    sphere_buff.binding(gl::SHADER_STORAGE_BUFFER, 2);

    let mut mouse_pos: [i32;2] = [0;2];
    'mainLoop: loop{
        for event in event_pump.poll_iter(){
            match event{
                sdl2::event::Event::Quit {..} => {break 'mainLoop;}
                sdl2::event::Event::MouseMotion {x, y, ..} => {
                    mouse_pos = [x,900 - y];
                }
                sdl2::event::Event::MouseButtonDown {mouse_btn, ..} => {
                    match mouse_btn{
                        MouseButton::Middle => {
                            unsafe {
                                gl::GetBufferSubData(gl::SHADER_STORAGE_BUFFER, 0, (std::mem::size_of::<Particle>() * num_particles) as gl::types::GLsizeiptr, particles.as_mut_ptr().cast());
                            }
                            num_placed = num_particles;
                            num_particles += 100;
                            for y in 1..(900/EPS as i32){
                                for x in 1..((1600/6)/EPS as i32){
                                    if num_placed < num_particles {
                                        particles[num_placed] = Particle {
                                            position: glm::vec2(((x * (EPS-2.0) as i32) + mouse_pos[0]) as f32 + rand_thread.gen_range(0..10) as f32,
                                                                ((y * EPS as i32) + mouse_pos[1]) as f32 + 20.0),
                                            velocity: glm::vec2(0.0, 0.0),
                                            forces: glm::vec2(0.0, 0.0),
                                            press_force: glm::vec2(0.0, 0.0),
                                            visc_force: glm::vec2(0.0, 0.0),
                                            pressure: 0.0,
                                            rho: 0.0,
                                        };
                                        num_placed += 1;
                                    }
                                }
                            }
                            sphere_buff.data(gl::SHADER_STORAGE_BUFFER, particles.as_ptr().cast(), std::mem::size_of::<Particle>() * num_particles);
                        }
                        MouseButton::Right => {
                            unsafe {
                                gl::GetBufferSubData(gl::SHADER_STORAGE_BUFFER, 0, (std::mem::size_of::<Particle>() * num_particles) as gl::types::GLsizeiptr, particles.as_mut_ptr().cast());
                            }
                            num_particles -= 100;
                            sphere_buff.data(gl::SHADER_STORAGE_BUFFER, particles.as_ptr().cast(), std::mem::size_of::<Particle>() * num_particles);
                        }
                        _ => {}
                    }
                }
                _ => {}
            }
        }


        if !pause {
            unsafe {
                param_ubo.data(gl::UNIFORM_BUFFER,
                               ptr::NonNull::new(&mut parameters)
                                   .expect("Couldn't new")
                                   .as_ptr()
                                   .cast(),
                               std::mem::size_of::<PARAMS>());

                compute_density_clear.use_shader();
                gl::DispatchCompute(num_particles as u32, 1, 1);

                gl::MemoryBarrier(gl::SHADER_STORAGE_BARRIER_BIT);

                compute_density_shader.use_shader();
                gl::DispatchCompute(num_particles as u32, num_particles as u32, 1);

                gl::MemoryBarrier(gl::SHADER_STORAGE_BARRIER_BIT);

                compute_density_finish.use_shader();
                gl::DispatchCompute(num_particles as u32, 1, 1);

                compute_force_clear.use_shader();
                gl::DispatchCompute(num_particles as u32, 1, 1);

                gl::MemoryBarrier(gl::SHADER_STORAGE_BARRIER_BIT);

                compute_force.use_shader();
                gl::DispatchCompute(num_particles as u32, num_particles as u32, 1);

                gl::MemoryBarrier(gl::SHADER_STORAGE_BARRIER_BIT);

                compute_force_finish.use_shader();
                gl::DispatchCompute(num_particles as u32, 1, 1);

                gl::MemoryBarrier(gl::SHADER_STORAGE_BARRIER_BIT);
                //update_velocity_position(&mut particles, num_particles,  0.0007);
                //sphere_buff.update_data(gl::SHADER_STORAGE_BUFFER, particles.as_ptr().cast(), std::mem::size_of::<Particle>() * num_particles);
                compute_position.use_shader();

                //let initTime = timer.ticks();
                gl::DispatchCompute(num_particles as u32, 1, 1);

                gl::MemoryBarrier(gl::SHADER_STORAGE_BARRIER_BIT);
                //gl::GetBufferSubData(gl::SHADER_STORAGE_BUFFER, 0, (std::mem::size_of::<Particle>() * num_particles) as gl::types::GLsizeiptr, particles.as_mut_ptr().cast());
                //println!("Time: {}", timer.ticks() - initTime);
            }
        }



        compute_clear.use_shader();
        unsafe{
            gl::DispatchCompute(1600,900,1);
            gl::MemoryBarrier(gl::VERTEX_ATTRIB_ARRAY_BARRIER_BIT);
            gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
        }
        compute.use_shader();
        unsafe{
            gl::DispatchCompute(num_particles as u32, 1, 1);
            gl::MemoryBarrier(gl::VERTEX_ATTRIB_ARRAY_BARRIER_BIT);
            gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
        }


        shader.use_shader();


        imgui_sdl2.prepare_frame(imgui.io_mut(), &sdl2_gl_window, &event_pump.mouse_state());

        let mut fps_out = String::from("FPS: ");
        fps_out.push_str(real_fps
            .to_string()
            .as_str());

        let ui = imgui.frame();
        let mut str_out = String::from("Num Particles: ");
        str_out.push_str(num_particles.to_string().as_str());

        ui.window("Parameter editor")
            .size([300.0, 200.0], Condition::FirstUseEver)
            .build(|| {
                ui.text(fps_out);
                ui.text(str_out);
                ui.checkbox("Paused: ",&mut pause);
                ui.slider("Gas Const", 1.0, 3000.0, &mut parameters.gas_const);
                ui.slider("Rest Density", 1.0, 3000.0, &mut parameters.rest_dens);
                ui.slider("Mass", 1.0, 3000.0, &mut parameters.mass);
                ui.slider("Visc", 1.0, 3000.0, &mut parameters.visc);
            });
        unsafe{
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Uniform1i(2, 0);
        }
        mesh.draw_all_indices();


        imgui_sdl2.prepare_render(&ui, &sdl2_gl_window);
        renderer.render(&mut imgui);
        sdl2_gl_window.gl_swap_window();

        if timer.ticks() - last_time > 1000{
            real_fps = fps;
            fps = 0;
            last_time = timer.ticks();
        } else{
            fps += 1;
        }
    }
}
