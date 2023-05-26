use std::{
    collections::HashMap,
    fs::{self, File},
    io::BufReader,
    iter::repeat,
    path::PathBuf,
};

use glam::{Mat4, Vec4};
use gltf::{iter::Buffers, Gltf};

use crate::{
    game_root::GameError,
    graphics::{
        mesh::Primitive,
        vertices::base_vertices::{TriangleIndex, Vertex3PT},
    },
};

#[derive(Clone)]
struct MeshNode<'a> {
    children: Vec<&'a MeshNode<'a>>,
}

impl<'a> MeshNode<'a> {
    pub fn new() -> Self {
        MeshNode { children: vec![] }
    }
}

pub fn collect_mesh(
    index: &Vec<(String, Vec<PathBuf>)>,
) -> HashMap<String, Primitive<Vertex3PT, TriangleIndex>> {
    let Some((_,meshes)) = index.iter().find(|(res_type, _)| res_type == "meshes") else{
        return HashMap::new();
    };

    meshes
        .iter()
        .filter(|path| {
            path.extension().map_or(false, |path| {
                path.to_str().map_or(false, |path| path == "glb")
            })
        })
        .map(|path| {
            if let Ok(file) = File::open(path) {
                let reader = BufReader::new(file);
                if let Ok(gltf) = Gltf::from_reader(reader) {
                    load_mesh(&gltf)
                } else {
                    vec![]
                }
            } else {
                vec![]
            }
        })
        .flatten()
        .map(|(name, mesh)| (name.clone(), mesh))
        .collect()
}

fn load_mesh(gltf: &Gltf) -> Vec<(String, Primitive<Vertex3PT, TriangleIndex>)> {
    let buffers = gltf.buffers();
    let empty = vec![];
    let blob = gltf.blob.as_ref().unwrap_or(&empty);
    let buffers = load_buffers(buffers, blob);

    println!("Mesh node definition:");
    gltf.nodes()
        .for_each(|node| println!("\tNode: {}", node.name().unwrap_or("Unnamed node")));

    gltf.nodes()
        .filter_map(|node| -> Option<Vec<_>> {
            let mesh = node.mesh()?;
            let mesh_name = String::from(mesh.name().unwrap_or("UMO"));
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
                        let mut indices =
                            reader.read_indices().map(|indices| indices.into_u32())?;

                        let vertices: Vec<_> = positions
                            .zip(tex_coords)
                            .map(|(pos, tex)| {
                                (transform * Vec4::new(pos[0], pos[1], pos[2], 1.0), tex)
                            })
                            .map(|(pos, tex)| Vertex3PT {
                                pos: [pos.x, pos.y, pos.z],
                                tex,
                            })
                            .collect();

                        let indices: Vec<_> = repeat(())
                            .map_while(|_| {
                                Some(TriangleIndex {
                                    triangle: [indices.next()?, indices.next()?, indices.next()?],
                                })
                            })
                            .collect();

                        Some((mesh_name.clone(), Primitive::new(&vertices, &indices)))
                    })
                    .collect(),
            )
        })
        .flatten()
        .collect()
}

fn load_buffers(buffers: Buffers, blob: &[u8]) -> Vec<Vec<u8>> {
    buffers
        .map(|buffer| match buffer.source() {
            gltf::buffer::Source::Bin => Vec::from(blob),
            gltf::buffer::Source::Uri(uri) => buffer_from_file(uri).unwrap(),
        })
        .collect()
}

fn buffer_from_file(uri: &str) -> Result<Vec<u8>, GameError> {
    Ok(fs::read_to_string(uri).map(|s| Vec::from(s))?)
}
