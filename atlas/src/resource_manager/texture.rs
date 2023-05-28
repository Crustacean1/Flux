use std::{fs, path::PathBuf};

use crate::graphics::material::{TextureMaterial, UiMaterial};

use super::ResourceManager;

pub fn load_mat(
    res_id: &str,
    ext: &str,
    dir: &PathBuf,
    res_man: &mut (impl ResourceManager<TextureMaterial> + ResourceManager<UiMaterial>),
) {
    if let Ok(files) = fs::read_dir(dir) {
        let files: Vec<_> = files
            .filter_map(|file| file.map(|file| Some(file.path())).ok()?)
            .filter(|file| {
                file.extension().map_or(false, |extension| {
                    extension == "jpg" || extension == "jpeg" || extension == "png"
                })
            })
            .collect();

        match ext {
            "ui_mat" => {
                UiMaterial::load(&files)
                    .map(|res| res_man.register(res_id, res))
                    .unwrap();
            }
            "mesh_mat" => {
                TextureMaterial::load(&files)
                    .map(|res| res_man.register(res_id, res))
                    .unwrap();
            }
            _ => {}
        }
    }
}
