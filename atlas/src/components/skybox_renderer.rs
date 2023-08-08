use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    game_entities::space_box::SpaceBox,
    graphics::{
        context::Context,
        instanced_mesh::InstancedMesh,
        material::{skybox_material::SkyboxMaterial},
        shaders::skybox_shader::{SkyboxShaderDefinition},
        vertices::{
            indices::TriangleGeometry,
            layouts::PTVertex,
            skybox::{self, SkyboxInstance},
        },
    },
};

use super::{camera::Camera, transform::Transform};

pub struct SkyboxRenderer {
    material: SkyboxMaterial,
    mesh: InstancedMesh<SkyboxInstance, PTVertex, TriangleGeometry>,
}

pub struct SkyboxRendererSystem {
    shader: SkyboxShaderDefinition,
}

impl SkyboxRenderer {
    pub fn new(size: f32, material: SkyboxMaterial) -> Self {
        let (vertices, indices, instances) = skybox::skybox(size);
        SkyboxRenderer {
            mesh: InstancedMesh::new(&vertices, &indices, &instances),
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
        context: &mut Context,
        entity_manager: &EntityManager,
        camera: &Camera,
        camera_transform: &Transform,
    ) {
        let skyboxes = entity_manager.get_view();

        context.use_shader(&self.shader, |context| {
            skyboxes.for_each(|skybox: &SkyboxRenderer| {
                context.use_material(&skybox.material, |context| {
                    let (projection, view) = camera.projection_view(camera_transform);

                    context.shader.projection(&projection);
                    context.shader.view(&view);
                    skybox.mesh.render();
                });
            });
        });
    }

    pub fn new(shader: SkyboxShaderDefinition) -> Self {
        SkyboxRendererSystem { shader }
    }
}
