// Event Definitions:
//   https://github.com/rust-windowing/winit/blob/master/src/event.rs

use crate::event_state::{EventState, MouseButtonState};
use crate::screen_units::Screen2d;
use glium::glutin::event as gle;
use glium::glutin::event::{DeviceId, VirtualKeyCode};
use glium::glutin::window::WindowId;
use noisy_float::prelude::*;
use std::path::PathBuf;

pub type AxisId = u32;
pub type ScanCode = u32;
pub type ButtonId = u32;
pub type FingerId = u64;

/// An event from OpenGL. This is a simplified version of the events provided by winit.
#[derive(Clone, Debug, PartialEq)]
pub enum Event {
    Placeholder,

    AppAwaken,
    AppResume,
    AppSuspend,
    Redraw,

    /// Shift/ctrl/alt changed
    ModifiersChanged,

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

    pub fn from_gl<T>(src: &gle::Event<T>, state: &mut EventState) -> Event {
        match *src {
            gle::Event::WindowEvent {
                window_id,
                ref event,
            } => Self::from_window_event(window_id, event, state),

            gle::Event::DeviceEvent {
                device_id,
                ref event,
            } => Self::from_device_event(device_id, event, state),

            gle::Event::Suspended => Event::AppSuspend,
            gle::Event::Resumed => Event::AppResume,

            gle::Event::RedrawRequested(_) => Event::Redraw,
            gle::Event::RedrawEventsCleared => Event::Redraw,

            // New events (ignored for now)
            gle::Event::NewEvents(_cause) => Event::Placeholder,
            gle::Event::UserEvent(..) => Event::Placeholder,
            gle::Event::MainEventsCleared => Event::Placeholder,
            gle::Event::LoopDestroyed => Event::Placeholder,
        }
    }

    fn from_window_event<'a>(
        win_id: WindowId,
        evt: &gle::WindowEvent,
        evt_state: &mut EventState,
    ) -> Event {
        match evt {
            gle::WindowEvent::Resized(phys_size) => {
                let size = {
                    let w = evt_state.get_or_create_win(win_id);
                    let f = w.hidpi_factor;
                    let size = Screen2d::from_physical_size_u32(phys_size, f.const_raw());
                    w.dim = size;
                    size
                };
                Event::WindowResize { win_id, size }
            }
            gle::WindowEvent::Moved(logical_pos) => {
                let f = evt_state.get_or_create_win(win_id).hidpi_factor;
                let pos = Screen2d::from_physical_position_i32(logical_pos, f);
                Event::WindowMove { win_id, pos }
            }
            gle::WindowEvent::CloseRequested => Event::WindowClose { win_id },
            gle::WindowEvent::Destroyed => {
                evt_state.window_destroyed(win_id);
                Event::WindowDestroyed { win_id }
            }
            //gle::WindowEvent::Refresh => Event::WindowRefresh { win_id },
            gle::WindowEvent::Focused(true) => Event::WindowFocus { win_id },
            gle::WindowEvent::Focused(false) => Event::WindowBlur { win_id },

            gle::WindowEvent::DroppedFile(ref path) => Event::FileDrop {
                win_id,
                path: path.clone(),
            },
            gle::WindowEvent::HoveredFile(ref path) => Event::FileHover {
                win_id,
                path: path.clone(),
            },
            gle::WindowEvent::HoveredFileCancelled => Event::FileCancel { win_id },

            gle::WindowEvent::ReceivedCharacter(codepoint) => {
                if evt_state.ctrl_down {
                    Event::KeyText {
                        win_id,
                        codepoint: *codepoint,
                        ch: None,
                    }
                } else {
                    Event::KeyText {
                        win_id,
                        codepoint: *codepoint,
                        ch: Self::text_char(*codepoint),
                    }
                }
            }
            gle::WindowEvent::KeyboardInput {
                device_id,
                input,
                is_synthetic,
            } => {
                let _ = is_synthetic;
                //Self::set_modifiers(evt_state, &input.modifiers);

                match (input.state, input.virtual_keycode) {
                    (gle::ElementState::Pressed, Some(VirtualKeyCode::Escape)) => {
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
                    gle::ElementState::Pressed => Event::KeyDown {
                        win_id,
                        device_id: *device_id,
                        code: input.scancode,
                        vkey: input.virtual_keycode,
                    },
                    gle::ElementState::Released => Event::KeyUp {
                        win_id,
                        device_id: *device_id,
                        code: input.scancode,
                        vkey: input.virtual_keycode,
                    },
                }
            }

            gle::WindowEvent::CursorMoved {
                device_id,
                position,
                ..
            } => {
                let f = evt_state.get_or_create_win(win_id).hidpi_factor;
                let pos = Screen2d::from_physical_position_f64(position, f);
                evt_state.mouse_pos = pos;
                if !evt_state.is_any_mouse_button_pressed() {
                    evt_state.mouse_activity_start = pos;
                }
                Event::MouseMove {
                    win_id,
                    device_id: *device_id,
                    pos,
                }
            }
            gle::WindowEvent::CursorEntered { device_id } => {
                evt_state.mouse_in_window = true;
                Event::MouseWindowEnter {
                    win_id,
                    device_id: *device_id,
                }
            }
            gle::WindowEvent::CursorLeft { device_id } => {
                evt_state.mouse_in_window = false;
                Event::MouseWindowLeave {
                    win_id,
                    device_id: *device_id,
                }
            }
            gle::WindowEvent::MouseWheel {
                device_id,
                delta,
                phase,
                ..
            } => match delta {
                gle::MouseScrollDelta::LineDelta(dx, dy) => {
                    let f = evt_state.get_or_create_win(win_id).hidpi_factor;
                    let delta = Screen2d::from_line_delta(
                        r32(*dx),
                        r32(*dy),
                        evt_state.logical_line_height,
                        f,
                    );
                    Event::MouseWheel {
                        win_id,
                        device_id: *device_id,
                        delta,
                        delta_line: Some([*dx, *dy]),
                        phase: TouchPhase::from_gl(*phase),
                    }
                }
                gle::MouseScrollDelta::PixelDelta(phys_pos) => {
                    let f = evt_state.get_or_create_win(win_id).hidpi_factor;
                    let delta = Screen2d::from_physical_position_f64(phys_pos, f);
                    Event::MouseWheel {
                        win_id,
                        device_id: *device_id,
                        delta,
                        delta_line: None,
                        phase: TouchPhase::from_gl(*phase),
                    }
                }
            },
            gle::WindowEvent::MouseInput {
                device_id,
                state,
                button,
                ..
            } => match state {
                gle::ElementState::Pressed => {
                    let pos = evt_state.mouse_pos;
                    if let Some(d) = Self::mouse_data_for(evt_state, *button) {
                        d.pressed = true;
                        d.pressed_at = pos;
                        d.cancelled = false;
                    }
                    Event::MouseDown {
                        win_id,
                        device_id: *device_id,
                        button: MouseButton::from_gl(*button),
                    }
                }
                gle::ElementState::Released => {
                    if let Some(d) = Self::mouse_data_for(evt_state, *button) {
                        d.pressed = false;
                    }
                    Event::MouseUp {
                        win_id,
                        device_id: *device_id,
                        button: MouseButton::from_gl(*button),
                    }
                }
            },

            gle::WindowEvent::AxisMotion {
                device_id,
                axis,
                value,
            } => Event::AxisMotion {
                win_id,
                device_id: *device_id,
                axis: *axis,
                delta: *value as f32,
            },

            gle::WindowEvent::TouchpadPressure {
                device_id,
                pressure,
                stage,
            } => Event::TouchpadPressure {
                win_id,
                device_id: *device_id,
                pressure: *pressure,
                stage: *stage,
            },
            gle::WindowEvent::Touch(ref t) => {
                let f = evt_state.get_or_create_win(win_id).hidpi_factor;
                let s = Screen2d::from_physical_position_f64(&t.location, f);
                Event::Touch {
                    win_id,
                    device_id: t.device_id,
                    finger: t.id,
                    pos: s,
                    phase: TouchPhase::from_gl(t.phase),
                }
            }
            gle::WindowEvent::ScaleFactorChanged {
                scale_factor,
                new_inner_size,
            } => {
                let _ = new_inner_size;
                let factor = *scale_factor as f32;
                evt_state.get_or_create_win(win_id).hidpi_factor = r32(factor);
                Event::HiDpiFactorChanged { win_id, factor }
            }
            gle::WindowEvent::ModifiersChanged(m) => {
                Self::set_modifiers(evt_state, m);
                // TODO! Assign modifiers in the future
                Event::ModifiersChanged
            }
            gle::WindowEvent::ThemeChanged(_t) => Event::Placeholder,
        }
    }

    fn from_device_event(
        device_id: DeviceId,
        evt: &gle::DeviceEvent,
        state: &mut EventState,
    ) -> Event {
        match *evt {
            gle::DeviceEvent::Added => Event::DeviceAdded { device_id },
            gle::DeviceEvent::Removed => Event::DeviceRemoved { device_id },
            gle::DeviceEvent::MouseMotion { delta } => Event::MouseMotion {
                device_id,
                delta: [delta.0 as f32, delta.1 as f32],
            },
            gle::DeviceEvent::MouseWheel { delta } => match delta {
                gle::MouseScrollDelta::LineDelta(dx, dy) => {
                    let f = state.hidpi_factor_r32();
                    let delta =
                        Screen2d::from_line_delta(r32(dx), r32(dy), state.logical_line_height, f);
                    Event::AnywhereMouseWheel {
                        device_id,
                        delta,
                        delta_line: Some([dx, dy]),
                    }
                }
                gle::MouseScrollDelta::PixelDelta(phys_pos) => {
                    let f = state.hidpi_factor_r32();
                    let delta = Screen2d::from_physical_position_f64(&phys_pos, f);
                    Event::AnywhereMouseWheel {
                        device_id,
                        delta,
                        delta_line: None,
                    }
                }
            },
            gle::DeviceEvent::Motion { axis, value } => Event::DeviceMotion {
                device_id,
                axis,
                delta: value as f32,
            },
            gle::DeviceEvent::Button {
                button,
                state: gle::ElementState::Pressed,
            } => Event::DeviceButtonDown { device_id, button },
            gle::DeviceEvent::Button {
                button,
                state: gle::ElementState::Released,
            } => Event::DeviceButtonUp { device_id, button },
            gle::DeviceEvent::Key(key_input) => {
                //Self::set_modifiers(state, &key_input.modifiers);
                match key_input.state {
                    gle::ElementState::Pressed => Event::DeviceKeyDown {
                        device_id,
                        code: key_input.scancode,
                        vkey: key_input.virtual_keycode,
                    },
                    gle::ElementState::Released => Event::DeviceKeyUp {
                        device_id,
                        code: key_input.scancode,
                        vkey: key_input.virtual_keycode,
                    },
                }
            }
            gle::DeviceEvent::Text { codepoint } => Event::DeviceText {
                device_id,
                codepoint,
                ch: Self::text_char(codepoint),
            },
        }
    }

    fn mouse_data_for<'a>(
        state: &'a mut EventState,
        b: gle::MouseButton,
    ) -> Option<&'a mut MouseButtonState> {
        match b {
            gle::MouseButton::Left => Some(&mut state.mouse_left),
            gle::MouseButton::Middle => Some(&mut state.mouse_middle),
            gle::MouseButton::Right => Some(&mut state.mouse_right),
            _ => None,
        }
    }

    fn set_modifiers(state: &mut EventState, modifiers: &gle::ModifiersState) {
        state.shift_down = modifiers.shift();
        state.ctrl_down = modifiers.ctrl();
        state.alt_down = modifiers.alt();
        state.logo_down = modifiers.logo();
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
    fn from_gl(btn: gle::MouseButton) -> Self {
        match btn {
            gle::MouseButton::Left => MouseButton::Left,
            gle::MouseButton::Right => MouseButton::Right,
            gle::MouseButton::Middle => MouseButton::Middle,
            gle::MouseButton::Other(n) => MouseButton::Other(n as u8),
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
    fn from_gl(phase: gle::TouchPhase) -> Self {
        match phase {
            gle::TouchPhase::Started => TouchPhase::Started,
            gle::TouchPhase::Moved => TouchPhase::Moved,
            gle::TouchPhase::Ended => TouchPhase::Ended,
            gle::TouchPhase::Cancelled => TouchPhase::Cancelled,
        }
    }
}
