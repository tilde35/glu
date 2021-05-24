extern crate fps_counter;
extern crate glium;
extern crate glu;

use fps_counter::FPSCounter;
use glium::Surface;
use glu::{AsciiText, Event, TimeStep, VirtualKeyCode};

fn main() {
    let win = glu::Window::create("FPS and Time Step")
        .with_inner_logical([800., 400.])
        .with_vsync(true)
        .create();

    let ascii_text = AsciiText::new(&win.display);

    let mut fps = FPSCounter::new();
    let mut sim_step = TimeStep::for_freq_ms(500).max_missed_steps_before_discard(1);
    let mut sim_counter = 0;

    win.run(move |display, e, event_state| {
        match e {
            Event::WindowClose { .. } => glu::WindowState::Exit,
            Event::KeyDown {
                vkey: Some(VirtualKeyCode::Escape),
                ..
            } => glu::WindowState::Exit,
            // Hide noisy events
            Event::MouseMove { .. }
            | Event::MouseMotion { .. }
            | Event::DeviceMotion { .. }
            | Event::Placeholder => glu::WindowState::Run,
            Event::Redraw => {
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

                glu::WindowState::WaitFor(std::time::Duration::from_millis(10))
            }
            // Display all other events
            _ => {
                println!("{:?}", &e);
                glu::WindowState::Run
            }
        }
    });
}
