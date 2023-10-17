pub mod vao;
pub mod shader;

extern crate gl;
pub struct Buffer{
    buf: gl::types::GLuint
}

impl Drop for Buffer{
    fn drop(&mut self){
        unsafe{
            if self.buf > 0 {
                gl::DeleteBuffers(1, &self.buf);
                println!("Deleting a buffer!");
            }
        }
    }
}

impl Buffer{
    pub fn new() -> Buffer{
        let mut buf: Buffer = Buffer { buf: 0};
        unsafe{
            gl::GenBuffers(1, &mut buf.buf);
        }
        buf
    }

    pub fn bind(&self, target: gl::types::GLenum){
        unsafe{
            gl::BindBuffer(target, self.buf);
        }
    }

    pub fn data(&self, target: gl::types::GLenum, data: *const gl::types::GLvoid, len: usize){
        self.bind(target);
        unsafe{
            gl::BufferData(target, len as gl::types::GLsizeiptr,
                           data, gl::STREAM_DRAW);
        }
    }

    pub fn update_data(&self, target: gl::types::GLenum, data: *const gl::types::GLvoid, len: usize){
        self.bind(target);
        unsafe{
            gl::BufferSubData(target, 0,len as gl::types::GLsizeiptr,
                           data);
        }
    }

    pub fn binding(&self, target: gl::types::GLenum, index: u32){
        unsafe{
            gl::BindBufferBase(target, index, self.buf);
        }
    }

    pub fn bind_none(target: gl::types::GLenum){
        unsafe{
            gl::BindBuffer(target, 0);
        }
    }
}