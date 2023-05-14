use std::sync::mpsc::Receiver;

use glad_gl::gl;
use glfw::{Context, Glfw, InitError, WindowEvent};

use crate::game_root::GameError;

pub struct GraphicsContext {
    title: String,
    glfw: Glfw,
    window: glfw::Window,
    event_channel: Receiver<(f64, WindowEvent)>,
}

impl From<InitError> for GameError {
    fn from(value: InitError) -> Self {
        GameError::new(&value.to_string())
    }
}

pub enum UserEvent {
    WindowResized(f64, f64),
    MouseMovement(f64, f64),
    LeftMousePress(f64, f64),
    LeftMouseRelease(f64, f64),
    RightMousePress(f64, f64),
    RightMouseRelease(f64, f64),
    Close,
    Other,
}

impl GraphicsContext {
    pub fn new(title: &str) -> Result<Self, GameError> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

        let (mut window, event_channel) =
            match glfw.create_window(800, 600, title, glfw::WindowMode::Windowed) {
                Some(result) => Ok(result),
                None => Err(GameError::new("Failed to create window")),
            }?;

        /*glfw.with_primary_monitor(|_glfw, monitor| match monitor {
            Some(monitor) => {
                if let Some(video_mode) = monitor.get_video_mode() {
                    window.set_monitor(
                        glfw::WindowMode::FullScreen(monitor),
                        0,
                        0,
                        video_mode.width,
                        video_mode.height,
                        Some(60),
                    );
                    Ok(())
                } else {
                    Err(GameError::new("Failed to get video mode"))
                }
            }
            None => Err(GameError::new("Failed to get monitor")),
        })?;*/

        window.make_current();

        window.set_close_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_size_polling(true);
        window.set_mouse_button_polling(true);

        gl::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

        unsafe {
            gl::ClearColor(0.2, 0.2, 0.2, 1.0);
            gl::PointSize(10.0);
        }

        Ok(GraphicsContext {
            title: String::from(title),
            window,
            event_channel,
            glfw,
        })
    }

    pub fn get_events(&mut self) -> impl Iterator<Item = UserEvent> + '_ {
        self.glfw.poll_events();
        glfw::flush_messages(&self.event_channel).map(|(_time, event)| match event {
            WindowEvent::CursorPos(x, y) => UserEvent::MouseMovement(x, y),
            WindowEvent::Close => UserEvent::Close,
            _ => UserEvent::Other,
        })
    }

    pub fn texture_unit_count(&self) -> u32 {
        unsafe {
            let mut texture_unit_count = 0;
            gl::GetIntegerv(gl::MAX_TEXTURE_IMAGE_UNITS, &mut texture_unit_count);
            texture_unit_count as u32
        }
    }

    pub fn display(&mut self) {
        self.window.swap_buffers();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn dimensions(&self) -> (i32, i32) {
        self.window.get_size()
    }
}
