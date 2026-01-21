// Rust API for the uinput kernel module with minimal guardrails.

// Expose these for convenience
pub use libc::{input_event, input_id, timeval, uinput_user_dev};

mod device;
/// Some key codes for convenience.
pub mod key_codes;
/// Some key tuples for simple event enabling
pub mod key_events;
/// Some key types for convenience.
pub mod key_types;
pub use device::*;
/// Random device presets
pub mod devices;
