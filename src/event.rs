// Event Definitions:
//   https://github.com/rust-windowing/winit/blob/master/src/event.rs

use crate::event_state::{EventState, MouseButtonState};
use crate::screen_units::Screen2d;
use glium::glutin as gl;
use glium::glutin::{DeviceId, VirtualKeyCode, WindowId};
use noisy_float::prelude::*;
use std::path::PathBuf;

pub type AxisId = u32;
pub type ScanCode = u32;
pub type ButtonId = u32;
pub type FingerId = u64;

/// An event from OpenGL. This is a simplified version of the events provided by winit.
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    AppAwaken,
    AppResume,
    AppSuspend,

    WindowResize {
        win_id: WindowId,
        size: Screen2d,
    },
    WindowMove {
        win_id: WindowId,
        pos: Screen2d,
    },
    WindowClose {
        win_id: WindowId,
    },
    WindowDestroyed {
        win_id: WindowId,
    },
    WindowRefresh {
        win_id: WindowId,
    },
    WindowFocus {
        win_id: WindowId,
    },
    WindowBlur {
        win_id: WindowId,
    },

    FileDrop {
        win_id: WindowId,
        path: PathBuf,
    },
    FileHover {
        win_id: WindowId,
        path: PathBuf,
    },
    FileCancel {
        win_id: WindowId,
    },

    /// Represents the unfiltered movement coming from the mouse. This data does not correspond to
    /// screen units (although it may match somewhat with physical screen coordinates). Comparing a
    /// sum-total of mouse motion against screen position will typically be different due to mouse
    /// acceleration, O/S settings, and other factors.
    MouseMotion {
        device_id: DeviceId,
        delta: [f32; 2],
    },
    /// This event is triggered when a mouse wheel is scrolled, even if the mouse is over a different window.
    /// However, there are some drawbacks. Some devices (ex. some touchpads) may not trigger this callback
    /// and will only trigger the MouseWheel callback.
    AnywhereMouseWheel {
        device_id: DeviceId,
        delta: Screen2d,
        delta_line: Option<[f32; 2]>,
    },

    MouseMove {
        win_id: WindowId,
        device_id: DeviceId,
        pos: Screen2d,
    },
    MouseDown {
        win_id: WindowId,
        device_id: DeviceId,
        button: MouseButton,
    },
    MouseUp {
        win_id: WindowId,
        device_id: DeviceId,
        button: MouseButton,
    },
    MouseWheel {
        win_id: WindowId,
        device_id: DeviceId,
        delta: Screen2d,
        delta_line: Option<[f32; 2]>,
        phase: TouchPhase,
    },
    MouseWindowEnter {
        win_id: WindowId,
        device_id: DeviceId,
    },
    MouseWindowLeave {
        win_id: WindowId,
        device_id: DeviceId,
    },

    TouchpadPressure {
        win_id: WindowId,
        device_id: DeviceId,
        pressure: f32,
        stage: i64,
    },
    Touch {
        win_id: WindowId,
        device_id: DeviceId,
        finger: FingerId,
        pos: Screen2d,
        phase: TouchPhase,
    },

    AxisMotion {
        win_id: WindowId,
        device_id: DeviceId,
        axis: AxisId,
        delta: f32,
    },

    KeyDown {
        win_id: WindowId,
        device_id: DeviceId,
        code: ScanCode,
        vkey: Option<VirtualKeyCode>,
    },
    KeyUp {
        win_id: WindowId,
        device_id: DeviceId,
        code: ScanCode,
        vkey: Option<VirtualKeyCode>,
    },
    KeyText {
        win_id: WindowId,
        codepoint: char,
        ch: Option<char>,
    },

    DeviceAdded {
        device_id: DeviceId,
    },
    DeviceRemoved {
        device_id: DeviceId,
    },
    DeviceMotion {
        device_id: DeviceId,
        axis: AxisId,
        delta: f32,
    },
    DeviceButtonDown {
        device_id: DeviceId,
        button: ButtonId,
    },
    DeviceButtonUp {
        device_id: DeviceId,
        button: ButtonId,
    },
    DeviceKeyDown {
        device_id: DeviceId,
        code: ScanCode,
        vkey: Option<VirtualKeyCode>,
    },
    DeviceKeyUp {
        device_id: DeviceId,
        code: ScanCode,
        vkey: Option<VirtualKeyCode>,
    },
    DeviceText {
        device_id: DeviceId,
        codepoint: char,
        ch: Option<char>,
    },

    HiDpiFactorChanged {
        win_id: WindowId,
        factor: f32,
    },
}
impl Event {
    pub fn is_mouse_event(&self) -> bool {
        match *self {
            Event::MouseMove { .. }
            | Event::MouseDown { .. }
            | Event::MouseUp { .. }
            | Event::MouseWheel { .. }
            | Event::MouseWindowEnter { .. }
            | Event::MouseWindowLeave { .. } => true,
            _ => false,
        }
    }

    pub fn from_gl(src: &gl::Event, state: &mut EventState) -> Event {
        match *src {
            gl::Event::WindowEvent {
                window_id,
                ref event,
            } => Self::from_window_event(window_id, event, state),

            gl::Event::DeviceEvent {
                device_id,
                ref event,
            } => Self::from_device_event(device_id, event, state),

            // In recent winit versions, this has been moved from WindowEvent to here
            gl::Event::Suspended(true) => Event::AppSuspend,
            gl::Event::Suspended(false) => Event::AppResume,
            gl::Event::Awakened => Event::AppAwaken,
        }
    }

    fn from_window_event(
        win_id: WindowId,
        evt: &gl::WindowEvent,
        evt_state: &mut EventState,
    ) -> Event {
        match *evt {
            gl::WindowEvent::Resized(logical_size) => {
                let size = Screen2d::from_logical_size(logical_size, evt_state.hidpi_factor);
                Event::WindowResize { win_id, size }
            }
            gl::WindowEvent::Moved(logical_pos) => {
                let pos = Screen2d::from_logical_position(logical_pos, evt_state.hidpi_factor);
                Event::WindowMove { win_id, pos }
            }
            gl::WindowEvent::CloseRequested => Event::WindowClose { win_id },
            gl::WindowEvent::Destroyed => Event::WindowDestroyed { win_id },
            gl::WindowEvent::Refresh => Event::WindowRefresh { win_id },
            gl::WindowEvent::Focused(true) => Event::WindowFocus { win_id },
            gl::WindowEvent::Focused(false) => Event::WindowBlur { win_id },

            gl::WindowEvent::DroppedFile(ref path) => Event::FileDrop {
                win_id,
                path: path.clone(),
            },
            gl::WindowEvent::HoveredFile(ref path) => Event::FileHover {
                win_id,
                path: path.clone(),
            },
            gl::WindowEvent::HoveredFileCancelled => Event::FileCancel { win_id },

            gl::WindowEvent::ReceivedCharacter(codepoint) => {
                if evt_state.ctrl_down {
                    Event::KeyText {
                        win_id,
                        codepoint,
                        ch: None,
                    }
                } else {
                    Event::KeyText {
                        win_id,
                        codepoint,
                        ch: Self::text_char(codepoint),
                    }
                }
            }
            gl::WindowEvent::KeyboardInput { device_id, input } => {
                Self::set_modifiers(evt_state, &input.modifiers);

                match (input.state, input.virtual_keycode) {
                    (gl::ElementState::Pressed, Some(VirtualKeyCode::Escape)) => {
                        if evt_state.mouse_left.pressed {
                            evt_state.mouse_left.cancelled = true;
                        }
                        if evt_state.mouse_right.pressed {
                            evt_state.mouse_right.cancelled = true;
                        }
                        if evt_state.mouse_middle.pressed {
                            evt_state.mouse_middle.cancelled = true;
                        }
                    }
                    _ => {}
                }

                match input.state {
                    gl::ElementState::Pressed => Event::KeyDown {
                        win_id,
                        device_id,
                        code: input.scancode,
                        vkey: input.virtual_keycode,
                    },
                    gl::ElementState::Released => Event::KeyUp {
                        win_id,
                        device_id,
                        code: input.scancode,
                        vkey: input.virtual_keycode,
                    },
                }
            }

            gl::WindowEvent::CursorMoved {
                device_id,
                position,
                modifiers: _,
            } => {
                let pos = Screen2d::from_logical_position(position, evt_state.hidpi_factor);
                evt_state.mouse_pos = pos;
                if !evt_state.is_any_mouse_button_pressed() {
                    evt_state.mouse_activity_start = pos;
                }
                Event::MouseMove {
                    win_id,
                    device_id,
                    pos,
                }
            }
            gl::WindowEvent::CursorEntered { device_id } => {
                evt_state.mouse_in_window = true;
                Event::MouseWindowEnter { win_id, device_id }
            }
            gl::WindowEvent::CursorLeft { device_id } => {
                evt_state.mouse_in_window = false;
                Event::MouseWindowLeave { win_id, device_id }
            }
            gl::WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
                modifiers: _,
            } => match delta {
                gl::MouseScrollDelta::LineDelta(dx, dy) => {
                    let delta = Screen2d::from_line_delta(
                        r32(dx),
                        r32(dy),
                        evt_state.logical_line_height,
                        evt_state.hidpi_factor,
                    );
                    Event::MouseWheel {
                        win_id,
                        device_id,
                        delta,
                        delta_line: Some([dx, dy]),
                        phase: TouchPhase::from_gl(phase),
                    }
                }
                gl::MouseScrollDelta::PixelDelta(logical_pos) => {
                    let delta =
                        Screen2d::from_logical_position(logical_pos, evt_state.hidpi_factor);
                    Event::MouseWheel {
                        win_id,
                        device_id,
                        delta,
                        delta_line: None,
                        phase: TouchPhase::from_gl(phase),
                    }
                }
            },
            gl::WindowEvent::MouseInput {
                device_id,
                state,
                button,
                modifiers: _,
            } => match state {
                gl::ElementState::Pressed => {
                    let pos = evt_state.mouse_pos;
                    if let Some(d) = Self::mouse_data_for(evt_state, button) {
                        d.pressed = true;
                        d.pressed_at = pos;
                        d.cancelled = false;
                    }
                    Event::MouseDown {
                        win_id,
                        device_id,
                        button: MouseButton::from_gl(button),
                    }
                }
                gl::ElementState::Released => {
                    if let Some(d) = Self::mouse_data_for(evt_state, button) {
                        d.pressed = false;
                    }
                    Event::MouseUp {
                        win_id,
                        device_id,
                        button: MouseButton::from_gl(button),
                    }
                }
            },

            gl::WindowEvent::AxisMotion {
                device_id,
                axis,
                value,
            } => Event::AxisMotion {
                win_id,
                device_id,
                axis,
                delta: value as f32,
            },

            gl::WindowEvent::TouchpadPressure {
                device_id,
                pressure,
                stage,
            } => Event::TouchpadPressure {
                win_id,
                device_id,
                pressure,
                stage,
            },
            gl::WindowEvent::Touch(ref t) => {
                let s = Screen2d::from_logical_position(t.location, evt_state.hidpi_factor);
                Event::Touch {
                    win_id,
                    device_id: t.device_id,
                    finger: t.id,
                    pos: s,
                    phase: TouchPhase::from_gl(t.phase),
                }
            }
            gl::WindowEvent::HiDpiFactorChanged(factor) => {
                let factor = factor as f32;
                evt_state.hidpi_factor = noisy_float::types::R32::new(factor);
                Event::HiDpiFactorChanged { win_id, factor }
            }
        }
    }

    fn from_device_event(
        device_id: DeviceId,
        evt: &gl::DeviceEvent,
        state: &mut EventState,
    ) -> Event {
        match *evt {
            gl::DeviceEvent::Added => Event::DeviceAdded { device_id },
            gl::DeviceEvent::Removed => Event::DeviceRemoved { device_id },
            gl::DeviceEvent::MouseMotion { delta } => Event::MouseMotion {
                device_id,
                delta: [delta.0 as f32, delta.1 as f32],
            },
            gl::DeviceEvent::MouseWheel { delta } => match delta {
                gl::MouseScrollDelta::LineDelta(dx, dy) => {
                    let delta = Screen2d::from_line_delta(
                        r32(dx),
                        r32(dy),
                        state.logical_line_height,
                        state.hidpi_factor,
                    );
                    Event::AnywhereMouseWheel {
                        device_id,
                        delta,
                        delta_line: Some([dx, dy]),
                    }
                }
                gl::MouseScrollDelta::PixelDelta(logical_pos) => {
                    let delta = Screen2d::from_logical_position(logical_pos, state.hidpi_factor);
                    Event::AnywhereMouseWheel {
                        device_id,
                        delta,
                        delta_line: None,
                    }
                }
            },
            gl::DeviceEvent::Motion { axis, value } => Event::DeviceMotion {
                device_id,
                axis,
                delta: value as f32,
            },
            gl::DeviceEvent::Button {
                button,
                state: gl::ElementState::Pressed,
            } => Event::DeviceButtonDown { device_id, button },
            gl::DeviceEvent::Button {
                button,
                state: gl::ElementState::Released,
            } => Event::DeviceButtonUp { device_id, button },
            gl::DeviceEvent::Key(key_input) => {
                Self::set_modifiers(state, &key_input.modifiers);
                match key_input.state {
                    gl::ElementState::Pressed => Event::DeviceKeyDown {
                        device_id,
                        code: key_input.scancode,
                        vkey: key_input.virtual_keycode,
                    },
                    gl::ElementState::Released => Event::DeviceKeyUp {
                        device_id,
                        code: key_input.scancode,
                        vkey: key_input.virtual_keycode,
                    },
                }
            }
            gl::DeviceEvent::Text { codepoint } => Event::DeviceText {
                device_id,
                codepoint,
                ch: Self::text_char(codepoint),
            },
        }
    }

    fn mouse_data_for<'a>(
        state: &'a mut EventState,
        b: gl::MouseButton,
    ) -> Option<&'a mut MouseButtonState> {
        match b {
            gl::MouseButton::Left => Some(&mut state.mouse_left),
            gl::MouseButton::Middle => Some(&mut state.mouse_middle),
            gl::MouseButton::Right => Some(&mut state.mouse_right),
            _ => None,
        }
    }

    fn set_modifiers(state: &mut EventState, modifiers: &gl::ModifiersState) {
        state.shift_down = modifiers.shift;
        state.ctrl_down = modifiers.ctrl;
        state.alt_down = modifiers.alt;
        state.logo_down = modifiers.logo;
    }

    fn text_char(ch: char) -> Option<char> {
        if (ch as u32) <= 31 {
            if ch == '\t' {
                Some(ch)
            } else if ch == '\r' {
                Some('\n')
            } else {
                None
            }
        } else if ch == '\u{7f}' {
            None
        } else {
            Some(ch)
        }
    }
}

/// Describes a button of a mouse controller.
#[derive(Clone, Debug, Eq, PartialEq, Hash)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Other(u8),
}
impl MouseButton {
    fn from_gl(btn: gl::MouseButton) -> Self {
        match btn {
            gl::MouseButton::Left => MouseButton::Left,
            gl::MouseButton::Right => MouseButton::Right,
            gl::MouseButton::Middle => MouseButton::Middle,
            gl::MouseButton::Other(n) => MouseButton::Other(n),
        }
    }
}
/// Describes touch-screen input state.
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
pub enum TouchPhase {
    Started,
    Moved,
    Ended,
    Cancelled,
}
impl TouchPhase {
    fn from_gl(phase: gl::TouchPhase) -> Self {
        match phase {
            gl::TouchPhase::Started => TouchPhase::Started,
            gl::TouchPhase::Moved => TouchPhase::Moved,
            gl::TouchPhase::Ended => TouchPhase::Ended,
            gl::TouchPhase::Cancelled => TouchPhase::Cancelled,
        }
    }
}
