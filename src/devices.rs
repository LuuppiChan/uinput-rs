use libc::input_id;

use crate::{
    Device, UInputUserDevice,
    key_codes::{
        ABS_MT_POSITION_X, ABS_MT_POSITION_Y, ABS_MT_SLOT, ABS_MT_TRACKING_ID, ABS_X, ABS_Y,
    },
    key_events::{
        ABS_MT_POSITION_X_EVENT, ABS_MT_POSITION_Y_EVENT, ABS_MT_PRESSURE_EVENT, ABS_MT_SLOT_EVENT,
        ABS_MT_TOUCH_MAJOR_EVENT, ABS_MT_TOUCH_MINOR_EVENT, ABS_MT_TRACKING_ID_EVENT, ABS_X_EVENT,
        ABS_Y_EVENT, BTN_TOOL_FINGER_EVENT, BTN_TOUCH_EVENT,
    },
    name_from_str,
};

/// Creates a multitouch touchscreen device
pub fn touchscreen(max_x: i32, max_y: i32) -> Device {
    let mut absmax = [0; 64];

    absmax[ABS_X as usize] = max_x;
    absmax[ABS_Y as usize] = max_y;

    absmax[ABS_MT_POSITION_X as usize] = max_x;
    absmax[ABS_MT_POSITION_Y as usize] = max_y;

    absmax[ABS_MT_SLOT as usize] = 9; // 10 fingers
    absmax[ABS_MT_TRACKING_ID as usize] = 65535; // Tracking IDs just need to be unique per contact until release.

    let info = UInputUserDevice {
        name: name_from_str("Multitouch touchscreen").unwrap(),
        id: input_id {
            bustype: 0x03,
            vendor: 0x1234,
            product: 0x5678,
            version: 1,
        },
        absmax,
        ..Default::default()
    };

    let events = [
        ABS_X_EVENT,
        ABS_Y_EVENT,
        BTN_TOUCH_EVENT,
        BTN_TOOL_FINGER_EVENT,
        ABS_MT_SLOT_EVENT,
        ABS_MT_TRACKING_ID_EVENT,
        ABS_MT_POSITION_X_EVENT,
        ABS_MT_POSITION_Y_EVENT,
        // optional
        ABS_MT_PRESSURE_EVENT,
        ABS_MT_TOUCH_MAJOR_EVENT,
        ABS_MT_TOUCH_MINOR_EVENT,
    ];

    Device::new_custom(&events, &info).expect("Error creating device")
}
