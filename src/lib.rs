#[cfg(target_os = "windows")]
#[cfg(feature = "hotkeys")]
pub mod hotkeys;

#[cfg(target_os = "windows")]
#[cfg(feature = "keys")]
pub mod keys;

#[macro_use]
extern crate bitflags;

