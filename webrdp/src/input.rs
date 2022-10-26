use rdp::core::event::{KeyboardEvent as RdpKeyBorad, PointerButton, PointerEvent, RdpEvent};
use tracing::trace;
use web_sys::{KeyboardEvent, MouseEvent};

#[derive(Debug)]
pub enum MouseEventType {
    Down,
    Up,
    Move,
}

#[derive(Debug)]

pub enum KeyEventType {
    Down,
    Up,
}

pub enum InputEvent {
    Mouse(MouseEvent, MouseEventType),
    Keyboard(KeyboardEvent, KeyEventType),
    KeyCode(u16, KeyEventType),
}

fn to_scancode(code: &str) -> u16 {
    match code {
        "Escape" => 0x0001,
        "Digit1" => 0x0002,
        "Digit2" => 0x0003,
        "Digit3" => 0x0004,
        "Digit4" => 0x0005,
        "Digit5" => 0x0006,
        "Digit6" => 0x0007,
        "Digit7" => 0x0008,
        "Digit8" => 0x0009,
        "Digit9" => 0x000A,
        "Digit0" => 0x000B,
        "Minus" => 0x000C,
        "Equal" => 0x000D,
        "Backspace" => 0x000E,
        "Tab" => 0x000F,
        "KeyQ" => 0x0010,
        "KeyW" => 0x0011,
        "KeyE" => 0x0012,
        "KeyR" => 0x0013,
        "KeyT" => 0x0014,
        "KeyY" => 0x0015,
        "KeyU" => 0x0016,
        "KeyI" => 0x0017,
        "KeyO" => 0x0018,
        "KeyP" => 0x0019,
        "BracketLeft" => 0x001A,
        "BracketRight" => 0x001B,
        "Enter" => 0x001C,
        "ControlLeft" => 0x001D,
        "KeyA" => 0x001E,
        "KeyS" => 0x001F,
        "KeyD" => 0x0020,
        "KeyF" => 0x0021,
        "KeyG" => 0x0022,
        "KeyH" => 0x0023,
        "KeyJ" => 0x0024,
        "KeyK" => 0x0025,
        "KeyL" => 0x0026,
        "Semicolon" => 0x0027,
        "Quote" => 0x0028,
        "Backquote" => 0x0029,
        "ShiftLeft" => 0x002A,
        "Backslash" => 0x002B,
        "KeyZ" => 0x002C,
        "KeyX" => 0x002D,
        "KeyC" => 0x002E,
        "KeyV" => 0x002F,
        "KeyB" => 0x0030,
        "KeyN" => 0x0031,
        "KeyM" => 0x0032,
        "Comma" => 0x0033,
        "Period" => 0x0034,
        "Slash" => 0x0035,
        "ShiftRight" => 0x0036,
        "NumpadMultiply" => 0x0037,
        "AltLeft" => 0x0038,
        "Space" => 0x0039,
        "CapsLock" => 0x003A,
        "F1" => 0x003B,
        "F2" => 0x003C,
        "F3" => 0x003D,
        "F4" => 0x003E,
        "F5" => 0x003F,
        "F6" => 0x0040,
        "F7" => 0x0041,
        "F8" => 0x0042,
        "F9" => 0x0043,
        "F10" => 0x0044,
        "Pause" => 0x0045,
        "ScrollLock" => 0x0046,
        "Numpad7" => 0x0047,
        "Numpad8" => 0x0048,
        "Numpad9" => 0x0049,
        "NumpadSubtract" => 0x004A,
        "Numpad4" => 0x004B,
        "Numpad5" => 0x004C,
        "Numpad6" => 0x004D,
        "NumpadAdd" => 0x004E,
        "Numpad1" => 0x004F,
        "Numpad2" => 0x0050,
        "Numpad3" => 0x0051,
        "Numpad0" => 0x0052,
        "NumpadDecimal" => 0x0053,
        "IntlBackslash" => 0x0056,
        "F11" => 0x0057,
        "F12" => 0x0058,
        "NumpadEqual" => 0x0059,
        "F13" => 0x0064,
        "F14" => 0x0065,
        "F15" => 0x0066,
        "F16" => 0x0067,
        "F17" => 0x0068,
        "F18" => 0x0069,
        "F19" => 0x006A,
        "F20" => 0x006B,
        "F21" => 0x006C,
        "F22" => 0x006D,
        "F23" => 0x006E,
        "KanaMode" => 0x0070,
        "Lang2" => 0x0071,
        "Lang1" => 0x0072,
        "IntlRo" => 0x0073,
        "F24" => 0x0076,
        "Convert" => 0x0079,
        "NonConvert" => 0x007B,
        "IntlYen" => 0x007D,
        "NumpadComma" => 0x007E,
        "MediaTrackPrevious" => 0xE010,
        "MediaTrackNext" => 0xE019,
        "NumpadEnter" => 0xE01C,
        "ControlRight" => 0xE01D,
        "AudioVolumeMute" => 0xE020,
        "LaunchApp2" => 0xE021,
        "MediaPlayPause" => 0xE022,
        "MediaStop" => 0xE024,
        "BrowserHome" => 0xE032,
        "NumpadDivide" => 0xE035,
        "PrintScreen" => 0xE037,
        "AltRight" => 0xE038,
        "NumLock" => 0xE045,
        // "Pause" => 0xE046,
        "Home" => 0xE047,
        "ArrowUp" => 0xE048,
        "PageUp" => 0xE049,
        "ArrowLeft" => 0xE04B,
        "ArrowRight" => 0xE04D,
        "End" => 0xE04F,
        "ArrowDown" => 0xE050,
        "PageDown" => 0xE051,
        "Insert" => 0xE052,
        "Delete" => 0xE053,
        "ContextMenu" => 0xE05D,
        "Power" => 0xE05E,
        "BrowserSearch" => 0xE065,
        "BrowserFavorites" => 0xE066,
        "BrowserRefresh" => 0xE067,
        "BrowserStop" => 0xE068,
        "BrowserForward" => 0xE069,
        "BrowserBack" => 0xE06A,
        "LaunchApp1" => 0xE06B,
        "LaunchMail" => 0xE06C,
        "MediaSelect" => 0xE06D,
        _ => 0x0000,
    }
}

impl From<InputEvent> for RdpEvent {
    fn from(i: InputEvent) -> Self {
        match i {
            InputEvent::Mouse(e, t) => RdpEvent::Pointer(PointerEvent {
                x: e.offset_x().try_into().unwrap_or(0),
                y: e.offset_y().try_into().unwrap_or(0),
                button: match e.button() {
                    0 => {
                        if let MouseEventType::Move = t {
                            PointerButton::None
                        } else {
                            PointerButton::Left
                        }
                    }
                    1 => PointerButton::Middle,
                    2 => PointerButton::Right,
                    _ => PointerButton::None,
                },
                down: matches!(t, MouseEventType::Down),
            }),
            InputEvent::Keyboard(e, t) => {
                trace!("Key {}, event {:?}", e.code(), t);
                RdpEvent::Key(RdpKeyBorad {
                    code: to_scancode(&e.code()),
                    down: matches!(t, KeyEventType::Down),
                })
            }
            InputEvent::KeyCode(e, t) => RdpEvent::Key(RdpKeyBorad {
                code: e,
                down: matches!(t, KeyEventType::Down),
            }),
        }
    }
}
