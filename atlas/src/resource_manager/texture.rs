use std::{collections::HashMap, path::PathBuf};

use crate::graphics::material::TextureMaterial;

use super::{resource::Resource, scene_resource_manager::LazyResource};

pub fn collect_textures(
    resource_files: Vec<(String, Vec<PathBuf>)>,
) -> HashMap<String, LazyResource<Resource<TextureMaterial>>> {
    let Some((_,texture_resources)) = resource_files
        .iter()
        .find(|(resource_type, _)| resource_type == "materials") else{
        return HashMap::new();
    };

    texture_resources
        .iter()
        .filter(|file| texture_resource_filer(file))
        .filter_map(|path| {
            let filename = String::from(path.file_stem()?.to_str()?);
            println!("Collecting: '{}'", filename);
            Some((filename, LazyResource::Unloaded(path.clone())))
        })
        .collect()
}

fn texture_resource_filer(path: &PathBuf) -> bool {
    let Some(ext) = path.extension() else {return false;};
    ext == "jpg" || ext == "jpeg" || ext == "png"
}
