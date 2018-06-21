// Event Definitions:
// Version 0.7.6
//   https://github.com/tomaka/winit/blob/be9d4e7e03c9b37d50868c129199564b4efb7517/src/events.rs
// Current
//   https://github.com/tomaka/winit/blob/master/src/events.rs

use event_state::{EventState, MouseButtonState};
use glium::glutin as gl;
// Note: these are re-exported by lib.rs as well
pub use glium::glutin::{DeviceId, VirtualKeyCode, WindowId};
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

    WindowResize(WindowId, u32, u32),
    WindowMove(WindowId, i32, i32),
    WindowClose(WindowId),
    WindowDestroyed(WindowId),
    WindowRefresh(WindowId),
    WindowFocus(WindowId),
    WindowBlur(WindowId),

    FileDrop(WindowId, PathBuf),
    FileHover(WindowId, PathBuf),
    FileCancel(WindowId),

    MouseMotion(DeviceId, f64, f64),
    AnyMouseWheelByLine(DeviceId, f32, f32),
    AnyMouseWheelByPixel(DeviceId, f32, f32),

    MouseMove(WindowId, DeviceId, i32, i32),
    MouseDown(WindowId, DeviceId, MouseButton),
    MouseUp(WindowId, DeviceId, MouseButton),
    MouseWheelByLine(WindowId, DeviceId, f32, f32, TouchPhase),
    MouseWheelByPixel(WindowId, DeviceId, f32, f32, TouchPhase),
    MouseWindowEnter(WindowId, DeviceId),
    MouseWindowLeave(WindowId, DeviceId),

    // WindowId, DeviceId, Pressure(f32), Stage(i64)
    TouchpadPressure(WindowId, DeviceId, f32, i64),
    Touch(WindowId, DeviceId, FingerId, f64, f64, TouchPhase),

    AxisMotion(WindowId, DeviceId, AxisId, f64),

    KeyDown(WindowId, DeviceId, ScanCode, Option<VirtualKeyCode>),
    KeyUp(WindowId, DeviceId, ScanCode, Option<VirtualKeyCode>),
    KeyText(WindowId, char, Option<char>),

    DeviceAdded(DeviceId),
    DeviceRemoved(DeviceId),
    DeviceMotion(DeviceId, AxisId, f64),
    DeviceButtonDown(DeviceId, ButtonId),
    DeviceButtonUp(DeviceId, ButtonId),
    DeviceKeyDown(DeviceId, ScanCode, Option<VirtualKeyCode>),
    DeviceKeyUp(DeviceId, ScanCode, Option<VirtualKeyCode>),
    DeviceText(DeviceId, char, Option<char>),

    HiDPIFactorChanged(f32),
}
impl Event {
    pub fn is_mouse_event(&self) -> bool {
        match *self {
            Event::MouseMove(..)
            | Event::MouseDown(..)
            | Event::MouseUp(..)
            | Event::MouseWheelByLine(..)
            | Event::MouseWheelByPixel(..)
            | Event::MouseWindowEnter(..)
            | Event::MouseWindowLeave(..) => true,
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

    fn from_window_event(id: WindowId, evt: &gl::WindowEvent, evt_state: &mut EventState) -> Event {
        match *evt {
            gl::WindowEvent::Resized(w, h) => Event::WindowResize(id, w, h),
            gl::WindowEvent::Moved(x, y) => Event::WindowMove(id, x, y),
            gl::WindowEvent::CloseRequested => Event::WindowClose(id),
            gl::WindowEvent::Destroyed => Event::WindowDestroyed(id),
            gl::WindowEvent::Refresh => Event::WindowRefresh(id),
            gl::WindowEvent::Focused(true) => Event::WindowFocus(id),
            gl::WindowEvent::Focused(false) => Event::WindowBlur(id),

            gl::WindowEvent::DroppedFile(ref path) => Event::FileDrop(id, path.clone()),
            gl::WindowEvent::HoveredFile(ref path) => Event::FileHover(id, path.clone()),
            gl::WindowEvent::HoveredFileCancelled => Event::FileCancel(id),

            gl::WindowEvent::ReceivedCharacter(ch) => if evt_state.ctrl_down {
                Event::KeyText(id, ch, None)
            } else {
                Event::KeyText(id, ch, Self::text_char(ch))
            },
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
                    gl::ElementState::Pressed => {
                        Event::KeyDown(id, device_id, input.scancode, input.virtual_keycode)
                    }
                    gl::ElementState::Released => {
                        Event::KeyUp(id, device_id, input.scancode, input.virtual_keycode)
                    }
                }
            }

            gl::WindowEvent::CursorMoved {
                device_id,
                position,
                modifiers: _,
            } => {
                let x = position.0 as i32;
                let y = position.1 as i32;
                evt_state.mouse_pos = [x, y];
                if !evt_state.is_any_mouse_button_pressed() {
                    evt_state.mouse_activity_start = [x, y];
                }
                Event::MouseMove(id, device_id, x, y)
            }
            gl::WindowEvent::CursorEntered { device_id } => {
                evt_state.mouse_in_window = true;
                Event::MouseWindowEnter(id, device_id)
            }
            gl::WindowEvent::CursorLeft { device_id } => {
                evt_state.mouse_in_window = false;
                Event::MouseWindowLeave(id, device_id)
            }
            gl::WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
                modifiers: _,
            } => match delta {
                gl::MouseScrollDelta::LineDelta(dx, dy) => {
                    Event::MouseWheelByLine(id, device_id, dx, dy, TouchPhase::from_gl(phase))
                }
                gl::MouseScrollDelta::PixelDelta(dx, dy) => {
                    Event::MouseWheelByPixel(id, device_id, dx, dy, TouchPhase::from_gl(phase))
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
                    Event::MouseDown(id, device_id, MouseButton::from_gl(button))
                }
                gl::ElementState::Released => {
                    if let Some(d) = Self::mouse_data_for(evt_state, button) {
                        d.pressed = false;
                    }
                    Event::MouseUp(id, device_id, MouseButton::from_gl(button))
                }
            },

            gl::WindowEvent::AxisMotion {
                device_id,
                axis,
                value,
            } => Event::AxisMotion(id, device_id, axis, value),

            gl::WindowEvent::TouchpadPressure {
                device_id,
                pressure,
                stage,
            } => Event::TouchpadPressure(id, device_id, pressure, stage),
            gl::WindowEvent::Touch(ref t) => Event::Touch(
                id,
                t.device_id,
                t.id,
                t.location.0,
                t.location.1,
                TouchPhase::from_gl(t.phase),
            ),
            gl::WindowEvent::HiDPIFactorChanged(factor) => Event::HiDPIFactorChanged(factor),
        }
    }

    fn from_device_event(id: DeviceId, evt: &gl::DeviceEvent, state: &mut EventState) -> Event {
        match *evt {
            gl::DeviceEvent::Added => Event::DeviceAdded(id),
            gl::DeviceEvent::Removed => Event::DeviceRemoved(id),
            gl::DeviceEvent::MouseMotion { delta } => Event::MouseMotion(id, delta.0, delta.1),
            gl::DeviceEvent::MouseWheel { delta } => match delta {
                gl::MouseScrollDelta::LineDelta(dx, dy) => Event::AnyMouseWheelByLine(id, dx, dy),
                gl::MouseScrollDelta::PixelDelta(dx, dy) => Event::AnyMouseWheelByPixel(id, dx, dy),
            },
            gl::DeviceEvent::Motion { axis, value } => Event::DeviceMotion(id, axis, value),
            gl::DeviceEvent::Button {
                button,
                state: gl::ElementState::Pressed,
            } => Event::DeviceButtonDown(id, button),
            gl::DeviceEvent::Button {
                button,
                state: gl::ElementState::Released,
            } => Event::DeviceButtonUp(id, button),
            gl::DeviceEvent::Key(key_input) => {
                Self::set_modifiers(state, &key_input.modifiers);
                match key_input.state {
                    gl::ElementState::Pressed => {
                        Event::DeviceKeyDown(id, key_input.scancode, key_input.virtual_keycode)
                    }
                    gl::ElementState::Released => {
                        Event::DeviceKeyUp(id, key_input.scancode, key_input.virtual_keycode)
                    }
                }
            }
            gl::DeviceEvent::Text { codepoint } => {
                Event::DeviceText(id, codepoint, Self::text_char(codepoint))
            }
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
