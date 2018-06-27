use event::{Event, MouseButton};
use glium::glutin as gl;

/// Persistant state associated with the events. This keeps track of things like which control keys
/// are currently pressed, location of the mouse, and the state of the mouse buttons.
#[derive(Clone, Debug)]
pub struct EventState {
    pub mouse_pos: [i32; 2],
    pub mouse_activity_start: [i32; 2],
    pub mouse_in_window: bool,
    pub mouse_left: MouseButtonState,
    pub mouse_middle: MouseButtonState,
    pub mouse_right: MouseButtonState,
    pub shift_down: bool,
    pub alt_down: bool,
    pub ctrl_down: bool,
    pub logo_down: bool,
    pub hidpi_factor: f64,
}
impl EventState {
    pub fn new() -> Self {
        Self {
            mouse_pos: [0, 0],
            mouse_activity_start: [0, 0],
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
            hidpi_factor: 1.0,
        }
    }

    pub fn is_any_mouse_button_pressed(&self) -> bool {
        self.mouse_left.pressed || self.mouse_middle.pressed || self.mouse_right.pressed
    }
    pub fn get_mouse_pressed_at(&self) -> Option<[i32; 2]> {
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
    pub fn process_event(&mut self, evt: &gl::Event) -> Event {
        Event::from_gl(evt, self)
    }
    pub fn get_mouse_drag_dist(&self) -> Option<[i32; 2]> {
        if let Some(start) = self.get_mouse_pressed_at() {
            let dx = self.mouse_pos[0] - start[0];
            let dy = self.mouse_pos[1] - start[1];
            Some([dx, dy])
        } else {
            None
        }
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
    pub pressed_at: [i32; 2],
    /// Indicates if the user pressed escape while the mouse button was down.
    pub cancelled: bool,
}
impl Default for MouseButtonState {
    fn default() -> MouseButtonState {
        MouseButtonState {
            button: MouseButton::Left,
            pressed: false,
            pressed_at: [0, 0],
            cancelled: false,
        }
    }
}
