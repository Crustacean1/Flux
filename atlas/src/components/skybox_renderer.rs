use std::{ffi::c_void, mem::size_of, ptr};

use glad_gl::gl;

use crate::graphics::{
    material::{Material, TextureMaterial},
    primitive::Primitive,
    shaders::{ShaderProgram, SkyboxShader},
};

use super::camera::Camera;

pub struct SkyboxRenderer {
    textures: [TextureMaterial; 6],
    mesh: Primitive,
}

pub struct SkyboxSystem {
    shader: ShaderProgram<SkyboxShader>,
}

impl SkyboxRenderer {
    pub fn new(size: f32, textures: &[TextureMaterial]) -> Self {
        let mesh = Primitive::skybox(size);
        SkyboxRenderer {
            mesh,
            textures: [
                textures[0].clone(),
                textures[1].clone(),
                textures[2].clone(),
                textures[3].clone(),
                textures[4].clone(),
                textures[5].clone(),
            ],
        }
    }
}

impl SkyboxSystem {
    pub fn render<'a>(&self, camera: &Camera, skyboxes: &[(usize, *const SkyboxRenderer)]) {
        unsafe {
            /*skyboxes.iter().take(1).for_each(|(_, skybox)| {
                let skybox = &**skybox;

                skybox.mesh.bind();
                skybox.textures.iter().enumerate().for_each(|(i, texture)| {
                    let pv = camera.static_projection_view_mat();
                    self.shader.load_mvp(&pv.to_cols_array());
                    texture.bind(&self.shader);
                    gl::DrawElements(
                        skybox.mesh.primitive_type(),
                        6,
                        gl::UNSIGNED_INT,
                        (i * 6 * size_of::<u32>()) as *const c_void,
                    );
                });
            });*/
        }
    }

    pub fn new(shader: ShaderProgram<SkyboxShader>) -> Self {
        SkyboxSystem { shader }
    }
}
