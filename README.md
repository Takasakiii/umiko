# Umiko
Permite usar a WinApi para criar facilmente hotkeys globais.

[Documentação](https://docs.rs/umiko)

[![Crates.io](https://img.shields.io/crates/v/umiko?style=flat-square)](https://crates.io/crates/umiko)
[![Docs](https://img.shields.io/docsrs/umiko?style=flat-square)](https://docs.rs/umiko)
[![Licença MIT](https://img.shields.io/github/license/Takasakiii/umiko?style=flat-square)](https://github.com/Takasakiii/umiko/blob/main/LICENSE)
 ## Exemplo de uso:
```rust
use umiko::{hotkeys::{HotKeys, KeyModifiers}, keys::Keys};

let mut hotkeys = HotKeys::new();
hotkeys.add(KeyModifiers::MOD_CONTROL | KeyModifiers::MOD_ALT, Keys::H, || {
    println!("Control + alt + h acionado!");
});
hotkeys.handle();
```
