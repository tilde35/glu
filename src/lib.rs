//! glu - Small Simple OpenGL Utilities
//!
//! This is a collection of simple OpenGL utilities to make writing OpenGL applications a little easier.
//!
//! Main components:
//!
//! - AsciiText
//! - Event
//! - EventState
//! - TimeStep
//!

#[macro_use]
extern crate glium;

mod ascii_text;
mod ascii_text_img;
mod event;
mod event_state;
mod screen_units;
mod time_step;
mod window;

pub use crate::ascii_text::AsciiText;
pub use crate::event::{AxisId, ButtonId, Event, FingerId, MouseButton, ScanCode, TouchPhase};
pub use crate::event_state::{EventState, MouseButtonState};
pub use crate::screen_units::Screen2d;
pub use crate::time_step::TimeStep;
pub use crate::window::*;
pub use glium::glutin::event::{DeviceId, VirtualKeyCode};
pub use glium::glutin::window::WindowId;
