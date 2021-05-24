use crate::event::{Event, MouseButton};
use crate::screen_units::Screen2d;
use glium::glutin as gl;
use noisy_float::prelude::*;

/// Persistant state associated with the events. This keeps track of things like which control keys
/// are currently pressed, location of the mouse, and the state of the mouse buttons.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct EventState {
    pub mouse_pos: Screen2d,
    pub mouse_activity_start: Screen2d,
    pub mouse_in_window: bool,
    pub mouse_left: MouseButtonState,
    pub mouse_middle: MouseButtonState,
    pub mouse_right: MouseButtonState,
    pub shift_down: bool,
    pub alt_down: bool,
    pub ctrl_down: bool,
    pub logo_down: bool,
    pub windows: Vec<WindowData>,
    pub(crate) logical_line_height: R32,
}
impl EventState {
    pub fn new(display: &glium::Display) -> Self {
        use glium::backend::Facade;
        let hidpi_factor = display.gl_window().window().scale_factor() as f32;
        let win_id = display.gl_window().window().id();
        let win_dim = display.get_context().get_framebuffer_dimensions();
        let win_dim = Screen2d::from_physical_u32([win_dim.0, win_dim.1], hidpi_factor);

        Self {
            mouse_pos: Screen2d::from_logical([0.0, 0.0], hidpi_factor),
            mouse_activity_start: Screen2d::from_logical([0.0, 0.0], hidpi_factor),
            mouse_in_window: false,
            mouse_left: MouseButtonState {
                button: MouseButton::Left,
                ..Default::default()
            },
            mouse_middle: MouseButtonState {
                button: MouseButton::Middle,
                ..Default::default()
            },
            mouse_right: MouseButtonState {
                button: MouseButton::Right,
                ..Default::default()
            },
            shift_down: false,
            alt_down: false,
            ctrl_down: false,
            logo_down: false,
            windows: vec![WindowData::new(win_id, win_dim, hidpi_factor)],
            logical_line_height: r32(18.0),
        }
    }

    pub fn primary_win_dim(&self) -> Screen2d {
        self.windows
            .iter()
            .nth(0)
            .map(|w| w.dim)
            .unwrap_or(Screen2d::from_logical([0.0, 0.0], 1.0))
    }

    pub fn hidpi_factor(&self) -> f32 {
        self.hidpi_factor_r32().raw()
    }
    pub(crate) fn hidpi_factor_r32(&self) -> R32 {
        self.windows
            .iter()
            .nth(0)
            .map(|w| w.hidpi_factor)
            .unwrap_or(r32(1.0))
    }
    pub(crate) fn get_or_create_win<'a>(&'a mut self, id: crate::WindowId) -> &'a mut WindowData {
        let idx = self
            .windows
            .iter()
            .enumerate()
            .find(|(_, w)| w.id == id)
            .map(|(idx, _)| idx);
        if let Some(idx) = idx {
            &mut self.windows[idx]
        } else {
            let idx = self.windows.len();
            let hidpi_factor = self.hidpi_factor_r32();
            self.windows.push(WindowData {
                id,
                dim: Screen2d::from_logical_r32([r32(0.0), r32(0.0)], hidpi_factor),
                hidpi_factor,
            });
            &mut self.windows[idx]
        }
    }
    pub(crate) fn window_destroyed(&mut self, id: crate::WindowId) {
        self.windows.retain(|w| w.id != id);
    }

    pub fn logical_line_height(&self) -> f32 {
        self.logical_line_height.raw()
    }
    pub fn set_logical_line_height(&mut self, h: f32) {
        assert!(
            h >= 1.0,
            "Line height must be at least 1 logical pixel high"
        );
        self.logical_line_height = r32(h);
    }

    pub fn is_any_mouse_button_pressed(&self) -> bool {
        self.mouse_left.pressed || self.mouse_middle.pressed || self.mouse_right.pressed
    }
    pub fn get_mouse_pressed_at(&self) -> Option<Screen2d> {
        if self.mouse_left.pressed {
            return Some(self.mouse_left.pressed_at);
        }
        if self.mouse_middle.pressed {
            return Some(self.mouse_middle.pressed_at);
        }
        if self.mouse_right.pressed {
            return Some(self.mouse_right.pressed_at);
        }
        None
    }
    pub fn process_event<T>(&mut self, evt: &gl::event::Event<T>) -> Event {
        Event::from_gl(evt, self)
    }
    pub fn get_mouse_drag_dist(&self) -> Option<Screen2d> {
        if let Some(start) = self.get_mouse_pressed_at() {
            Some(self.mouse_pos - start)
        } else {
            None
        }
    }
    /// Distance from the last mouse click (left button).
    pub fn mouse_left_dist(&self) -> Screen2d {
        self.mouse_pos - self.mouse_left.pressed_at
    }
    /// Distance from the last mouse click (middle button).
    pub fn mouse_middle_dist(&self) -> Screen2d {
        self.mouse_pos - self.mouse_middle.pressed_at
    }
    /// Distance from the last mouse click (right button).
    pub fn mouse_right_dist(&self) -> Screen2d {
        self.mouse_pos - self.mouse_right.pressed_at
    }
}

/// Current state of the specified mouse button.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub struct MouseButtonState {
    /// Which mouse button this is for.
    pub button: MouseButton,
    /// Indicates if the button is currently being pressed down by the user.
    pub pressed: bool,
    /// Location where the mouse button was last pressed.
    pub pressed_at: Screen2d,
    /// Indicates if the user pressed escape while the mouse button was down.
    pub cancelled: bool,
}
impl Default for MouseButtonState {
    fn default() -> MouseButtonState {
        MouseButtonState {
            button: MouseButton::Left,
            pressed: false,
            pressed_at: Screen2d::from_logical([0.0, 0.0], 1.0),
            cancelled: false,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct WindowData {
    pub id: crate::WindowId,
    pub dim: Screen2d,
    pub(crate) hidpi_factor: R32,
}
impl WindowData {
    pub fn new(id: crate::WindowId, dim: Screen2d, hidpi_factor: f32) -> Self {
        assert!(hidpi_factor > 0.0, "HiDPI factor must be greater than zero");
        let hidpi_factor = r32(hidpi_factor);
        Self {
            id,
            dim,
            hidpi_factor,
        }
    }
    pub fn hidpi_factor(&self) -> f32 {
        self.hidpi_factor.raw()
    }
}
