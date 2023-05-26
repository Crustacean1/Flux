use std::ptr;

use glad_gl::gl;

use crate::graphics::{
    material::TextureMaterial,
    mesh::Primitive,
    shaders::{ShaderProgram, SkyboxShader},
    vertices::base_vertices::{TriangleIndex, Vertex3PT},
};

use super::{camera::Camera, transform::Transform};

type MeshType = Primitive<Vertex3PT, TriangleIndex>;

pub struct MeshRenderer {
    pub mesh: MeshType,
    pub material: TextureMaterial,
}

pub struct MeshRendererSystem {
    shader: ShaderProgram<SkyboxShader>,
}

impl MeshRendererSystem {
    pub fn new(shader: ShaderProgram<SkyboxShader>) -> Self {
        MeshRendererSystem { shader }
    }

    pub fn render(
        &self,
        camera: &Camera,
        meshes: &[((usize, *const Transform), *const MeshRenderer)],
    ) {
        let view_projection = camera.projection_view_mat();
        meshes
            .iter()
            .for_each(|((_, transform), mesh_renderer)| unsafe {
                let mesh_renderer = &**mesh_renderer;
                let transform = &**transform;

                let mvp = view_projection * transform.model();
                self.shader.load_mvp(&mvp.to_cols_array());
                mesh_renderer.mesh.bind();
                mesh_renderer.material.bind();
                self.shader.bind_material(&mesh_renderer.material);

                gl::DrawElements(
                    mesh_renderer.mesh.primitive_type(),
                    mesh_renderer.mesh.count() as i32,
                    gl::UNSIGNED_INT,
                    ptr::null(),
                );
            })
    }
}
