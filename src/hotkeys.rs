use winapi::{shared::windef::HWND, um::winuser::{RegisterHotKey, GetMessageW, UnregisterHotKey}};
use std::{collections::HashMap, mem::MaybeUninit};
use crate::keys::Keys;

bitflags! {
    /// Representa os modificadores de tecla suportado pela winapi.
    pub struct KeyModifiers: u32 {
        /// Nenhum modificador acionado.
        const NONE_MODIFIES = 0;
        /// Modificador de tecla `Alt`
        const MOD_ALT = 0x0001;
        /// Modificador de tecla `Ctrl`
        const MOD_CONTROL = 0x0002;
        /// Modificador de não permitir repetição de teclas
        ///
        /// **Não suportado no Windows Vista.**
        const MOD_NO_REPEAT = 0x4000;
        /// Modificador de tecla `Shift`
        const MOD_SHIFT = 0x0004;
        /// Modificador de tecla `Windows/Win/Super`
        const MOD_WIN = 0x0008;
    }
}

/// Referencia a uma hotkey registrada no sistema.
pub struct HotKeyRegister {
    id: i32
}

/// Responsavel por gerenciar os webhooks do programa.
pub struct HotKeys {
    id: i32,
    callbacks: HashMap<i32, Box<dyn Fn() -> () + 'static>>
}


/// Implementação padrao da struct.
impl HotKeys {
    /// Cria uma nova intancia de `HotKeys`.
    pub fn new() -> Self {
        Self {
            id: 0,
            callbacks: HashMap::new()
        }
    }

    /// Adiciona uma nova hotkey.
    ///
    /// `key_modifies`: Modificadores de teclado desejado, para usar mais de um utilize os operadores `|` e `&`;
    ///
    /// `key`: Caractere representa a tecla desejada;
    ///
    /// `callback`: Função que será chamada quando a hotkey ser acionada.
    ///
    /// Retorna o registro da hotkey adicionada.
    ///
    ///
    /// # Exemplo:
    /// ```
    /// use umiko::{hotkeys::{HotKeys, KeyModifiers}, keys::Keys};
    ///
    /// let mut hotkeys = HotKeys::new();
    /// hotkeys.add(KeyModifiers::MOD_CONTROL | KeyModifiers::MOD_ALT, Keys::H, || {
    ///     println!("Control + alt + h acionado!");
    /// });
    /// hotkeys.handle();
    pub fn add<F>(&mut self, key_modifies: KeyModifiers, key: u32, callback: F) -> HotKeyRegister
    where
        F: Fn() -> () + 'static
    {
        unsafe {
            RegisterHotKey(0 as HWND, self.id, key_modifies.bits(), key);
        }

        self.callbacks.insert(self.id, Box::new(callback));
        let hotkey = HotKeyRegister {
            id: self.id
        };

        self.id += 1;
        hotkey
    }

    /// Recebe os eventos da winapi e redireciona aos devidos callbacks, deve sempre ser a ultima instrução da thread.
    pub fn handle(&self) {
        unsafe {
            let mut msg = MaybeUninit::zeroed()
                .assume_init();
            while GetMessageW(&mut msg, 0 as HWND, 0, 0) != 0 {
                if msg.message == 786 {
                    let id = msg.wParam as i32;
                    if let Some(event) = self.callbacks.get(&id) {
                        event();
                    }
                }
            }
        }
    }

    /// Remove uma hotkey do handler.
    ///
    /// `register_hotkey`: Referencia da hotkey registrada.
    ///
    /// # Exemplo:
    /// ```
    /// use umiko::{hotkeys::{HotKeys, KeyModifiers}, keys::Keys};
    ///
    /// let mut hotkeys = HotKeys::new();
    /// let register = hotkeys.add(KeyModifiers::MOD_CONTROL | KeyModifiers::MOD_ALT, Keys::H, || {
    ///     println!("Control + alt + h acionado!");
    /// });
    /// hotkeys.remove(register);
    pub fn remove(&mut self, register_hotkey: HotKeyRegister) {
        unsafe {
            UnregisterHotKey(0 as HWND, register_hotkey.id);
        }
        self.callbacks.remove(&register_hotkey.id);
    }
}
