use std::collections::HashMap;

use glutin::event::{VirtualKeyCode, MouseButton, ElementState};

/*
* https://github.com/rust-windowing/glutin/issues/708
* Code for polling events
*/

/// Keeps track of which keys have been pressed.
pub struct EventState {
    state: HashMap<KeyCode, ElementState>,
}
impl EventState {
    /// Constructs a new KeyboardState with all the keys released.
    pub fn new() -> EventState {
        EventState {
            state: HashMap::new(),
        }
    }

    /// Returns true if `key` is pressed.
    pub fn is_pressed(&self, key: &KeyCode) -> bool {
        self.state.get(key).map(|&s| s == ElementState::Pressed).unwrap_or(false)
    }
    /// Returns true if `key` is released.
    pub fn is_released(&self, key: &KeyCode) -> bool {
        !self.is_pressed(key)
    }

    /// Processes a keyboard event and updated the internal state.
    pub fn process_event(&mut self, code: KeyCode, element_state: ElementState) {
        match element_state {
            ElementState::Pressed => {
                self.state.insert(code, element_state);
            },
            ElementState::Released => {
                self.state.remove(&code);
            }
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub enum KeyCode {
    MouseLeft,
    MouseRight,
    MouseMiddle,
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    Space,
    LControl,
    LShift,
    RControl,
    RShift,
    Other(u8),
    Error,
}

impl From<MouseButton> for KeyCode {
    fn from(other: MouseButton) -> Self {
        match other {
            MouseButton::Left => KeyCode::MouseLeft,
            MouseButton::Right => KeyCode::MouseRight,
            MouseButton::Middle => KeyCode::MouseMiddle,
            MouseButton::Other(code) => KeyCode::Other(code),
        }
    }
}

impl From<VirtualKeyCode> for KeyCode {
    fn from(other: VirtualKeyCode) -> Self {
        match other {
            VirtualKeyCode::A           => KeyCode::A,
            VirtualKeyCode::B           => KeyCode::B,
            VirtualKeyCode::C           => KeyCode::C,
            VirtualKeyCode::D           => KeyCode::D,
            VirtualKeyCode::E           => KeyCode::E,
            VirtualKeyCode::F           => KeyCode::F,
            VirtualKeyCode::G           => KeyCode::G,
            VirtualKeyCode::H           => KeyCode::H,
            VirtualKeyCode::I           => KeyCode::I,
            VirtualKeyCode::J           => KeyCode::J,
            VirtualKeyCode::K           => KeyCode::K,
            VirtualKeyCode::L           => KeyCode::L,
            VirtualKeyCode::M           => KeyCode::M,
            VirtualKeyCode::N           => KeyCode::N,
            VirtualKeyCode::O           => KeyCode::O,
            VirtualKeyCode::P           => KeyCode::P,
            VirtualKeyCode::Q           => KeyCode::Q,
            VirtualKeyCode::R           => KeyCode::R,
            VirtualKeyCode::S           => KeyCode::S,
            VirtualKeyCode::T           => KeyCode::T,
            VirtualKeyCode::U           => KeyCode::U,
            VirtualKeyCode::V           => KeyCode::V,
            VirtualKeyCode::W           => KeyCode::W,
            VirtualKeyCode::X           => KeyCode::X,
            VirtualKeyCode::Y           => KeyCode::Y,
            VirtualKeyCode::Z           => KeyCode::Z,   
            VirtualKeyCode::Space       => KeyCode::Space,    
            VirtualKeyCode::LControl    => KeyCode::LControl,      
            VirtualKeyCode::LShift      => KeyCode::LShift,  
            VirtualKeyCode::RControl    => KeyCode::RControl,      
            VirtualKeyCode::RShift      => KeyCode::RShift, 
            _ => KeyCode::Error
        }
    }
}