use std::{
    fs::{self, File},
    io::{BufReader, Read},
    mem,
    path::PathBuf,
};

use glad_gl::gl;

use crate::{
    game_root::GameError,
    graphics::shaders::{
        bullet_shader::BulletShader, flat_shader::FlatShader, mesh_shader::MeshShader,
        particle_shader::ParticleShader, skybox_shader::SkyboxShader, text_shader::TextShader,
        ui_shader::SpriteShader, Shader, ShaderProgram, ShaderType,
    },
};

use super::{scene_resource_manager::SceneResourceManager, ResourceManager};

pub struct ShaderUnit {
    shader_id: u32,
}

impl ShaderUnit {
    pub fn compile(path: &PathBuf) -> Result<Self, GameError> {
        let filename = path
            .file_name()
            .ok_or(GameError::new("Failed to read shader filename"))?
            .to_str()
            .ok_or(GameError::new("Failed to parse shader filename"))?;

        let unit_type = Self::unit_type(path)?;
        let content = Self::read_file(path)?;
        let shader_id = Self::compile_shader(content, filename, unit_type)?;

        Ok(ShaderUnit { shader_id })
    }

    pub fn link<T>(shader_name: &str, shaders: &[ShaderUnit]) -> Result<ShaderProgram<T>, GameError>
    where
        T: Clone + Shader<T>,
    {
        unsafe {
            let program_id = gl::CreateProgram();

            shaders
                .iter()
                .for_each(|shader| gl::AttachShader(program_id, shader.shader_id));

            gl::LinkProgram(program_id);

            match Self::check_for_errors(
                program_id,
                gl::LINK_STATUS,
                gl::GetProgramiv,
                gl::GetProgramInfoLog,
            ) {
                Ok(_) => Ok(ShaderProgram::<T>::build(program_id)?),
                Err(msg) => Err(GameError::new(&format!(
                    "Failed to link shader '{}': {}",
                    shader_name, msg
                ))),
            }
        }
    }

    fn compile_shader(
        content: Vec<u8>,
        name: &str,
        unit_type: ShaderType,
    ) -> Result<u32, GameError> {
        unsafe {
            let shader_type = unit_type.to_gl();

            let shader_id = match gl::CreateShader(shader_type) {
                0 => {
                    return Err(GameError::new("Failed to create shader"));
                }
                shader => shader,
            };

            let shader_src: *const i8 = mem::transmute(content.as_ptr());

            gl::ShaderSource(shader_id, 1, &shader_src, std::ptr::null());
            gl::CompileShader(shader_id);

            match Self::check_for_errors(
                shader_id,
                gl::COMPILE_STATUS,
                gl::GetShaderiv,
                gl::GetShaderInfoLog,
            ) {
                Ok(_) => Ok(shader_id),
                Err(msg) => Err(GameError::new(&format!(
                    "Compilation failed '{}':\n{}",
                    name, msg
                ))),
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

    fn unit_type(path: &PathBuf) -> Result<ShaderType, GameError> {
        if let Some(filename) = path.file_name() {
            match filename.to_str() {
                Some("vertex") => Ok(ShaderType::Vertex),
                Some("geometry") => Ok(ShaderType::Geometry),
                Some("fragment") => Ok(ShaderType::Fragment),
                _ => Err(GameError::new(&format!(
                    "Invalid shader type: '{:?}'",
                    path
                ))),
            }
        } else {
            Err(GameError::new(&format!(
                "Failed to read shader_unit filename at path: '{:?}'",
                path
            )))
        }
    }

    fn read_file(path: &PathBuf) -> Result<Vec<u8>, GameError> {
        let file = File::open(path)?;
        let mut buffer = BufReader::new(file);

        let mut buff = vec![];
        buffer.read_to_end(&mut buff)?;
        buff.push(0);
        Ok(buff)
    }
}

pub fn load_shader(res_id: &str, ext: &str, path: &PathBuf, res_man: &mut SceneResourceManager) {
    if let Ok(shader_files) = fs::read_dir(path) {
        if ![
            "mesh_shader",
            "ui_shader",
            "sky_shader",
            "text_shader",
            "particle_shader",
            "bullet_shader",
            "flat_shader",
        ]
        .contains(&ext)
        {
            return;
        }

        let shader_units =
            shader_files.fold(Ok(vec![]), |shaders: Result<_, GameError>, shader_unit| {
                let mut shaders = shaders?;
                shaders.push(ShaderUnit::compile(&shader_unit?.path())?);
                Ok(shaders)
            });

        match shader_units.map(|units| link_shader(res_id, ext, &units, res_man)) {
            Err(e) => {
                println!("Shader compilation failed:\n\t{}", e);
            }
            Ok(Err(e)) => {
                println!("Shader compilation failed:\n\t{}", e);
            }
            _ => {}
        }
    }
}

fn link_shader(
    res_id: &str,
    ext: &str,
    shader_units: &[ShaderUnit],
    res_man: &mut SceneResourceManager,
) -> Result<(), GameError> {
    match ext {
        "ui_shader" => res_man.register(
            res_id,
            ShaderUnit::link::<SpriteShader>(res_id, &shader_units)?,
        ),
        "sky_shader" => res_man.register(
            res_id,
            ShaderUnit::link::<SkyboxShader>(res_id, &shader_units)?,
        ),
        "mesh_shader" => res_man.register(
            res_id,
            ShaderUnit::link::<MeshShader>(res_id, &shader_units)?,
        ),
        "text_shader" => res_man.register(
            res_id,
            ShaderUnit::link::<TextShader>(res_id, &shader_units)?,
        ),
        "particle_shader" => res_man.register(
            res_id,
            ShaderUnit::link::<ParticleShader>(res_id, &shader_units)?,
        ),
        "bullet_shader" => res_man.register(
            res_id,
            ShaderUnit::link::<BulletShader>(res_id, &shader_units)?,
        ),
        "flat_shader" => res_man.register(
            res_id,
            ShaderUnit::link::<FlatShader>(res_id, &shader_units)?,
        ),
        _ => {}
    };
    Ok(())
}
