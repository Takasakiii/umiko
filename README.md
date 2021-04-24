# Umiko
Permite usar a WinApi para criar facilmente hotkeys globais.

 - [Instale via crates.io](https://crates.io/crates/umiko)
 - [Documentação](https://docs.rs/umiko)

 ## Exemplo de uso:
```rs
use umiko::hotkeys::{HotKeys, KeyModifies};

let mut hotkeys = HotKeys::new();
hotkeys.add(KeyModifies::MOD_CONTROL | KeyModifies::MOD_ALT, 'h', || {
    println!("Control + alt + h acionado!");
});
hotkeys.handle();
```
