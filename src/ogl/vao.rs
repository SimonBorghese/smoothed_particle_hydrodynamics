extern crate gl;

use crate::ogl;
use crate::ogl::Buffer;

pub struct VertexArray{
    vao: gl::types::GLuint,
    vbo: ogl::Buffer,
    ebo: ogl::Buffer,
    num_indices: i32,
    num_vertices: i32
}

impl VertexArray{
    pub fn new() -> VertexArray{
        let mut vao: VertexArray = VertexArray{
            vao: 0,
            vbo: ogl::Buffer {buf: 0},
            ebo: ogl::Buffer {buf: 0},
            num_indices: 0,
            num_vertices: 0
        };

        unsafe{
            gl::GenVertexArrays(1, &mut vao.vao);
        }

        vao.vbo = ogl::Buffer::new();
        vao.ebo = ogl::Buffer::new();

        vao
    }

    pub fn bind(&self){
        unsafe{
            gl::BindVertexArray(self.vao);
        }
    }

    pub fn bind_base(&self, loc: u32){
        unsafe{
            gl::BindBufferBase(gl::SHADER_STORAGE_BUFFER, loc, self.vbo.buf);
        }
    }

    pub fn bind_none(){
        unsafe{
            gl::BindVertexArray(0);
        }
    }

    pub fn load_vertices(&mut self, vertices: Vec<f32>, indices: Vec<i32>){
        self.bind();
        self.vbo.bind(gl::ARRAY_BUFFER);
        self.ebo.bind(gl::ELEMENT_ARRAY_BUFFER);

        self.num_vertices = vertices.len() as i32;
        self.num_indices = indices.len() as i32;

        unsafe{
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER,
                           (indices.len() * std::mem::size_of::<i32>()) as gl::types::GLsizeiptr,
                           indices.as_ptr() as *const gl::types::GLvoid,
                           gl::STATIC_DRAW);

            gl::BufferData(gl::ARRAY_BUFFER,
                           (vertices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                            vertices.as_ptr() as *const gl::types::GLvoid,
                            gl::STATIC_DRAW);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE,
                                    (std::mem::size_of::<f32>() * 5) as gl::types::GLsizei,
                                    0 as *const gl::types::GLvoid);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE,
                                    (std::mem::size_of::<f32>() * 5) as gl::types::GLsizei,
                                    (std::mem::size_of::<f32>() * 3) as *const gl::types::GLvoid);
            gl::EnableVertexAttribArray(0);
            gl::EnableVertexAttribArray(1);
        }
        VertexArray::bind_none();
        Buffer::bind_none(gl::ELEMENT_ARRAY_BUFFER);
        Buffer::bind_none(gl::ARRAY_BUFFER);
    }

    pub fn draw_all_indices(&self){
        self.bind();
        unsafe{
            gl::DrawElements(gl::TRIANGLES,
                             self.num_indices,
                             gl::UNSIGNED_INT,
                             0 as *const gl::types::GLvoid);
        }
    }

    pub fn draw_as_points(&self){
        self.bind();
        unsafe{
            gl::DrawElements(gl::POINTS,
                             self.num_indices,
                             gl::UNSIGNED_INT,
                             0 as *const gl::types::GLvoid);
        }
    }


}

impl Drop for VertexArray{
    fn drop(&mut self) {
        VertexArray::bind_none();
        unsafe{
            gl::DeleteVertexArrays(1, &mut self.vao);
            println!("Deleting a VAO");
        }
    }
}

