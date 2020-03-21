use std::os::raw::c_void;
use std::path::Path;

use gl::types::*;
use image::{DynamicImage, GenericImageView};

pub struct Texture(GLuint);

impl Texture {
    pub fn new(path: &str) -> Self {
        let mut texture: GLuint = 0;

        let (img, width, height) = match image::open(&Path::new(path)) {
            Err(err) => panic!("Failed to load texure: {:?}", err),
            Ok(img) => {
                println!("Dimensions of image are {:?}", img.dimensions());

                let (width, height) = img.dimensions();

                let img = match img {
                    DynamicImage::ImageRgba8(img) => img,
                    img => img.to_rgba(),
                };

                (img, width, height)
            }
        };

        let img_data = img.into_raw();
        let img_ptr: *const c_void = img_data.as_ptr() as *const c_void;
        // let img_ptr: *const c_void = img.to_rgba().into_raw().as_ptr() as *const c_void;

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
                gl::RGBA as i32,
                width as i32,
                height as i32,
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
        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, self.0);
        }
    }

    pub fn gl_unbind_texture(&self) {
        unsafe { gl::BindTexture(gl::TEXTURE_2D, 0) }
    }
}
