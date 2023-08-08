use std::{
    ffi::{CStr, CString},
    fs, mem,
    path::PathBuf,
};

use glad_gl::gl;
use glam::{Mat4, Vec3};

use crate::{
    game_root::GameError,
    resource_manager::{try_get_file, ResourceLoader},
};

pub mod bullet_shader;
pub mod flat_shader;
pub mod mesh_shader;
pub mod particle_shader;
pub mod skybox_shader;
pub mod text_shader;
pub mod sprite_shader;

pub trait ShaderDefinition: Sized {
    type Shader: Shader;
    const EXTENSION: &'static str;

    fn create_shader(&self) -> Self::Shader;
    fn build(shader_id: u32) -> Result<Self, GameError>;
}

pub trait Shader {
    fn shader_id(&self) -> u32;
}

pub trait UniformLoader<T> {
    fn load(&self, uniform: i32, value: T);
}

fn locate_uniform(program: u32, uniform: &str) -> Option<i32> {
    unsafe {
        match gl::GetUniformLocation(program, CString::new(uniform).ok()?.as_ptr()) {
            -1 => None,
            uniform => Some(uniform),
        }
    }
}

fn try_locate_uniform(program: u32, uniform: &str) -> Result<i32, GameError> {
    locate_uniform(program, uniform).ok_or(GameError::uniform(uniform))
}

impl<T: ShaderDefinition + Clone + Default> ResourceLoader for T {
    type Resource = T;

    fn is_resource(path: &std::path::PathBuf) -> bool {
        path.extension().map_or(false, |e| e == T::EXTENSION)
    }

    fn load_resource(entries: &[std::path::PathBuf]) -> Result<Self::Resource, GameError> {
        let extract_filename = |filename: &PathBuf| -> Result<Option<CString>, GameError> {
            let file = fs::read(filename)?;
            let shader =
                CString::new(file).map_err(|_| GameError::new("Failed to convert to C-string"))?;
            Ok(Some(shader))
        };

        let vertex = try_get_file("vertex", entries).map_or(Ok(None), |f| extract_filename(f))?;
        let geometry =
            try_get_file("geometry", entries).map_or(Ok(None), |f| extract_filename(f))?;
        let fragment =
            try_get_file("fragment", entries).map_or(Ok(None), |f| extract_filename(f))?;

        let shader_id = build_shader(vertex, geometry, fragment)?;
        Self::Resource::build(shader_id)
    }
}

impl<Q: Shader> UniformLoader<&Mat4> for Q {
    fn load(&self, uniform: i32, value: &Mat4) {
        unsafe {
            gl::UniformMatrix4fv(uniform, 1, gl::FALSE, value.to_cols_array().as_ptr());
        }
    }
}

impl<Q: Shader> UniformLoader<i32> for Q {
    fn load(&self, uniform: i32, value: i32) {
        unsafe {
            gl::Uniform1i(uniform, value);
        }
    }
}

impl<Q: Shader> UniformLoader<Vec3> for Q {
    fn load(&self, uniform: i32, value: Vec3) {
        unsafe {
            gl::Uniform3f(uniform, value.x, value.y, value.z);
        }
    }
}

pub fn build_shader(
    vertex: Option<CString>,
    geometry: Option<CString>,
    fragment: Option<CString>,
) -> Result<u32, GameError> {
    let vertex = (vertex, gl::VERTEX_SHADER);
    let geometry = (geometry, gl::GEOMETRY_SHADER);
    let fragment = (fragment, gl::FRAGMENT_SHADER);

    let shader_program = create_program()?;

    [vertex, geometry, fragment]
        .iter()
        .filter_map(|(shader, shader_type)| Some(compile(shader.as_ref()?, *shader_type)))
        .fold(Ok(()), |compilation_result: Result<(), GameError>, x| {
            compilation_result?;
            Ok(attach(shader_program, x?))
        })?;

    link_program(shader_program)
}

fn compile(source: &CStr, shader_type: u32) -> Result<u32, GameError> {
    unsafe {
        let shader_id = match gl::CreateShader(shader_type) {
            0 => {
                return Err(GameError::new("Failed to create shader"));
            }
            shader => shader,
        };

        let shader_src: *const i8 = mem::transmute(source.as_ptr());

        gl::ShaderSource(shader_id, 1, &shader_src, std::ptr::null());
        gl::CompileShader(shader_id);

        match check_for_errors(
            shader_id,
            gl::COMPILE_STATUS,
            gl::GetShaderiv,
            gl::GetShaderInfoLog,
        ) {
            Ok(_) => Ok(shader_id),
            Err(msg) => Err(GameError::new(&format!("Compilation failed:\n{}", msg))),
        }
    }
}

fn attach(program: u32, shader: u32) {
    unsafe {
        gl::AttachShader(program, shader);
    }
}

fn create_program() -> Result<u32, GameError> {
    unsafe {
        match gl::CreateProgram() {
            0 => Err(GameError::new("Failed to create shader program")),
            program => Ok(program),
        }
    }
}

fn link_program(program: u32) -> Result<u32, GameError> {
    unsafe {
        gl::LinkProgram(program);

        match check_for_errors(
            program,
            gl::LINK_STATUS,
            gl::GetProgramiv,
            gl::GetProgramInfoLog,
        ) {
            Ok(_) => Ok(program),
            Err(msg) => Err(GameError::new(&format!("Shader linking failed:\n{}", msg))),
        }
    }
}

fn check_for_errors(
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
