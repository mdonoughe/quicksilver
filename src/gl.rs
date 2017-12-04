extern crate gl;

#[cfg(not(target_arch="wasm32"))]
pub use self::gl::*;


#[cfg(target_arch="wasm32")]
pub use self::gl::{RGBA, DEPTH_BUFFER_BIT, ONE_MINUS_SRC_ALPHA, TEXTURE_MAG_FILTER, TRUE, UNSIGNED_INT, BLEND, FRAGMENT_SHADER, VERTEX_SHADER, LINEAR, RGB, STREAM_DRAW, ARRAY_BUFFER, TEXTURE_MIN_FILTER, ELEMENT_ARRAY_BUFFER, TRIANGLES, FALSE, BGRA, BGR, TEXTURE_WRAP_T, UNSIGNED_BYTE, COLOR_BUFFER_BIT, FLOAT, TEXTURE_WRAP_S, INVALID_VALUE, TEXTURE, COMPILE_STATUS, SRC_ALPHA, CLAMP_TO_EDGE, TEXTURE_2D, TEXTURE0};
#[cfg(target_arch="wasm32")]
use std::os::raw::c_void;
#[cfg(target_arch="wasm32")]
extern "C" {
    pub fn ActiveTexture(tex: u32);
    pub fn AttachShader(program: u32, shader: u32);
    pub fn Clear(buffer: u32);
    pub fn ClearColor(r: f32, g: f32, b: f32, a: f32);
    pub fn CompileShader(shader: u32);
    pub fn CreateShader(shader_type: u32) -> u32;
    pub fn CreateProgram() -> u32;
    pub fn BindBuffer(target: u32, buffer: u32);
    pub fn BindFragDataLocation(program: u32, color_num: u32, name: *const i8);
    pub fn BindTexture(textype: u32, index: u32);
    pub fn BindVertexArray(array: u32);
    pub fn BlendFunc(src: u32, dst: u32);
    pub fn BufferData(target: u32, size: isize, data: *const c_void, usage: u32);
    pub fn BufferSubData(target: u32, offset: i32, size: isize, data: *const c_void);
    pub fn DeleteBuffers(count: u32, buffers: *const u32);
    pub fn DeleteProgram(index: u32);
    pub fn DeleteShader(index: u32);
    pub fn DeleteTextures(index: i32, data: *const u32);
    pub fn DeleteVertexArrays(count: u32, arrays: *const u32);
    pub fn DrawElements(mode: u32, count: i32, elem_type: u32, indices: *const c_void);
    pub fn Enable(feature: u32);
    pub fn EnableVertexAttribArray(index: u32);
    pub fn GenBuffers(count: u32, buffers: *mut u32);
    pub fn GenerateMipmap(enum_type: u32);
    pub fn GenTextures(index: i32, data: *mut u32);
    pub fn GenVertexArrays(count: u32, buffers: *mut u32);
    pub fn GetAttribLocation(program: u32, name: *const i8) -> i32;
    pub fn GetError() -> u32;
    pub fn GetShaderInfoLog(shader: u32, max_length: isize, length: *mut i32, log: *mut i8);
    pub fn GetShaderiv(shader: u32, name: u32, params: *mut i32);
    pub fn GetUniformLocation(program: u32, name: *const i8) -> i32;
    pub fn LinkProgram(shader: u32);
    pub fn ShaderSource(shader: u32, count: isize, string: *const *const i8, length: *const i32);
    pub fn TexImage2D(
        target: u32, 
        level: i32, 
        internalformat: i32, 
        width: i32, 
        height: i32, 
        border: i32, 
        format: u32, 
        type_: u32, 
        pixels: *const c_void
    );
    pub fn TexParameteri(textype: u32, filtertype: u32, value: i32);
    pub fn Uniform1i(location: i32, index: u32);
    pub fn UseProgram(program: u32);
    pub fn VertexAttribPointer(index: u32, size: i32, attr_type: u32, norm: u8, stride: i32, ptr: *const c_void);
    pub fn Viewport(x: i32, y: i32, w: i32, h: i32);
}
