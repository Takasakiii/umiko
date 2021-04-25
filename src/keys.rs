use winapi::um::winuser::GetKeyState;

/// Representa um estado de uma key.
#[derive(Debug, PartialEq)]
pub enum KeyState {
    /// Botão pressionado.
    Press = -128,
    /// Botão precionado com o lock ativado (para teclas como capslock).
    PressAndLock = -127,
    /// Representa apenas o estado de lock ativo (para teclas como capslock).
    Lock = 1,
    /// Tecla não esta pressionada nem com lock.
    None = 0
}

pub struct Keys;
impl Keys {
    pub const BACKSPACE: u32 = 0x08;
    pub const TAB: u32 = 0x09;
    pub const ENTER: u32 = 0x0D;
    pub const ESCAPE: u32 = 0x1B;
    pub const SPACE: u32 = 0x20;
    pub const HOME: u32 = 0x24;
    pub const LEFT: u32 = 0x25;
    pub const UP: u32 = 0x26;
    pub const RIGHT: u32 = 0x27;
    pub const DOWN: u32 = 0x28;
    pub const INDERT: u32 = 0x2D;
    pub const DELETE: u32 = 0x2E;
    pub const NUMROW_0: u32 = 0x30;
    pub const NUMROW_1: u32 = 0x31;
    pub const NUMROW_2: u32 = 0x32;
    pub const NUMROW_3: u32 = 0x33;
    pub const NUMROW_4: u32 = 0x34;
    pub const NUMROW_5: u32 = 0x35;
    pub const NUMROW_6: u32 = 0x36;
    pub const NUMROW_7: u32 = 0x37;
    pub const NUMROW_8: u32 = 0x38;
    pub const NUMROW_9: u32 = 0x39;
    pub const A: u32 = 0x41;
    pub const B: u32 = 0x42;
    pub const C: u32 = 0x43;
    pub const D: u32 = 0x44;
    pub const E: u32 = 0x45;
    pub const F: u32 = 0x46;
    pub const G: u32 = 0x47;
    pub const H: u32 = 0x48;
    pub const I: u32 = 0x49;
    pub const J: u32 = 0x4A;
    pub const K: u32 = 0x4B;
    pub const L: u32 = 0x4C;
    pub const M: u32 = 0x4D;
    pub const N: u32 = 0x4E;
    pub const O: u32 = 0x4F;
    pub const P: u32 = 0x50;
    pub const Q: u32 = 0x51;
    pub const R: u32 = 0x52;
    pub const S: u32 = 0x53;
    pub const T: u32 = 0x54;
    pub const U: u32 = 0x55;
    pub const V: u32 = 0x56;
    pub const W: u32 = 0x57;
    pub const X: u32 = 0x58;
    pub const Y: u32 = 0x59;
    pub const Z: u32 = 0x5A;
    pub const NUMPAD_0: u32 = 0x60;
    pub const NUMPAD_1: u32 = 0x61;
    pub const NUMPAD_2: u32 = 0x62;
    pub const NUMPAD_3: u32 = 0x63;
    pub const NUMPAD_4: u32 = 0x64;
    pub const NUMPAD_5: u32 = 0x65;
    pub const NUMPAD_6: u32 = 0x66;
    pub const NUMPAD_7: u32 = 0x67;
    pub const NUMPAD_8: u32 = 0x68;
    pub const NUMPAD_9: u32 = 0x69;
    pub const F1: u32= 0x70;
    pub const F2: u32= 0x71;
    pub const F3: u32= 0x72;
    pub const F4: u32= 0x73;
    pub const F5: u32= 0x74;
    pub const F6: u32= 0x75;
    pub const F7: u32= 0x76;
    pub const F8: u32= 0x77;
    pub const F9: u32= 0x78;
    pub const F10: u32 = 0x79;
    pub const F11: u32 = 0x7A;
    pub const F12: u32 = 0x7B;
    pub const F13: u32 = 0x7C;
    pub const F14: u32 = 0x7D;
    pub const F15: u32 = 0x7E;
    pub const F16: u32 = 0x7F;
    pub const F17: u32 = 0x80;
    pub const F18: u32 = 0x81;
    pub const F19: u32 = 0x82;
    pub const F20: u32 = 0x83;
    pub const F21: u32 = 0x84;
    pub const F22: u32 = 0x85;
    pub const F23: u32 = 0x86;
    pub const F24: u32 = 0x87;
    pub const NUMLOCK: u32 = 0x90;
    pub const SCROLL_LOCK: u32 = 0x91;
    pub const CAPS_LOCK: u32 = 0x14;
    pub const L_SHIFT: u32 = 0xA0;
    pub const R_SHIFT: u32 = 0xA1;
    pub const L_CONTROL: u32 = 0xA2;
    pub const R_CONTROL: u32 = 0xA3;
}

/// Pega o atual estado de uma tecla, retornando seu estado de forma não tratada.
///
/// `key_code`: é o codigo ascii da tecla que deseja pegar seu estado.
///
/// # Exemplo:
/// ```
/// use umiko::keys::{check_key_state, KeyState, Keys};
/// if check_key_state(Keys::CAPS_LOCK) == KeyState::Lock {
///    // tecla capslock esta em estado Lock
/// }
/// ```
pub fn check_key_state(key_code: u32) -> KeyState {
    unsafe {
        let key_state = GetKeyState(key_code as i32);
        match key_state {
            -128 => KeyState::Press,
            -127 => KeyState::PressAndLock,
            1 => KeyState::Lock,
            0 => KeyState::None,
            _ => panic!("An unexpected error occurred in the key states")
        }
    }
}


/// Verifica se a tecla esta em um dos estados considerados pressionados.
///
/// `key_code`: é o codigo ascii da tecla que deseja pegar seu estado.
///
/// # Exemplo:
/// ```
/// use umiko::keys::{Keys, is_pressed};
/// if is_pressed(Keys::CAPS_LOCK) {
///    // tecla capslock esta sendo pressionada
/// }
/// ```
pub fn is_pressed(key_code: u32) -> bool {
    match check_key_state(key_code) {
        KeyState::None => false,
        KeyState::Lock => false,
        _ => true
    }
}

/// Verifica se a tecla esta em um dos estados considerados ativado.
///
/// `key_code`: é o codigo ascii da tecla que deseja pegar seu estado.
///
/// # Exemplo:
/// ```
/// use umiko::keys::{Keys, is_locked};
/// if is_locked(Keys::CAPS_LOCK) {
///    // tecla capslock esta ativada
/// }
/// ```
pub fn is_locked(key_code: u32) -> bool {
    match check_key_state(key_code) {
        KeyState::PressAndLock => true,
        KeyState::Lock => true,
        _ => false
    }
}
