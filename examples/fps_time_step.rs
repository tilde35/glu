extern crate fps_counter;
extern crate glium;
extern crate glu;

use fps_counter::FPSCounter;
use glium::{glutin, Surface};
use glu::{AsciiText, Event, EventState, TimeStep};

fn main() {
    let win_size = (1024, 720);

    let mut event_state = EventState::new();
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new()
        .with_dimensions(win_size.0, win_size.1)
        .with_title("FPS and Time Step");
    let context = glutin::ContextBuilder::new().with_vsync(true);
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let ascii_text = AsciiText::new(&display);

    let mut fps = FPSCounter::new();
    let mut sim_step = TimeStep::for_freq_ms(500).max_missed_steps_before_discard(1);
    let mut sim_counter = 0;

    let mut exit = false;
    while !exit {
        let cur_fps = fps.tick();
        sim_step.tick(|| sim_counter += 1);
        let s = format!("FPS: {:?}, Simulation Counter: {:?}", cur_fps, sim_counter);

        events_loop.poll_events(|event| {
            let e = Event::from_gl(&event, &mut event_state);
            match e {
                Event::WindowClose(..) => exit = true,
                _ => {}
            }
        });

        let mut target = display.draw();
        target.clear_color_and_depth((0.01, 0.0, 0.1, 1.0), 1.0);

        ascii_text.draw_white(&display, &mut target, &s, 1.5, [10.0, 10.0]);

        target.finish().unwrap();
    }
}
