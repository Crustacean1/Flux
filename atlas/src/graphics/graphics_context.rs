use std::sync::mpsc::Receiver;

use glad_gl::gl;
use glfw::{Action, Context, Glfw, InitError, WindowEvent};

use crate::{
    event_bus::{EventSender, EventSenderTrait},
    game_root::GameError,
    scene::SceneEvent,
};

pub struct GraphicsContext {
    title: String,
    glfw: Glfw,
    window: glfw::Window,
    event_channel: Receiver<(f64, WindowEvent)>,
    tracked_mouse_pos: (f32, f32),
    tracked_keys: Vec<u8>,
}

impl From<InitError> for GameError {
    fn from(value: InitError) -> Self {
        GameError::new(&value.to_string())
    }
}

#[derive(Clone)]
pub enum IoEvent {
    WindowResized((f32, f32)),
    MouseMotion((f32, f32)),
    LeftMousePress((f32, f32)),
    LeftMouseRelease((f32, f32)),
    RightMousePress((f32, f32)),
    RightMouseRelease((f32, f32)),
    KeyPressed(u8),
    KeyHeld(u8),
    Other,
}

#[derive(Clone)]
pub enum ContextEvent {
    Resized(i32, i32),
    Close,
}

impl GraphicsContext {
    pub fn new(title: &str) -> Result<Self, GameError> {
        let mut glfw = glfw::init(glfw::FAIL_ON_ERRORS)?;

        let (mut window, event_channel) =
            match glfw.create_window(800, 600, title, glfw::WindowMode::Windowed) {
                Some(result) => Ok(result),
                None => Err(GameError::new("Failed to create window")),
            }?;

        glfw.with_primary_monitor(|_glfw, monitor| match monitor {
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
        })?;

        window.make_current();

        window.set_close_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_size_polling(true);
        window.set_mouse_button_polling(true);
        window.set_key_polling(true);

        gl::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

        unsafe {
            gl::ClearColor(0., 0., 0., 1.0);
            gl::PointSize(10.0);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEPTH_TEST);
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }

        let tracked_keys = vec![];

        Ok(GraphicsContext {
            title: String::from(title),
            window,
            event_channel,
            glfw,
            tracked_mouse_pos: (0.0, 0.0),
            tracked_keys,
        })
    }

    pub fn poll_events(&mut self, event_sender: &mut EventSender) {
        self.glfw.poll_events();
        glfw::flush_messages(&self.event_channel).for_each(|(_time, event)| match event {
            WindowEvent::CursorPos(x, y) => {
                self.tracked_mouse_pos = (x as f32, y as f32);
                event_sender.send(IoEvent::MouseMotion((x as f32, y as f32)));
            }
            WindowEvent::MouseButton(glfw::MouseButton::Button1, glfw::Action::Press, _) => {
                event_sender.send(IoEvent::LeftMousePress(self.tracked_mouse_pos));
            }
            WindowEvent::MouseButton(glfw::MouseButton::Button2, glfw::Action::Press, _) => {
                event_sender.send(IoEvent::RightMousePress(self.tracked_mouse_pos));
            }
            WindowEvent::Close => {
                event_sender.send(SceneEvent::Exit);
            }
            WindowEvent::Size(width, height) => {
                event_sender.send(ContextEvent::Resized(width, height));
            }
            WindowEvent::Key(glfw::Key::Escape, _, Action::Release, _) => {
                event_sender.send(ContextEvent::Close);
            }
            WindowEvent::Key(key, _, Action::Press, _) => {
                event_sender.send(IoEvent::KeyPressed(key as u8));
                self.tracked_keys.push(key as u8);
            }
            WindowEvent::Key(key, _, Action::Release, _) => {
                if let Some(tracked_key) = self
                    .tracked_keys
                    .iter()
                    .position(|key_id| *key_id == key as u8)
                {
                    self.tracked_keys.remove(tracked_key);
                }
            }
            _ => {}
        });
        self.tracked_keys
            .iter()
            .for_each(|&key| event_sender.send(IoEvent::KeyHeld(key)))
    }

    pub fn cursor_lock(&mut self, lock: bool) {
        match lock {
            true => self.window.set_cursor_mode(glfw::CursorMode::Disabled),
            false => self.window.set_cursor_mode(glfw::CursorMode::Normal),
        }
    }

    pub fn set_viewport(&self, width: i32, height: i32) {
        unsafe {
            gl::Viewport(0, 0, width, height);
        }
    }

    pub fn display(&mut self) {
        self.window.swap_buffers();
        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn dimensions(&self) -> (i32, i32) {
        self.window.get_size()
    }
}
