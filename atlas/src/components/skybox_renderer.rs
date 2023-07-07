use crate::{
    entity_manager::{ComponentIteratorGenerator, EntityManager},
    game_entities::space_box::SpaceBox,
    graphics::{
        instanced_mesh::InstancedMesh,
        material::{skybox_material::SkyboxMaterial, Material},
        shaders::{skybox_shader::SkyboxShader, ShaderProgram},
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
    shader: SkyboxShader,
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

                let pass = self.shader.new_pass(&projection, &view);
                pass.render(&skybox.mesh);
            });
    }

    pub fn new(shader: SkyboxShader) -> Self {
        SkyboxRendererSystem { shader }
    }
}
