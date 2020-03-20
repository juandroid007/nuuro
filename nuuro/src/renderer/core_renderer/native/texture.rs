use std::os::raw::c_void;
use std::path::Path;

use gl;
use gl::types::*;
use image::GenericImageView;

pub struct Texture(GLuint);

impl Texture {
    pub fn new(path: &str) -> Self {
        let mut texture: GLuint = 0;

        // load image, create texture and generate mipmaps
        let img = image::open(&Path::new(path)).expect("Failed to load texture");
        let img_ptr: *const c_void = img.to_rgba().into_raw().as_ptr() as *const c_void;

        unsafe {
            gl::GenTextures(1, &mut texture);

            // all upcoming GL_TEXTURE_2D operations now have effect on this texture object
            gl::BindTexture(gl::TEXTURE_2D, texture);

            // set the texture wrapping parameters
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::REPEAT as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::REPEAT as i32);
        }

        unsafe {
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGB as i32,
                img.width() as i32,
                img.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img_ptr,
            );
            // gl::GenerateMipmap(gl::TEXTURE_2D);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as GLint);

            // Unbind texture
            gl::BindTexture(gl::TEXTURE_2D, 0);
        }

        Texture(texture)
    }

    pub fn gl_bind_texture(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, self.0) }
    }

    pub fn gl_unbind_texture(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0) }
    }
}
