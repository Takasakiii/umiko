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



/// Pega o atual estado de uma tecla, retornando seu estado de forma não tratada.
///
/// `key_code`: é o codigo ascii da tecla que deseja pegar seu estado.
///
/// # Exemplo:
/// ```
/// use umiko::{common::Keys, keys::{KeyState, check_key_state}};
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
/// use umiko::{common::Keys, keys::is_pressed};
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
/// use umiko::{common::Keys, keys::is_locked};
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
