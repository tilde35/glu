# glu - Small Simple GL Utilities #

The glu project is a set of small, simple OpenGL utilities for use in my personal projects.

Cargo Dependency: `glu = { git = "https://github.com/tilde35/glu" }`

For a more complete setup, refer to the  [example project](examples/fps_time_step.rs).

### Utility List ###
* ASCII Text
* Events and Event State
* Time Step

## ASCII Text ##

```rust
use glu::AsciiText;

// Intialization
let ascii_text = AsciiText::new(&display);

// Render text (text, scale, location, color)
ascii_text.draw(&display, &mut target, "Example Text", 2.0, [15.0, 15.0], [0.8, 0.9, 0.8, 1.0]);

// Render white/black text (text, scale, location)
ascii_text.draw_white(&display, &mut target, "Example Text", 2.0, [15.0, 15.0]);
ascii_text.draw_black(&display, &mut target, "Example Text", 2.0, [15.0, 15.0]);
```

## Events and Event State ##

For more information, refer to the [event quick-reference guide](Events.md).

```rust
use glu::{Event, EventState};

// Intialization
let mut event_state = EventState::new();

// Event loop
events_loop.poll_events(|event| {
    let e = Event::from_gl(&event, &mut event_state);
    match e {
        Event::WindowClose(..) => exit = true,
        _ => {}
    }
});
```

## Time Step ##

```rust
use glu::TimeStep;

// Intialization
let mut step = TimeStep::for_freq_ms(250);

// Per frame
step.tick(|| simulator.next_step());
```

# External Library Quick-Reference #

## fps_counter  ##

Cargo Dependency: `fps_counter = "1.0.0"`

### Usage ###

```rust
use fps_counter::FPSCounter;

let mut fps = FPSCounter::new();

loop {
  let cur_fps = fps.tick();
  // Process events and render page...
}
```
