use glam::Mat4;

use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    game_entities::space_box::SpaceBox,
    graphics::{
        instanced_primitive::InstancedPrimitive,
        material::{skybox_material::SkyboxMaterial, Material},
        primitive::Primitive,
        shaders::{skybox_shader::SkyboxShader, ShaderProgram},
        vertices::{
            generator,
            indices::TriangleGeometry,
            layouts::PTVertex,
            skybox::{self, SkyboxInstance},
        },
    },
};

use super::{camera::Camera, transform::Transform};

pub struct SkyboxRenderer {
    material: SkyboxMaterial,
    mesh: InstancedPrimitive<SkyboxInstance, PTVertex, TriangleGeometry>,
}

pub struct SkyboxRendererSystem {
    shader: ShaderProgram<SkyboxShader>,
}

impl SkyboxRenderer {
    pub fn new(size: f32, material: SkyboxMaterial) -> Self {
        let (vertices, indices, instances) = skybox::skybox(size);
        SkyboxRenderer {
            mesh: InstancedPrimitive::new(&vertices, &indices, &instances),
            material,
        }
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
        self.shader.bind();
        entity_manager
            .get_view()
            .for_each(|skybox: &SkyboxRenderer| {
                skybox.material.bind();

                let (projection, view) = camera.projection_view(camera_transform);

                self.shader
                    .bind_projection_view(&projection.to_cols_array(), &view.to_cols_array());
                skybox.mesh.render();
            });
    }

    pub fn new(shader: ShaderProgram<SkyboxShader>) -> Self {
        SkyboxRendererSystem { shader }
    }
}
