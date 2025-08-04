use crate::utils::core::GPUTexture;

pub struct Framebuffer {
    id: u32,
    texture: GPUTexture,
    depth: Option<u32>,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32, depth: bool) -> Self {
        let mut fbo = 0;
        let mut tex = 0;
        let mut depth_buffer = None;
        unsafe {
            gl::GenFramebuffers(1, &mut fbo);
            gl::BindFramebuffer(gl::FRAMEBUFFER, fbo);

            gl::GenTextures(1, &mut tex);
            gl::BindTexture(gl::TEXTURE_2D, tex);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                width as i32,
                height as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                std::ptr::null(),
            );
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as i32);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as i32);
            gl::FramebufferTexture2D(
                gl::FRAMEBUFFER,
                gl::COLOR_ATTACHMENT0,
                gl::TEXTURE_2D,
                tex,
                0,
            );

            if depth {
                let mut depth_buf = 0;
                gl::GenRenderbuffers(1, &mut depth_buf);
                gl::BindRenderbuffer(gl::RENDERBUFFER, depth_buf);
                gl::RenderbufferStorage(
                    gl::RENDERBUFFER,
                    gl::DEPTH_COMPONENT24 as u32,
                    width as i32,
                    height as i32,
                );
                gl::FramebufferRenderbuffer(
                    gl::FRAMEBUFFER,
                    gl::DEPTH_ATTACHMENT,
                    gl::RENDERBUFFER,
                    depth_buf,
                );
                depth_buffer = Some(depth_buf);
            }

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                panic!("Framebuffer is not complete!");
            }

            // Unbind the framebuffer
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        Framebuffer {
            id: fbo,
            texture: GPUTexture(width, height, tex),
            depth: depth_buffer,
        }
    }

    pub fn texture(&self) -> &GPUTexture {
        &self.texture
    }

    pub fn bind(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.id);
        }
    }

    pub fn unbind() {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }
    }
}

impl Drop for Framebuffer {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteFramebuffers(1, &self.id);
            gl::DeleteTextures(1, &self.texture.2);
            if let Some(depth_id) = self.depth {
                gl::DeleteRenderbuffers(1, &depth_id);
            }
        }
    }
}
