extern crate gl;

use std::fs::File;
use std::io::Read;

pub struct Shader{
    vertex_shader: gl::types::GLuint,
    fragment_shader: gl::types::GLuint,
    compute_shader: gl::types::GLuint,
    pub shader_program: gl::types::GLuint
}

impl Drop for Shader{
    fn drop(&mut self) {
        unsafe{
            if self.shader_program > 0 {
                Shader::disable_shader();
                gl::DeleteProgram(self.shader_program);
                println!("Deleting Shader");
            }
        }
    }
}

impl Shader{
    pub fn new(vertex: String, fragment: String) -> Shader{
        let mut shader = Shader{
            vertex_shader: 0,
            fragment_shader: 0,
            compute_shader: 0,
            shader_program: 0
        };

        let mut vertex_file = File::open(vertex)
            .expect("Couldn't open vertex shader");

        let mut vertex_source = String::new();
        vertex_file.read_to_string(&mut vertex_source)
            .expect("Couldn't read vertex file");

        vertex_source.push('\0');

        let mut fragment_file = File::open(fragment)
            .expect("Couldn't open fragment shader");

        let mut fragment_source = String::new();
        fragment_file.read_to_string(&mut fragment_source)
            .expect("Couldn't read fragment file");

        fragment_source.push('\0');

        unsafe {
            shader.vertex_shader = gl::CreateShader(gl::VERTEX_SHADER);
            shader.fragment_shader = gl::CreateShader(gl::FRAGMENT_SHADER);
            shader.shader_program = gl::CreateProgram();


            gl::ShaderSource(shader.vertex_shader, 1,
                             &(vertex_source.as_bytes().as_ptr().cast()),
                             std::ptr::null());
            gl::ShaderSource(shader.fragment_shader, 1,
                             &(fragment_source.as_bytes().as_ptr().cast()),
                             std::ptr::null());

            gl::CompileShader(shader.vertex_shader);

            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(
                shader.vertex_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            println!("Vertex Compile Error: {}", String::from_utf8_lossy(&v));

            gl::CompileShader(shader.fragment_shader);

            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(
                shader.fragment_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            println!("Fragment Compile Error: {}", String::from_utf8_lossy(&v));

            gl::AttachShader(shader.shader_program, shader.vertex_shader);
            gl::AttachShader(shader.shader_program, shader.fragment_shader);

            gl::LinkProgram(shader.shader_program);

            gl::DeleteShader(shader.vertex_shader);
            gl::DeleteShader(shader.fragment_shader);
        }

        shader
    }

    pub fn new_compute(compute: String) -> Shader{
        let mut shader = Shader{
            vertex_shader: 0,
            fragment_shader: 0,
            compute_shader: 0,
            shader_program: 0
        };

        let mut compute_file = File::open(compute)
            .expect("Couldn't open compute shader");

        let mut compute_source = String::new();
        compute_file.read_to_string(&mut compute_source)
            .expect("Couldn't read compute file");

        compute_source.push('\0');


        unsafe {
            shader.compute_shader = gl::CreateShader(gl::COMPUTE_SHADER);
            shader.shader_program = gl::CreateProgram();


            gl::ShaderSource(shader.compute_shader, 1,
                             &(compute_source.as_bytes().as_ptr().cast()),
                             std::ptr::null());

            gl::CompileShader(shader.compute_shader);

            let mut v: Vec<u8> = Vec::with_capacity(1024);
            let mut log_len = 0_i32;
            gl::GetShaderInfoLog(
                shader.compute_shader,
                1024,
                &mut log_len,
                v.as_mut_ptr().cast(),
            );
            v.set_len(log_len.try_into().unwrap());
            println!("Compute Compile Error: {}", String::from_utf8_lossy(&v));

            gl::AttachShader(shader.shader_program, shader.compute_shader);

            gl::LinkProgram(shader.shader_program);

            gl::DeleteShader(shader.compute_shader);
        }

        shader
    }

    pub fn use_shader(&self){
        unsafe{
            gl::UseProgram(self.shader_program);
        }
    }

    pub fn disable_shader(){
        unsafe{
            println!("Disabling shader!");
            gl::UseProgram(0);
        }
    }
}