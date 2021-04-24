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
/// use umiko::keys::{check_key_state, KeyState};
/// let capslock_key_code = 0x14;
/// if check_key_state(capslock_key_code) == KeyState::Lock {
///    // tecla capslock esta em estado Lock
/// }
/// ```
pub fn check_key_state(key_code: i32) -> KeyState {
    unsafe {
        let key_state = GetKeyState(key_code);
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
/// use umiko::keys::is_pressed;
/// let capslock_key_code = 0x14;
/// if is_pressed(capslock_key_code) {
///    // tecla capslock esta sendo pressionada
/// }
/// ```
pub fn is_pressed(key_code: i32) -> bool {
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
/// use umiko::keys::is_locked;
/// let capslock_key_code = 0x14;
/// if is_locked(capslock_key_code) {
///    // tecla capslock esta ativada
/// }
/// ```
pub fn is_locked(key_code: i32) -> bool {
    match check_key_state(key_code) {
        KeyState::PressAndLock => true,
        KeyState::Lock => true,
        _ => false
    }
}
