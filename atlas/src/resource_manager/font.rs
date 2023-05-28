use std::{fs, path::PathBuf};

use freetype::{face::LoadFlag, Library};
use glam::Vec2;

use crate::{
    game_root::GameError,
    graphics::texture::{ChannelLayout, Texture},
};

use super::{scene_resource_manager::SceneResourceManager, ResourceManager};

#[derive(Clone, Copy)]
struct Character {
    texture: Texture,
    size: Vec2,
    bearing: Vec2,
    advance: f32,
}

pub struct Font {
    characters: [Character; 128],
}

pub fn load_font(
    res_id: &str,
    ext: &str,
    dir: &PathBuf,
    freetype: &mut Library,
    res_man: &mut SceneResourceManager,
) {
    match ext {
        "font" => {
            if let Ok(font_files) = fs::read_dir(dir) {
                if let Some(font_file) = font_files.filter_map(|font| font.ok()).find(|font| {
                    font.path().extension().map_or(false, |font| {
                        font.to_str().map_or(false, |font| font == "otf")
                    })
                }) {
                    match load_font_file(&font_file.path(), freetype) {
                        Ok(_) => {}
                        Err(e) => {
                            println!("Failed to load font:\n{}", e);
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

fn load_font_file(path: &PathBuf, freetype: &mut Library) -> Result<Font, GameError> {
    println!("Loading font: {:?}", path);
    let Ok(face) = freetype.new_face(path, 0) else {return Err(GameError::new("Couldn't create new font face"))};
    let default_texture = Texture::from_color((0.0, 0.0, 0.0));
    let mut characters = [Character {
        texture: default_texture.clone(),
        size: Vec2::new(0.0, 0.0),
        bearing: Vec2::new(0.0, 0.0),
        advance: 0.0,
    }; 128];

    characters
        .iter_mut()
        .enumerate()
        .for_each(|(i, character)| match face.load_char(i, LoadFlag::RENDER) {
            Ok(_) => {
                let bitmap = face.glyph().bitmap();
                match Texture::from_buff(
                    bitmap.buffer(),
                    ChannelLayout::R8,
                    (bitmap.width() as u32, bitmap.rows() as u32),
                ) {
                    Ok(texture) => {
                        character.texture = texture;
                        character.advance = face.glyph().advance().x as f32;
                        character.size = Vec2::new(
                            face.glyph().bitmap().width() as f32,
                            face.glyph().bitmap().rows() as f32,
                        );
                        character.bearing = Vec2::new(
                            face.glyph().bitmap_left() as f32,
                            face.glyph().bitmap_top() as f32,
                        );
                    }
                    _ => {
                        println!("Failed to convert to bitmap");
                    }
                }
            }
            _ => {}
        });

    Ok(Font { characters })
}
