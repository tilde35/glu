# glu - Small Simple GL Utilities #

The glu project is a set of small, simple OpenGL utilities for use in my personal projects.

Cargo Dependency: `glu = { git = "https://github.com/tilde35/glu" }`

### Utility List ###
* ASCII Font
* Event Mapping
* Event State
* Time Step

## ASCII Font ##

```rust
use glu::AsciiFont;

// Intialization
let ascii_font = AsciiFont::new(&display);

// Render text (text, scale, location, color)
ascii_font.draw(&display, &mut target, "Example Text", 2.0, (15, 15), (255, 128, 0));

// Render white/black text (text, scale, location)
ascii_font.draw_white(&display, &mut target, "Example Text", 2.0, (15, 15));
ascii_font.draw_black(&display, &mut target, "Example Text", 2.0, (15, 15));
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
