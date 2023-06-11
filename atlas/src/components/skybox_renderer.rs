use std::{ffi::c_void, mem::size_of};

use glad_gl::gl;
use glam::{Mat4, Vec4};

use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    game_entities::space_box::SpaceBox,
    graphics::{
        material::{skybox_material::SkyboxMaterial, Material},
        primitive::Primitive,
        shaders::{skybox_shader::SkyboxShader, ShaderProgram},
        vertices::layouts::{PTVertex, TriangleGeometry},
    },
};

use super::{camera::Camera, transform::Transform};

pub struct SkyboxRenderer {
    material: SkyboxMaterial,
    mesh: Primitive<PTVertex, TriangleGeometry>,
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

impl<'a> ComponentIteratorGenerator<'a, &'a SkyboxRenderer> for EntityManager {
    fn get_view(&'a self) -> Box<dyn Iterator<Item = &'a SkyboxRenderer> + 'a> {
        let skybox = self
            .iter::<SpaceBox>()
            .map(|space_box| &space_box.entity.renderer)
            .take(1);

        Box::new(skybox)
    }
}

impl SkyboxRendererSystem {
    pub fn render<'a>(
        &self,
        entity_manager: &EntityManager,
        camera: &Camera,
        camera_transform: &Transform,
    ) {
        unsafe {
            self.shader.bind();
            entity_manager
                .get_view()
                .for_each(|skybox: &SkyboxRenderer| {
                    skybox.mesh.bind();
                    skybox.material.bind();

                    let (projection, view) = camera.projection_view(camera_transform);
                    //let projection_view = projection * view;

                    self.shader
                        .bind_projection_view(&projection.to_cols_array(), &view.to_cols_array());

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
