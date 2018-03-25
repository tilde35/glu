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

mod ascii_text_img;
mod ascii_text;
mod event;
mod event_state;
mod time_step;

pub use ascii_text::AsciiText;
pub use event::{AxisId, ButtonId, Event, FingerId, MouseButton, ScanCode, TouchPhase};
pub use event_state::{EventState, MouseButtonState};
pub use glium::glutin::{DeviceId, VirtualKeyCode, WindowId};
pub use time_step::TimeStep;
