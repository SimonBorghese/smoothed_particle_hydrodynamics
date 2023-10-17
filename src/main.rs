use std::ops::Neg;
use sdl2;
use glm;
use gl;
use imgui;
use imgui::Condition;
use imgui_sdl2;
use imgui_opengl_renderer;
mod ogl;
#[derive(Copy)]
pub struct Particle{
    pub position: glm::Vec2,
    pub velocity: glm::Vec2,
    pub forces: glm::Vec2,
    pub pressure: f32,
    pub rho: f32
}

impl Clone for Particle {
    fn clone(&self) -> Self {
        Particle{
            position: self.position,
            velocity: self.velocity,
            forces: self.forces,
            pressure: self.pressure,
            rho: self.rho,
        }
    }
}

static G: glm::Vec2 = glm::Vector2{
    x: 0.0,
    y: -10.0,
};

const PIXEL_SIZE: i32 = 8;
const PIXEL_MIN: i32 = -(PIXEL_SIZE / 2);
const PIXEL_MAX: i32 = PIXEL_SIZE / 2;
static REST_DENS: f32 = 300.0;
static mut GAS_CONST: f32 = 2000.0; // default 2000
static KERNEL_RADIUS: f32 = 16.0;
static KR_SQ: f32 = KERNEL_RADIUS * KERNEL_RADIUS;
static MASS: f32 = 2.5;
static VISC: f32 = 100.0;
static EPS: f32 = KERNEL_RADIUS;
static BOUND_DAMPING: f32 = -0.2;

fn POLY6() -> f32{
    4.0 / (std::f32::consts::PI * KERNEL_RADIUS.powf(8.0))
}

fn SPIKY_GRAD() -> f32{
    -10.0 / (std::f32::consts::PI * KERNEL_RADIUS.powf(5.0))
}

fn VISC_LAP() -> f32{
    40.0 / (std::f32::consts::PI * KERNEL_RADIUS.powf(5.0))
}

fn compute_density(particles: &mut [Particle], num_particles: usize){
    for pi in 0..num_particles{
        particles[pi].rho = 0.0;
        for pj in 0..num_particles{
            let rij = particles[pj].position - particles[pi].position;
            let r2 = glm::pow(rij.x, 2.0) + glm::pow(rij.y, 2.0);
            if r2 < KR_SQ{
                particles[pi].rho += MASS * POLY6() * (KR_SQ - r2).powf(3.0);
            }
        }
        unsafe { particles[pi].pressure = GAS_CONST * (particles[pi].rho - REST_DENS); }
    }
}

fn compute_forces(particles: &mut [Particle], num_particles: usize){
    for pi in 0..num_particles{
        let mut pressure_force = glm::Vec2::new(0.0, 0.0);
        let mut viscosity_force = glm::Vec2::new(0.0, 0.0);
        for pj in 0..num_particles{
            if pi == pj{
                continue;
            }

            let rij = particles[pj].position - particles[pi].position;
            let r = glm::sqrt(glm::pow(rij.x, 2.0) + glm::pow(rij.y, 2.0));

            if r < KERNEL_RADIUS{
                pressure_force = pressure_force + (glm::normalize(rij.neg()) * MASS * (particles[pi].pressure + particles[pj].pressure) / (2.0 * particles[pj].rho) * SPIKY_GRAD() * (KERNEL_RADIUS - r).powf(3.0));

                viscosity_force = viscosity_force + (glm::to_vec2(VISC * MASS) * (particles[pj].velocity - particles[pi].velocity) / particles[pj].rho * VISC_LAP() * (KERNEL_RADIUS - r));

            }
        }
        particles[pi].forces = pressure_force + viscosity_force + (G * MASS / particles[pi].rho);
    }
}

fn update_velocity_position(particles: &mut [Particle], num_particles: usize, delta_time: f32){
    for pi in 0..num_particles{
        particles[pi].velocity = particles[pi].velocity + (glm::to_vec2(delta_time) * particles[pi].forces / particles[pi].rho);
        particles[pi].position = particles[pi].position + (particles[pi].velocity * delta_time);
        let particle = &mut particles[pi];

        if particle.position.x - EPS < 0.0{
            particle.velocity.x *= BOUND_DAMPING;
            particle.position.x = EPS;
        }
        if particle.position.x + EPS > 800.0{
            particle.velocity.x *= BOUND_DAMPING;
            particle.position.x = 800.0 - EPS;
        }

        if particle.position.y - EPS < 0.0{
            particle.velocity.y *= BOUND_DAMPING;
            particle.position.y = EPS;
        }
        if particle.position.y + EPS > 600.0{
            particle.velocity.y *= BOUND_DAMPING;
            particle.position.y = 600.0 - EPS;
        }
    }
}
fn main() {
    let mut particles: [Particle; 1000] = [Particle{
        position: glm::vec2(0.0, 0.0),
        velocity: glm::vec2(0.0, 0.0),
        forces: glm::vec2(0.0, 0.0),
        pressure: 0.0,
        rho: 0.0,
    }; 1000];
    let mut num_particles: usize = 400;
    for x in 0..400{
        particles[x] = Particle{
            position: glm::vec2(10.0 * x as f32, 2.0 * x as f32),
            velocity: glm::vec2(0.0, 0.0),
            forces: glm::vec2(0.0, 0.0),
            pressure: 0.0,
            rho: 0.0,
        };
    }

    let sdl2_ctx = sdl2::init().expect("Couldn't get SDL2 CTX");
    let sdl2_video = sdl2_ctx.video()
        .expect("Couldn't create video subsystem");

    /*
    let mut canvas = sdl2_video.window("SPH", 800, 600)
        .build()
        .expect("Couldn't build window!")
        .into_canvas()
        .software()
        .build()
        .expect("Couldn't build canvas from window!");
*/
    let gl_attr = sdl2_video.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 6);

    let sdl2_gl_window = sdl2_video.window("SPH-GL", 800, 600)
        .opengl()
        .position_centered()
        .build()
        .expect("Couldn't make OpenGL Window!!!");

    gl::load_with(|name| sdl2_video.gl_get_proc_address(name) as *const _);

    let ogl_ctx = sdl2_gl_window.gl_create_context().expect("Couldn't create GL Context");
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

    let compute = ogl::shader::Shader::new_compute(String::from("compute.glsl"));
    compute.use_shader();


    let width_height = (800, 600);

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
                             gl::WRITE_ONLY, gl::RGBA32F);


    }

    let mut event_pump = sdl2_ctx.event_pump().expect("Couldn't get event pump!");

    let timer = sdl2_ctx.timer().expect("Couldn't get timer");

    let mut last_time = timer.ticks();

    let sphere_buff = ogl::Buffer::new();
    sphere_buff.data(gl::SHADER_STORAGE_BUFFER, particles.as_ptr().cast(), std::mem::size_of::<Particle>() * num_particles);
    sphere_buff.binding(gl::SHADER_STORAGE_BUFFER, 2);
    'mainLoop: loop{
        for event in event_pump.poll_iter(){
            match event{
                sdl2::event::Event::Quit {..} => {break 'mainLoop;}
                _ => {}
            }
        }

        compute_density(&mut particles, num_particles);
        compute_forces(&mut particles, num_particles);
        update_velocity_position(&mut particles, num_particles,  0.0007);
        last_time = timer.ticks();

        sphere_buff.update_data(gl::SHADER_STORAGE_BUFFER, particles.as_ptr().cast(), std::mem::size_of::<Particle>() * num_particles);

        /*
        canvas.set_draw_color(sdl2::pixels::Color::BLACK);
        canvas.clear();
        for p in 0..num_particles{
            let position = particles[p].position;
            canvas.set_draw_color(sdl2::pixels::Color::BLUE);
            for x in PIXEL_MIN..=PIXEL_MAX {
                for y in PIXEL_MIN..=PIXEL_MAX {
                    if ( x.pow(2) + PIXEL_MAX) + (y.pow(2)+PIXEL_MAX) <= PIXEL_SIZE {
                        canvas.draw_point(sdl2::rect::Point::new(position.x as i32 + x, position.y as i32 + y)).expect("Couldn't draw point");
                    }
                }
            }
        }
        canvas.present();
*/

        compute.use_shader();
        unsafe{
            gl::DispatchCompute(800,600,1);
            gl::MemoryBarrier(gl::VERTEX_ATTRIB_ARRAY_BARRIER_BIT);
            gl::MemoryBarrier(gl::SHADER_IMAGE_ACCESS_BARRIER_BIT);
        }

        shader.use_shader();


        imgui_sdl2.prepare_frame(imgui.io_mut(), &sdl2_gl_window, &event_pump.mouse_state());

        let ui = imgui.frame();

        ui.window("Hello world")
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(|| {
                unsafe { ui.slider("Gas Const", 0.0, 3000.0, &mut GAS_CONST); }
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
    }
}
