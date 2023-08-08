use std::{
    fs::{self, File},
    io::BufReader,
};

use glam::{Mat4, Vec4};
use gltf::{iter::Buffers, Gltf};

use crate::{
    game_root::GameError,
    graphics::{
        material::phong_material::PhongMaterial,
        mesh::Mesh,
        model::Model,
        vertices::{indices::TriangleGeometry, layouts::PTNVertex},
    },
};

use super::ResourceLoader;

impl ResourceLoader for Model {
    type Resource = Model;

    fn is_resource(path: &std::path::PathBuf) -> bool {
        path.extension().map_or(false, |path| path == "mesh")
    }

    fn load_resource(contents: &[std::path::PathBuf]) -> Result<Self::Resource, GameError> {
        let file = contents.iter().find(|entry| {
            entry.is_file()
                && entry
                    .extension()
                    .map_or(false, |ext| ext == "glb" || ext == "gltf")
        });
        let file = file.ok_or(GameError::new(&format!(
            "No file with matching extension found (.glb | .gltf)",
        )))?;

        let mesh_file = File::open(file)?;
        let reader = BufReader::new(mesh_file);

        let gltf = Gltf::from_reader(reader)
            .map_err(|_| GameError::new(&format!("Failed to load '{:?}'", file)))?;
        Ok(read_mesh(&gltf))
    }
}

fn read_mesh(gltf: &Gltf) -> Model {
    let buffers = gltf.buffers();
    let empty = vec![];
    let blob = gltf.blob.as_ref().unwrap_or(&empty);
    let buffers = load_buffers(buffers, blob);
    //let materials = load_materials(gltf.materials(), &buffers, root);

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
                            .map(|((pos, tex), norm)| PTNVertex(pos, tex, norm))
                            .collect();

                        let indices: Vec<_> = indices.collect();
                        let indices: Vec<_> = indices
                            .chunks(3)
                            .filter_map(|chunk| Some(TriangleGeometry(chunk.try_into().ok()?)))
                            .collect();

                        let material = PhongMaterial::default();
                        Some((material, Mesh::new(&vertices, &indices)))
                    })
                    .collect(),
            )
        })
        .flatten()
        .collect();
    return Model { meshes: primitives };
}

fn load_buffers(buffers: Buffers, blob: &[u8]) -> Vec<Vec<u8>> {
    buffers
        .map(|buffer| match buffer.source() {
            gltf::buffer::Source::Bin => Vec::from(&blob[0..buffer.length()]),
            gltf::buffer::Source::Uri(uri) => buffer_from_file(uri).unwrap(),
        })
        .collect()
}

fn buffer_from_file(uri: &str) -> Option<Vec<u8>> {
    Some(fs::read_to_string(uri).map(|s| Vec::from(s)).ok()?)
}
