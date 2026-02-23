//! OpenGL shader compile/link helpers.

use gl::types::*;
use std::ffi::CString;

pub fn compile_shader(src: &str, kind: GLenum) -> GLuint {
    unsafe {
        let shader = gl::CreateShader(kind);
        let c_src = CString::new(src).unwrap();
        gl::ShaderSource(shader, 1, &c_src.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);

        let mut ok = 0;
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut ok);
        if ok == 0 {
            let mut len = 0;
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
            let mut buf = vec![0u8; len as usize];
            gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut _,
            );
            eprintln!("Shader compile error:\n{}", String::from_utf8_lossy(&buf));
        }
        shader
    }
}

pub fn create_program(vert_src: &str, frag_src: &str) -> GLuint {
    unsafe {
        let vs = compile_shader(vert_src, gl::VERTEX_SHADER);
        let fs = compile_shader(frag_src, gl::FRAGMENT_SHADER);
        let prog = gl::CreateProgram();
        gl::AttachShader(prog, vs);
        gl::AttachShader(prog, fs);
        gl::LinkProgram(prog);
        gl::DeleteShader(vs);
        gl::DeleteShader(fs);
        prog
    }
}
