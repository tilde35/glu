use crate::{Event, EventState};
use glium::glutin;

pub struct Window {
    event_loop: glutin::event_loop::EventLoop<()>,
    pub display: glium::Display,
    pub event_state: EventState,
}
impl Window {
    pub fn create(title: &str) -> WindowBuilder {
        WindowBuilder {
            title: title.into(),
            inner_dim: [600.0, 400.0],
            logical: true,
            vsync: false,
        }
    }

    pub fn run(
        self,
        mut action: impl FnMut(&glium::Display, Event, &EventState) -> WindowState + 'static,
    ) -> ! {
        let display = self.display;
        let mut event_state = self.event_state;
        self.event_loop
            .run(move |event, _win_target, control_flow| {
                let e = Event::from_gl(&event, &mut event_state);

                match action(&display, e, &event_state) {
                    WindowState::Run => *control_flow = glutin::event_loop::ControlFlow::Poll,
                    WindowState::Wait => *control_flow = glutin::event_loop::ControlFlow::Wait,
                    WindowState::WaitUntil(t) => {
                        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(t)
                    }
                    WindowState::WaitFor(d) => {
                        let t = std::time::Instant::now() + d;
                        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(t)
                    }
                    WindowState::Exit => *control_flow = glutin::event_loop::ControlFlow::Exit,
                }
            });
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum WindowState {
    Run,
    /// When the current loop iteration finishes, suspend the thread until another event arrives.
    Wait,
    /// When the current loop iteration finishes, suspend the thread until either another event
    /// arrives or the given time is reached.
    WaitUntil(std::time::Instant),
    WaitFor(std::time::Duration),
    Exit,
}

pub struct WindowBuilder {
    title: String,
    inner_dim: [f32; 2],
    logical: bool,
    vsync: bool,
}
impl WindowBuilder {
    pub fn with_inner_logical(mut self, dim: [f32; 2]) -> Self {
        self.inner_dim = dim;
        self.logical = true;
        self
    }
    pub fn with_inner_physical(mut self, dim: [u32; 2]) -> Self {
        self.inner_dim = [dim[0] as f32, dim[1] as f32];
        self.logical = false;
        self
    }
    pub fn with_vsync(mut self, vsync: bool) -> Self {
        self.vsync = vsync;
        self
    }
    pub fn create(self) -> Window {
        let size = if self.logical {
            glutin::dpi::Size::Logical(glutin::dpi::LogicalSize {
                width: self.inner_dim[0] as f64,
                height: self.inner_dim[1] as f64,
            })
        } else {
            glutin::dpi::Size::Physical(glutin::dpi::PhysicalSize {
                width: self.inner_dim[0] as u32,
                height: self.inner_dim[1] as u32,
            })
        };
        let event_loop = glutin::event_loop::EventLoop::new();
        let window = glutin::window::WindowBuilder::new()
            .with_inner_size(size)
            .with_title(&self.title);
        let context = glutin::ContextBuilder::new().with_vsync(self.vsync);
        let display = glium::Display::new(window, context, &event_loop).unwrap();
        let event_state = EventState::new(&display);
        Window {
            event_loop,
            display,
            event_state,
        }
    }
}
