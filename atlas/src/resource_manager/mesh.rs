use std::{
    fs::{self, File},
    io::BufReader,
    path::PathBuf,
};

use glam::{Mat4, Vec4};
use gltf::{
    iter::{Buffers, Materials},
    Gltf,
};

use crate::graphics::{
    mesh::Mesh,
    primitive::{MeshIndices, Primitive},
    shaders::mesh_shader::MeshShader, material::phong_material::PhongMaterial,
};

use super::{resource::Resource, scene_resource_manager::SceneResourceManager, ResourceManager};

#[derive(Clone)]
struct MeshNode<'a> {
    children: Vec<&'a MeshNode<'a>>,
}

impl<'a> MeshNode<'a> {
    pub fn new() -> Self {
        MeshNode { children: vec![] }
    }
}

pub fn load_mesh(res_id: &str, dir: &PathBuf, res_man: &mut SceneResourceManager) {
    if let Ok(mut files) = fs::read_dir(dir) {
        if let Some(Ok(mesh_filename)) = files.find(|file_entry| {
            file_entry.as_ref().map_or(false, |file| {
                file.path().extension().map_or(false, |extension| {
                    extension
                        .to_str()
                        .map_or(false, |extension| extension == "glb" || extension == "gltf")
                })
            })
        }) {
            if let Ok(mesh_file) = File::open(mesh_filename.path()) {
                let reader = BufReader::new(mesh_file);
                if let Ok(gltf) = Gltf::from_reader(reader) {
                    res_man.register(res_id, read_mesh(&gltf, dir));
                } else {
                    println!("Mesh failure occured ");
                }
            }
        }
    }

    /*let default_material: Resource<TextureMaterial> = res_man.get("default");

    res_man.register(
        "sample",
        Mesh::<MeshShader, TextureMaterial> {
            primitives: vec![(default_material.res, Primitive::sphere(1.0, 3))],
        },
    );*/
}

fn read_mesh(gltf: &Gltf, root: &PathBuf) -> Mesh<MeshShader, PhongMaterial> {
    let buffers = gltf.buffers();
    let empty = vec![];
    let blob = gltf.blob.as_ref().unwrap_or(&empty);
    let buffers = load_buffers(buffers, blob);
    let materials = load_materials(gltf.materials(), &buffers, root);

    let primitives: Vec<_> = gltf
        .nodes()
        .filter_map(|node| -> Option<Vec<_>> {
            let mesh = node.mesh()?;
            let transform = Mat4::from_cols_array_2d(&node.transform().matrix());

            Some(
                mesh.primitives()
                    .filter_map(|primitive| {
                        let reader = primitive.reader(|buffer| {
                            buffers.get(buffer.index()).map(|buffer| buffer as &[u8])
                        });

                        let positions = reader.read_positions()?;
                        let tex_coords =
                            reader.read_tex_coords(0).map(|coords| coords.into_f32())?;
                        let normals = reader.read_normals()?;
                        let indices = reader.read_indices().map(|indices| indices.into_u32())?;

                        let positions = positions
                            .map(|pos| transform * Vec4::new(pos[0], pos[1], pos[2], 1.0))
                            .map(|vec| [vec[0], vec[1], vec[2]]);

                        let normals = normals
                            .map(|normal| {
                                transform * Vec4::new(normal[0], normal[1], normal[2], 0.0)
                            })
                            .map(|normal| [normal[0], normal[1], normal[2]]);

                        let vertices: Vec<_> = positions
                            .zip(tex_coords)
                            .zip(normals)
                            .map(|((pos, tex), norm)| {
                                [
                                    pos[0], pos[1], pos[2], tex[0], tex[1], norm[0], norm[1],
                                    norm[2],
                                ]
                            })
                            .flatten()
                            .collect();

                        let indices: Vec<_> = indices.collect();

                        let material = if let Some(material) = primitive.material().index() {
                            materials[material].clone()
                        } else {
                            PhongMaterial::default()
                        };

                        Some((
                            material,
                            Primitive::new(
                                &vertices,
                                &[3, 2, 3],
                                &mut MeshIndices::Triangles(indices),
                            ),
                        ))
                    })
                    .collect(),
            )
        })
        .flatten()
        .collect();
    return Mesh { primitives };
}

fn load_buffers(buffers: Buffers, blob: &[u8]) -> Vec<Vec<u8>> {
    buffers
        .map(|buffer| match buffer.source() {
            gltf::buffer::Source::Bin => Vec::from(&blob[0..buffer.length()]),
            gltf::buffer::Source::Uri(uri) => buffer_from_file(uri).unwrap(),
        })
        .collect()
}

fn load_materials(
    materials: Materials,
    _buffers: &[Vec<u8>],
    _root: &PathBuf,
) -> Vec<PhongMaterial> {
    materials.map(|_| PhongMaterial::default()).collect()
}

fn buffer_from_file(uri: &str) -> Option<Vec<u8>> {
    Some(fs::read_to_string(uri).map(|s| Vec::from(s)).ok()?)
}
