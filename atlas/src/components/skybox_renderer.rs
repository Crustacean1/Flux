use std::{ffi::c_void, mem::size_of};

use glad_gl::gl;

use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    graphics::{
        material::{skybox_material::SkyboxMaterial, Material},
        primitive::Primitive,
        shaders::{skybox_shader::SkyboxShader, ShaderProgram},
    },
};

use super::camera::Camera;

pub struct SkyboxRenderer {
    material: SkyboxMaterial,
    mesh: Primitive,
}

pub struct SkyboxRendererSystem {
    shader: ShaderProgram<SkyboxShader>,
}

impl SkyboxRenderer {
    pub fn new(size: f32, material: SkyboxMaterial) -> Self {
        let mesh = Primitive::skybox(size);

        SkyboxRenderer { mesh, material }
    }
}

impl SkyboxRendererSystem {
    pub fn render<'a>(&self, camera: &Camera, entity_manager: &EntityManager) {
        unsafe {
            self.shader.bind();
            entity_manager
                .iter()
                .take(1)
                .for_each(|(_, skybox): (usize, &SkyboxRenderer)| {
                    skybox.mesh.bind();
                    skybox.material.bind();

                    let projection_view = camera.static_projection_view_mat();
                    self.shader
                        .bind_projection_view(&projection_view.to_cols_array());

                    (0..6).for_each(|i| {
                        self.shader.bind_billboard(i as i32);

                        gl::DrawElements(
                            skybox.mesh.primitive_type(),
                            6,
                            gl::UNSIGNED_INT,
                            (i * 6 * size_of::<u32>()) as *const c_void,
                        );
                    });
                });
        }
    }

    pub fn new(shader: ShaderProgram<SkyboxShader>) -> Self {
        SkyboxRendererSystem { shader }
    }
}
