# uinput-rs
API for uinput kernel module with minimal guardrails.

# Why?
I wanted a similar kind of API as this: [python-uinput](https://github.com/pyinput/python-uinput)

And I need this for a Godot project and the existing libraries (that I looked at) have guardrails that would make using them really hard.

# Examples
You can use the constants from another library, just plain numbers or from the key_codes module.
## Mouse
```rust
use std::{thread::sleep, time::Duration};

use uinput_rs::{
    Device,
    key_codes::{BTN_MOUSE, REL_X, REL_Y},
};

fn main() {
    // Enable these events for the device
    let events = vec![BTN_MOUSE, REL_Y, REL_X];

    // Create device with the default configuration.
    // Enable the events for the device by passing them.
    let device = Device::new(events).unwrap();

    // Wait for the user space to initialize the device.
    sleep(Duration::from_millis(100));

    for _ in 0..1000 {
        // move to the right
        device.emit_key_code_silent(REL_X, 1);
        // Fire the events
        device.sync_silent();

        sleep(Duration::from_millis(1));
    }
    // Mouse down
    device.emit_key_code_silent(BTN_MOUSE, 1);
    device.sync_silent();

    sleep(Duration::from_millis(5));
    // Mouse up
    device.emit_key_code_silent(BTN_MOUSE, 0);
    device.sync_silent();
}
```

## Destroying the device
The device is automatically destroyed while dropping the variable, but if you want to drop it manually you can just use built-in functions for that.
```rust
use uinput_rs::{
    Device,
    key_codes::{KEY_R, KEY_S},
};

fn main() {
    let device = Device::new(vec![KEY_R, KEY_S]).unwrap();
    // Destroy the device the same way you drop variables.
    drop(device);

    println!("Device doesn't exist here anymore.");
} // It would normally be dropped here.
```
