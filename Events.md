# Event Quick-Reference #

Both the original events and the virtual key codes are found in the [winit events file](https://github.com/tomaka/winit/blob/master/src/events.rs).

## Complete Match Statement ##

```rust
match e {
  glu::Event::AppAwaken => {},
  glu::Event::AppResume => {},
  glu::Event::AppSuspend => {},

  glu::Event::WindowResize(window_id, width, height) => {},
  glu::Event::WindowMove(window_id, x, y) => {},
  glu::Event::WindowClose(window_id) => {},
  glu::Event::WindowRefresh(window_id) => {},
  glu::Event::WindowFocus(window_id) => {},
  glu::Event::WindowBlur(window_id) => {},

  glu::Event::FileDrop(window_id, path) => {},
  glu::Event::FileHover(window_id, path) => {},
  glu::Event::FileCancel(window_id) => {},

  glu::Event::MouseMotion(device_id, dx, dy) => {},
  glu::Event::AnyMouseWheelByLine(device_id, dx, dy) => {},
  glu::Event::AnyMouseWheelByPixel(device_id, dx, dy) => {},

  glu::Event::MouseMove(window_id, device_id, x, y) => {},
  glu::Event::MouseDown(window_id, device_id, mouse_button) => {},
  glu::Event::MouseUp(window_id, device_id, mouse_button) => {},
  glu::Event::MouseWheelByLine(window_id, device_id, dx, dy, touch_phase) => {},
  glu::Event::MouseWheelByPixel(window_id, device_id, dx, dy, touch_phase) => {},
  glu::Event::MouseWindowEnter(window_id, device_id) => {},
  glu::Event::MouseWindowLeave(window_id, device_id) => {},

  glu::Event::TouchpadPressure(window_id, device_id, pressure, stage) => {},
  glu::Event::Touch(window_id, device_id, finger_id, x, y, touch_phase) => {},

  glu::Event::AxisMotion(window_id, device_id, axis_id, delta) => {},

  glu::Event::KeyDown(window_id, device_id, scan_code, virtual_key_code) => {},
  glu::Event::KeyUp(window_id, device_id, scan_code, virtual_key_code) => {},
  glu::Event::KeyText(window_id, event_char, valid_char) => {},

  glu::Event::DeviceAdded(device_id) => {},
  glu::Event::DeviceRemoved(device_id) => {},
  glu::Event::DeviceMotion(device_id, axis_id, delta) => {},
  glu::Event::DeviceButtonDown(device_id, button_id) => {},
  glu::Event::DeviceButtonUp(device_id, button_id) => {},
  glu::Event::DeviceKeyDown(device_id, scan_code, virtual_key_code) => {},
  glu::Event::DeviceKeyUp(device_id, scan_code, virtual_key_code) => {},
  glu::Event::DeviceText(device_id, event_char, valid_char) => {},

  glu::Event::HiDPIFactorChanged(factor) => {},
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
