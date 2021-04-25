# Umiko
Permite usar a WinApi para criar facilmente hotkeys globais.

[Documentação](https://docs.rs/umiko)


[![Crates.io](https://img.shields.io/crates/v/umiko?style=flat-square)](https://crates.io/crates/umiko)
[![Docs](https://img.shields.io/docsrs/umiko?style=flat-square)](https://docs.rs/umiko)
https://img.shields.io/github/license/Takasakiii/umiko?style=flat-square

 ## Exemplo de uso:
```rust
use umiko::hotkeys::{HotKeys, KeyModifies};

let mut hotkeys = HotKeys::new();
hotkeys.add(KeyModifies::MOD_CONTROL | KeyModifies::MOD_ALT, 'h', || {
    println!("Control + alt + h acionado!");
});
hotkeys.handle();
```
