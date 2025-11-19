# uinput-rs
API for uinput kernel module.

# Why?
I wanted a similar kind of API as this: [python-uinput](https://github.com/pyinput/python-uinput)

# Examples
You can use the constants from another library or just plain numbers.
## Mouse
```rust
use std::{thread::sleep, time::Duration};

use uinput_rs::Device;


fn main() {
    // Enable these events for the device
    // (1, 272): BTN_MOUSE
    // (2, 0): REL_X
    // (2, 1): REL_Y
    let events = vec![(1, 272), (2, 0), (2, 1)];

    // Create device with the default configuration.
    // Enable the events for the device by passing them.
    let device = Device::new(events).unwrap();

    // Wait for the kernel to initialize the device.
    sleep(Duration::from_millis(100));

    for _ in 0..1000 {
        // move to the right
        device.emit_silent(2, 0, 1);
        // Fire the events
        device.sync_silent();

        sleep(Duration::from_millis(1));
    }
    // Mouse down
    device.emit_silent(1, 272, 1);
    device.sync_silent();

    sleep(Duration::from_millis(5));
    // Mouse up
    device.emit_silent(1, 272, 0);
    device.sync_silent();
}
```
