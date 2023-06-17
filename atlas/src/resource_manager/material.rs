use std::{fs, path::PathBuf};

use crate::graphics::material::{
    phong_material::PhongMaterial, skybox_material::SkyboxMaterial, sprite_material::SpriteMaterial, particle_material::ParticleMaterial,
};

use super::{scene_resource_manager::SceneResourceManager, ResourceManager};

pub fn load_mat(res_id: &str, ext: &str, dir: &PathBuf, res_man: &mut SceneResourceManager) {
    if let Ok(files) = fs::read_dir(dir) {
        let files: Vec<_> = files
            .filter_map(|file| file.map(|file| Some(file.path())).ok()?)
            .filter(|file| {
                file.extension().map_or(false, |extension| {
                    extension == "jpg" || extension == "jpeg" || extension == "png"
                })
            })
            .collect();

        let result = match ext {
            "sprite" => SpriteMaterial::load(&files).map(|res| res_man.register(res_id, res)),
            "mat" => PhongMaterial::load(&files).map(|res| res_man.register(res_id, res)),
            "skybox" => SkyboxMaterial::load(&files).map(|res| res_man.register(res_id, res)),
            "particle" => ParticleMaterial::load(&files).map(|res| res_man.register(res_id, res)),
            _ => Ok(()),
        };
        if let Err(e) = result {
            println!("Material loading '{}' failed:\n{}", res_id, e);
        }
    }
}
