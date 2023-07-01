use std::sync::mpsc::Receiver;

use glad_gl::gl;
use glfw::{Action, Context, Glfw, InitError, WindowEvent};

use crate::{event_bus::EventSender, game_root::GameError, scene::SceneEvent};

pub struct GraphicsContext {
    title: String,
    glfw: Glfw,
    window: glfw::Window,
    event_channel: Receiver<(f64, WindowEvent)>,
    tracked_mouse_pos: (f32, f32),
}

impl From<InitError> for GameError {
    fn from(value: InitError) -> Self {
        GameError::new(&value.to_string())
    }
}

#[derive(Clone)]
pub enum IoEvent {
    WindowResized((f32, f32)),
    MousePositionChange((f32, f32)),
    MouseMotion((f32, f32)),
    LeftMousePress((f32, f32)),
    LeftMouseRelease((f32, f32)),
    RightMousePress((f32, f32)),
    RightMouseRelease((f32, f32)),
    KeyPressed(char),
    KeyReleased(char),
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

        let (mut window, event_channel) = glfw
            .with_primary_monitor(|glfw, monitor| {
                glfw.create_window(1920, 1080, title, glfw::WindowMode::FullScreen(monitor?))
            })
            .ok_or(GameError::new("Failed to initialize window"))?;

        window.make_current();

        window.set_close_polling(true);
        window.set_cursor_pos_polling(true);
        window.set_size_polling(true);
        window.set_mouse_button_polling(true);
        window.set_key_polling(true);

        //glfw.set_swap_interval(glfw::SwapInterval::None);

        gl::load(|e| glfw.get_proc_address_raw(e) as *const std::os::raw::c_void);

        unsafe {
            gl::ClearColor(0., 0., 0., 1.0);
            gl::PointSize(5.0);
            gl::Enable(gl::BLEND);
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::DEPTH_TEST);
            gl::Enable(gl::CULL_FACE);
            //gl::CullFace(gl::FRONT);
            gl::PixelStorei(gl::UNPACK_ALIGNMENT, 1);
        }

        Ok(GraphicsContext {
            title: String::from(title),
            window,
            event_channel,
            glfw,
            tracked_mouse_pos: (0.0, 0.0),
        })
    }

    pub fn poll_events(&mut self, event_sender: &mut EventSender) {
        self.glfw.poll_events();
        glfw::flush_messages(&self.event_channel).for_each(|(_time, event)| match event {
            WindowEvent::CursorPos(x, y) => {
                let (prev_x, prev_y) = self.tracked_mouse_pos;
                event_sender.write(IoEvent::MouseMotion((x as f32 - prev_x, y as f32 - prev_y)));
                self.tracked_mouse_pos = (x as f32, y as f32);
                event_sender.write(IoEvent::MousePositionChange((x as f32, y as f32)));
            }
            WindowEvent::MouseButton(glfw::MouseButton::Button1, glfw::Action::Press, _) => {
                event_sender.write(IoEvent::LeftMousePress(self.tracked_mouse_pos));
            }
            WindowEvent::MouseButton(glfw::MouseButton::Button2, glfw::Action::Press, _) => {
                event_sender.write(IoEvent::RightMousePress(self.tracked_mouse_pos));
            }
            WindowEvent::Close => {
                event_sender.write(SceneEvent::Exit);
            }
            WindowEvent::Size(width, height) => {
                event_sender.write(ContextEvent::Resized(width, height));
            }
            WindowEvent::Key(glfw::Key::Escape, _, Action::Release, _) => {
                event_sender.write(ContextEvent::Close);
            }
            WindowEvent::Key(key, _, Action::Press, _) => {
                event_sender.write(IoEvent::KeyPressed(key as u8 as char));
            }
            WindowEvent::Key(key, _, Action::Release, _) => {
                event_sender.write(IoEvent::KeyReleased(key as u8 as char));
            }
            _ => {}
        });
    }

    pub fn cursor_lock(&mut self, lock: bool) {
        match lock {
            true => self.window.set_cursor_mode(glfw::CursorMode::Disabled),
            false => self.window.set_cursor_mode(glfw::CursorMode::Normal),
        }
    }

    pub fn depth_write(&self, enabled: bool) {
        unsafe {
            match enabled {
                true => gl::DepthMask(gl::TRUE),
                false => gl::DepthMask(gl::FALSE),
            }
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
