use glam::Mat4;

use super::vertices::Vertex;

trait Shader<VertexType: Vertex, MaterialType> {
    fn bind();
    fn load_material(material: MaterialType);
    fn load_model_mat(mat: Mat4);
    fn load_vp_mat(mat: Mat4);
}

pub struct UiShader {
    program_id: u32,
}
