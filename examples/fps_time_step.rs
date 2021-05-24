extern crate fps_counter;
extern crate glium;
extern crate glu;

use fps_counter::FPSCounter;
use glium::{glutin, Surface};
use glu::{AsciiText, Event, EventState, TimeStep, VirtualKeyCode};

fn main() {
    let win_size = (1024, 720);

    let events_loop = glutin::event_loop::EventLoop::new();
    let window = glutin::window::WindowBuilder::new()
        .with_inner_size(glutin::dpi::Size::Logical(win_size.into()))
        .with_title("FPS and Time Step");
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let mut event_state = EventState::new(&display);

    let ascii_text = AsciiText::new(&display);

    let mut fps = FPSCounter::new();
    let mut sim_step = TimeStep::for_freq_ms(500).max_missed_steps_before_discard(1);
    let mut sim_counter = 0;

    events_loop.run(move |event, _win_target, control_flow| {
        *control_flow = glutin::event_loop::ControlFlow::Poll;

        let redraw = match &event {
            glutin::event::Event::RedrawRequested(..)
            | glutin::event::Event::RedrawEventsCleared => true,
            _ => false,
        };

        if redraw {
            let cur_fps = fps.tick();
            sim_step.tick(|| sim_counter += 1);
            let s = format!("FPS: {:?}, Simulation Counter: {:?}", cur_fps, sim_counter);

            let mut target = display.draw();
            target.clear_color_and_depth((0.01, 0.0, 0.1, 1.0), 1.0);

            let mut msg = s.as_bytes().to_owned();
            msg.insert(0, b' ');
            msg.insert(0, 15);
            msg.push(b' ');
            msg.push(15);

            let size = event_state.primary_win_dim().logical();

            ascii_text.draw_white(
                &display,
                &mut target,
                &msg,
                2.0,
                [10.0, size[1] - 16.0 - 2.0],
            );

            target.finish().unwrap();
        } else {
            let e = Event::from_gl(&event, &mut event_state);

            match e {
                Event::WindowClose { .. } => *control_flow = glutin::event_loop::ControlFlow::Exit,
                Event::KeyDown {
                    vkey: Some(VirtualKeyCode::Escape),
                    ..
                } => *control_flow = glutin::event_loop::ControlFlow::Exit,
                // Hide noisy events
                Event::MouseMove { .. }
                | Event::MouseMotion { .. }
                | Event::DeviceMotion { .. } => {}
                Event::Placeholder => {}
                // Display all other events
                _ => {
                    println!("{:?}", &e);
                }
            }
        }
    });
}
