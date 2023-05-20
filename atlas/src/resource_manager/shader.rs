use std::{collections::HashMap, path::PathBuf};

use crate::graphics::shaders::ShaderSource;

pub fn index_shaders(files: &Vec<PathBuf>) -> HashMap<String, ShaderSource> {
    let shader_files: Vec<PathBuf> = files
        .iter()
        .filter(|path| shader_resource_filter(path))
        .map(|path| path.clone())
        .collect();

    group_shaders(shader_files)
}

fn shader_resource_filter(path: &PathBuf) -> bool {
    let Some(ext) = path.extension() else {return false};
    ext == "vs" || ext == "fs" || ext == "gs"
}

fn group_shaders(shaders_sources: Vec<PathBuf>) -> HashMap<String, ShaderSource> {
    let mut shader_map = HashMap::<String, ShaderSource>::new();

    shaders_sources.iter().for_each(|shader_path| {
        let Some(ext) = shader_path.extension() else {return;};
        let Some(basename) = shader_path.file_stem() else {return;};
        let basename = basename.to_str().unwrap();

        if !shader_map.contains_key(basename) {
            shader_map.insert(
                String::from(basename),
                ShaderSource {
                    vertex: None,
                    fragment: None,
                    geometry: None,
                },
            );
        }

        let shader_source = shader_map.get_mut(basename).unwrap();

        match ext.to_str() {
            Some("vs") => shader_source.vertex = Some(shader_path.clone()),
            Some("fs") => shader_source.fragment = Some(shader_path.clone()),
            Some("gs") => shader_source.vertex = Some(shader_path.clone()),
            _ => {}
        }
    });

    shader_map
}
