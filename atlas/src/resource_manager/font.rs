use std::{fs, path::PathBuf};

use freetype::{face::LoadFlag, Library};
use glad_gl::gl;

use crate::{
    game_root::GameError,
    graphics::{
        primitive::Primitive,
        texture::{ChannelLayout, Texture},
    },
};

use super::{scene_resource_manager::SceneResourceManager, ResourceManager};

#[derive(Clone, Copy)]
pub struct Character {
    pub size: (usize, usize),
    pub bearing: (i32, i32),
    pub advance: usize,
}

#[derive(Clone)]
pub struct Font {
    pub characters: [Character; 128],
    pub texture: Texture,
    texture_size: (usize, usize),
}

impl Default for Font {
    fn default() -> Self {
        todo!()
    }
}

impl Font {
    pub fn render(&self, text: &str, target: &mut Primitive) {
        let (mut x, y) = (0.0, 0.0);
        let char_quads: Vec<_> = text
            .bytes()
            .filter_map(|char| {
                let glyph = self.characters.get(char as usize)?;
                let (width, height) = self.texture_size;
                let (rows, columns) = (8, 16);
                let (column, row) = (char % columns, char / columns);

                if char == 106 {
                    println!(
                        "Size: {} {} pos: {} {} bearing: {} {}",
                        glyph.size.0, glyph.size.1, x, y, glyph.bearing.0, glyph.bearing.1
                    );
                }

                let (x1, y1) = (x + glyph.bearing.0 as f32, y - glyph.bearing.1 as f32);
                let (x2, y2) = (x1 + glyph.size.0 as f32, y1 + glyph.size.1 as f32);
                let (u1, v1) = (column as f32 / columns as f32, row as f32 / rows as f32);
                let (u2, v2) = (
                    u1 + (glyph.size.0 as f32 / width as f32),
                    v1 + (glyph.size.1 as f32 / height as f32),
                );

                x += (glyph.advance >> 6) as f32;

                Some([
                    [x1, y1, u1, v1],
                    [x2, y1, u2, v1],
                    [x2, y2, u2, v2],
                    [x1, y2, u1, v2],
                ])
            })
            .flatten()
            .flatten()
            .collect();

        let char_quads_indices: Vec<_> = text
            .bytes()
            .enumerate()
            .map(|(i, _)| {
                let i = i as u32;
                [
                    i * 4 + 0,
                    i * 4 + 1,
                    i * 4 + 2,
                    i * 4 + 2,
                    i * 4 + 3,
                    i * 4 + 0,
                ]
            })
            .flatten()
            .collect();

        target.reload_vertices(&char_quads);
        target.reload_indices(&char_quads_indices);
    }

    pub fn bind(&self) {
        unsafe { 
            gl::ActiveTexture(gl::TEXTURE0);
            self.texture.bind() 
        }
    }
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
                        font.to_str().map_or(false, |font| font == "ttf")
                    })
                }) {
                    match load_font_file(&font_file.path(), freetype) {
                        Ok(font) => res_man.register(res_id, font),
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
    let Ok(face) = freetype.new_face(path, 0) else {return Err(GameError::new("Couldn't create new font face"))};
    if face.set_pixel_sizes(0, 32).is_err() {
        return GameError::err(format!("Failed to read font file: '{:?}'", path));
    }

    let characters: Vec<_> = (0..128)
        .filter_map(|i| {
            face.load_char(i, LoadFlag::RENDER)
                .map(|_| {
                    let bitmap = Vec::from(face.glyph().bitmap().buffer());

                    let advance = face.glyph().advance().x as usize;
                    let size = (
                        face.glyph().bitmap().width() as usize,
                        face.glyph().bitmap().rows() as usize,
                    );
                    let bearing = (
                        face.glyph().bitmap_left() as i32,
                        face.glyph().bitmap_top() as i32,
                    );

                    (
                        Character {
                            advance,
                            size,
                            bearing,
                        },
                        bitmap,
                    )
                })
                .ok()
        })
        .collect();

    let max_width = characters
        .iter()
        .map(|(char, _)| char.size.0 as usize)
        .max()
        .ok_or(GameError::new("Failed to read max glyph width"))?;

    let max_height = characters
        .iter()
        .map(|(char, _)| char.size.1 as usize)
        .max()
        .ok_or(GameError::new("Failed to read max glyph height"))?;

    let (columns, rows) = (16, 8);
    let (width, height) = (columns * max_width, rows * max_height);
    let mut texture_buffer: Vec<u8> = vec![0; width * height];

    characters
        .iter()
        .enumerate()
        .for_each(|(i, (char, bitmap))| {
            let row = (i / columns) * max_height;
            let column = (i % columns) * max_width;

            write(
                &mut texture_buffer,
                (column, row),
                width,
                bitmap,
                (char.size.0 as usize, char.size.1 as usize),
            );
        });

    let texture = Texture::from_buff(
        texture_buffer.as_slice(),
        ChannelLayout::R8,
        (width as u32, height as u32),
    )?;

    let characters: Vec<_> = characters.iter().map(|(char, _)| *char).collect();

    Ok(Font {
        characters: characters
            .try_into()
            .map_err(|e| GameError::new(&format!("Failed to read characters")))?,
        texture,
        texture_size: (width, height),
    })
}

fn write(
    buffer: &mut [u8],
    (x, y): (usize, usize),
    buffer_width: usize,
    data: &[u8],
    (data_width, data_height): (usize, usize),
) {
    (0..data_width).for_each(|x1| {
        (0..data_height)
            .for_each(|y1| buffer[(y + y1) * buffer_width + (x + x1)] = data[y1 * data_width + x1])
    })
}
