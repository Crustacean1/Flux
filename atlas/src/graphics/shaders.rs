use glad_gl::gl;

use crate::game_root::GameError;
use std::{
    fs::{read_to_string, File},
    io::{self, Read},
    marker::PhantomData,
    mem,
    path::PathBuf,
};

use super::material::TextureMaterial;

#[derive(Clone)]
pub struct ShaderSource {
    pub vertex: Option<PathBuf>,
    pub fragment: Option<PathBuf>,
    pub geometry: Option<PathBuf>,
}

#[derive(Debug, Clone, Copy)]
pub enum ShaderType {
    Vertex,
    Fragment,
    Geometry,
}

#[derive(Clone)]
pub struct ShaderProgram<T: Clone> {
    shader_id: u32,
    phantom: PhantomData<T>,
}

impl<T: Clone> ShaderProgram<T> {
    pub fn load(source: &ShaderSource) -> Result<Self, GameError> {
        let sources = [
            (ShaderType::Vertex, source.vertex.clone()),
            (ShaderType::Fragment, source.fragment.clone()),
            (ShaderType::Geometry, source.geometry.clone()),
        ];

        let sources: Vec<_> = sources
            .iter()
            .filter_map(|(shader_type, source)| Some((*shader_type, source.clone()?)))
            .map(|(shader_type, path)| (shader_type, Self::read_file(&path)))
            .collect();

        let compilations: Vec<_> = sources
            .iter()
            .map(|(shader_type, source)| match source {
                Ok(source) => match Self::compile_shader(*shader_type, source) {
                    Ok(shader) => Ok(shader),
                    Err(e) => Err((shader_type, e)),
                },
                Err(e) => Err((shader_type, e.to_string())),
            })
            .collect();

        let errors: Vec<_> = compilations
            .iter()
            .filter_map(|compilation_result| match compilation_result {
                Err(e) => Some(e),
                _ => None,
            })
            .collect();

        if !errors.is_empty() {
            let error_msg = errors.iter().fold(
                String::new(),
                |error_msg, (shader_type, compilation_error)| {
                    format!(
                        "{}\n---------\n{:?}\nShader compilation failed:\n{}",
                        error_msg, shader_type, compilation_error
                    )
                },
            );

            Err(GameError::new(&error_msg))
        } else {
            let shaders: Vec<_> = compilations
                .iter()
                .filter_map(|compilation| match compilation {
                    Ok(shader) => Some(*shader),
                    Err(_) => None,
                })
                .collect();

            match Self::link_program(&shaders) {
                Ok(shader_program) => Ok(Self {
                    shader_id: shader_program,
                    phantom: PhantomData,
                }),
                Err(e) => Err(GameError::new(&e)),
            }
        }
    }

    pub fn bind(&self) {
        unsafe {
            gl::UseProgram(self.shader_id);
        }
    }

    fn read_file(filepath: &PathBuf) -> Result<Vec<u8>, io::Error> {
        let mut file = File::open(filepath)?;
        let file_size = file.metadata()?.len() as usize;

        let mut file_buffer: Vec<u8> = Vec::with_capacity(file_size);

        file.read_to_end(&mut file_buffer)?;
        file_buffer.push(0);

        Ok(file_buffer)
    }

    fn compile_shader(shader_type: ShaderType, source: &[u8]) -> Result<u32, String> {
        unsafe {
            let shader_type = match shader_type {
                ShaderType::Vertex => gl::VERTEX_SHADER,
                ShaderType::Fragment => gl::FRAGMENT_SHADER,
                ShaderType::Geometry => gl::GEOMETRY_SHADER,
            };

            let shader_id = match gl::CreateShader(shader_type) {
                0 => {
                    return Err(String::from("Failed to create shader"));
                }
                shader => shader,
            };

            let shader_src: *const i8 = mem::transmute(source.as_ptr());

            gl::ShaderSource(shader_id, 1, &shader_src, std::ptr::null());
            gl::CompileShader(shader_id);
            Ok(shader_id)
        }
    }

    fn link_program(shaders: &[u32]) -> Result<u32, String> {
        unsafe {
            let program_id = gl::CreateProgram();

            for shader in shaders {
                gl::AttachShader(program_id, *shader);
            }
            gl::LinkProgram(program_id);

            match Self::check_for_errors(
                program_id,
                gl::LINK_STATUS,
                gl::GetProgramiv,
                gl::GetProgramInfoLog,
            ) {
                Ok(_) => Ok(program_id),
                Err(msg) => Err(format!("Failed to link shader '{}': {}", "standin", msg)),
            }
        }
    }

    pub fn check_for_errors(
        target: u32,
        log_type: u32,
        get_status: unsafe fn(u32, u32, *mut i32),
        get_logs: unsafe fn(u32, i32, *mut i32, *mut i8),
    ) -> Result<(), String> {
        unsafe {
            let mut status: i32 = 0;
            get_status(target, log_type, &mut status);

            if status == 0 {
                let mut err_buff: Vec<u8> = vec![0; 512];
                let mut err_length = 0;

                get_logs(
                    target,
                    err_buff.len() as i32,
                    &mut err_length,
                    mem::transmute(err_buff.get_unchecked_mut(0)),
                );
                return Err(String::from_utf8(err_buff)
                    .expect("Compilation error message should conform to UTF-8"));
            }
        }

        Ok(())
    }

    pub fn load_mvp(&self, mat: &[f32; 16]) {
        unsafe {
            gl::UseProgram(self.shader_id);
            let mvp =
                gl::GetUniformLocation(self.shader_id, mem::transmute("mvp\0".as_ptr()));
            gl::UniformMatrix4fv(mvp, 1, gl::FALSE, mat.as_ptr());
        }
    }
}

#[derive(Clone)]
pub struct UiShader;

impl ShaderProgram<UiShader> {
    pub fn bind_material(&self, material: &TextureMaterial) {
        unsafe {
            gl::UseProgram(self.shader_id);
            let mat_texture =
                gl::GetUniformLocation(self.shader_id, mem::transmute("mat_texture\0".as_ptr()));
            gl::Uniform1ui(mat_texture, material.texture());
        }
    }
}
