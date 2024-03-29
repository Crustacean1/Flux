use std::{path::PathBuf};

use freetype::{face::LoadFlag, Library};
use glad_gl::gl;

use crate::{
    game_root::GameError,
    graphics::{
        material::Material,
        mesh::Mesh,
        texture::{ChannelLayout, Texture},
        vertices::{indices::TriangleGeometry, layouts::P2TVertex},
    },
};

use super::{ResourceLoader, ResourceManager};

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
    pub fn render(&self, text: &str, target: &mut Mesh<P2TVertex, TriangleGeometry>) {
        let (mut x, y) = (0.0, 0.0);
        let char_quads: Vec<_> = text
            .bytes()
            .filter_map(|char| {
                let glyph = self.characters.get(char as usize)?;
                let (width, height) = self.texture_size;
                let (rows, columns) = (8, 16);
                let (column, row) = (char % columns, char / columns);

                let (x1, y1) = (x + glyph.bearing.0 as f32, y - glyph.bearing.1 as f32);
                let (x2, y2) = (x1 + glyph.size.0 as f32, y1 + glyph.size.1 as f32);
                let (u1, v1) = (column as f32 / columns as f32, row as f32 / rows as f32);
                let (u2, v2) = (
                    u1 + (glyph.size.0 as f32 / width as f32),
                    v1 + (glyph.size.1 as f32 / height as f32),
                );

                x += (glyph.advance >> 6) as f32;

                Some([
                    P2TVertex([x1, y1], [u1, v1]),
                    P2TVertex([x2, y1], [u2, v1]),
                    P2TVertex([x2, y2], [u2, v2]),
                    P2TVertex([x1, y2], [u1, v2]),
                ])
            })
            .flatten()
            .collect();

        let char_quads_indices: Vec<_> = text
            .bytes()
            .enumerate()
            .map(|(i, _)| {
                let offset = i as u32 * 4;
                [
                    TriangleGeometry([offset + 1, offset + 0, offset + 2]),
                    TriangleGeometry([offset + 3, offset + 2, offset + 0]),
                ]
            })
            .flatten()
            .collect();

        target.load_vertices(&char_quads);
        target.load_indices(&char_quads_indices);
    }
}

impl Material for Font {
    fn bind(&self) {
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            self.texture.bind()
        }
    }
}

impl ResourceLoader for Font {
    type Resource = Font;

    fn is_resource(path: &PathBuf) -> bool {
        path.extension().map_or(false, |e| e == "font")
    }

    fn load_resource(contents: &[PathBuf]) -> Result<Self::Resource, GameError> {
        let mut freetype_lib =
            Library::init().map_err(|e| GameError::new(&format!("Failed to load font: {}", e)))?;

        let filename = contents
            .iter()
            .find(|f| f.extension().map_or(false, |f| f == "otf" || f == "ttf"))
            .ok_or(GameError::new("No otf or ttf file found"))?;

        load_font_file(filename, &mut freetype_lib)
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
            .map_err(|_e| GameError::new(&format!("Failed to read characters")))?,
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
