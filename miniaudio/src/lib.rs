

pub mod engine;
pub mod sound;


/// The raw, unsafe FFI binding, in case you need that escape hatch or the safe layer doesn't provide something you need.
/// There is currently no safe layer for the `miniaudio-sys` crate, so this is the same as `miniaudio-sys`.
pub mod ffi {
    pub use miniaudio_sys::*;
}