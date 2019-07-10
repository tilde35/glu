# Event Quick-Reference #

Both the original events and the virtual key codes were derived from the [winit events file](https://github.com/rust-windowing/winit/blob/master/src/event.rs).

## Complete Match Statement ##

```rust
match e {
    glu::Event::AppAwaken => {}
    glu::Event::AppResume => {}
    glu::Event::AppSuspend => {}

    glu::Event::WindowResize { win_id, size } => {}
    glu::Event::WindowMove { win_id, pos } => {}
    glu::Event::WindowClose { win_id } => {}
    glu::Event::WindowDestroyed { win_id } => {}
    glu::Event::WindowRefresh { win_id } => {}
    glu::Event::WindowFocus { win_id } => {}
    glu::Event::WindowBlur { win_id } => {}

    glu::Event::FileDrop { win_id, path } => {}
    glu::Event::FileHover { win_id, path } => {}
    glu::Event::FileCancel { win_id } => {}

    glu::Event::MouseMotion { device_id, delta } => {}
    glu::Event::AnywhereMouseWheel { device_id, delta, delta_line } => {} // Note: Prefer MouseWheel instead

    glu::Event::MouseMove { win_id, device_id, pos } => {}
    glu::Event::MouseDown { win_id, device_id, button } => {}
    glu::Event::MouseUp { win_id, device_id, button } => {}
    glu::Event::MouseWheel { win_id, device_id, delta, delta_line, phase } => {}
    glu::Event::MouseWindowEnter { win_id, device_id } => {}
    glu::Event::MouseWindowLeave { win_id, device_id } => {}

    glu::Event::TouchpadPressure { win_id, device_id, pressure, stage } => {}
    glu::Event::Touch { win_id, device_id, finger, pos, phase } => {}

    glu::Event::AxisMotion { win_id, device_id, axis, delta } => {}

    glu::Event::KeyDown { win_id, device_id, code, vkey } => {}
    glu::Event::KeyUp { win_id, device_id, code, vkey } => {}
    glu::Event::KeyText { win_id, codepoint, ch } => {}

    glu::Event::DeviceAdded { device_id } => {}
    glu::Event::DeviceRemoved { device_id } => {}
    glu::Event::DeviceMotion { device_id, axis, delta } => {}
    glu::Event::DeviceButtonDown { device_id, button } => {}
    glu::Event::DeviceButtonUp { device_id, button } => {}
    glu::Event::DeviceKeyDown { device_id, code, vkey } => {}
    glu::Event::DeviceKeyUp { device_id, code, vkey } => {}
    glu::Event::DeviceText { device_id, codepoint, ch } => {}

    glu::Event::HiDpiFactorChanged { win_id, factor } => {}
}
```

## Virtual Key Code Reference ##

```rust
pub enum VirtualKeyCode {
    // Numbers above letters (not the numpad)
    Key1, Key2, Key3, Key4, Key5, Key6, Key7, Key8, Key9, Key0,

    A, B, C, D, E, F, G, H, I, J, K, L, M, N, O, P, Q, R, S, T, U, V, W, X, Y, Z,

    Escape,

    F1, F2, F3, F4, F5, F6, F7, F8, F9, F10, F11, F12, F13, F14, F15,

    Snapshot, // Print Screen
    Scroll, // Scroll Lock
    Pause, // Pause/Break

    Insert,
    Home,
    Delete,
    End,
    PageDown,
    PageUp,

    Left, Up, Right, Down,

    Back, // Backspace
    Return, // Enter
    Space,

    LAlt,
    LBracket,
    LControl,
    LMenu,
    LShift,
    LWin,
    RAlt,
    RBracket,
    RControl,
    RMenu,
    RShift,
    RWin,

    Numlock,
    Numpad0, Numpad1, Numpad2, Numpad3, Numpad4, Numpad5, Numpad6, Numpad7, Numpad8, Numpad9,
    NumpadComma, NumpadEnter, NumpadEquals,

    AbntC1,
    AbntC2,
    Add,
    Apostrophe,
    Apps,
    At,
    Ax,
    Backslash,
    Capital,
    Colon,
    Comma,
    Convert,
    Decimal,
    Divide,
    Equals,
    Grave,
    Kana,
    Kanji,
    Mail,
    Minus,
    Multiply,
    NoConvert,
    OEM102,
    Period,
    Semicolon,
    Slash,
    Subtract,
    Sysrq,
    Tab,
    Underline,
    Unlabeled,
    Yen,

    Calculator,
    MediaSelect,
    MediaStop,
    MyComputer,
    Mute,
    NavigateForward, // also called "Prior"
    NavigateBackward, // also called "Next"
    NextTrack,
    PlayPause,
    Power,
    PrevTrack,
    Sleep,
    Stop,
    VolumeDown,
    VolumeUp,
    Wake,
    WebBack,
    WebFavorites,
    WebForward,
    WebHome,
    WebRefresh,
    WebSearch,
    WebStop,

    /// The "Compose" key on Linux.
    Compose,
}
```
