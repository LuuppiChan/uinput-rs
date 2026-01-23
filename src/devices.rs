use std::{io::Error, result::Result};

use libc::input_id;

use crate::{
    Device, UInputUserDevice,
    key_codes::{
        ABS_MT_POSITION_X, ABS_MT_POSITION_Y, ABS_MT_PRESSURE, ABS_MT_SLOT, ABS_MT_TOUCH_MAJOR,
        ABS_MT_TOUCH_MINOR, ABS_MT_TRACKING_ID, ABS_PRESSURE, ABS_TILT_X, ABS_TILT_Y, ABS_X, ABS_Y,
    },
    key_events::{
        ABS_MT_POSITION_X_EVENT, ABS_MT_POSITION_Y_EVENT, ABS_MT_PRESSURE_EVENT, ABS_MT_SLOT_EVENT,
        ABS_MT_TOUCH_MAJOR_EVENT, ABS_MT_TOUCH_MINOR_EVENT, ABS_MT_TRACKING_ID_EVENT,
        ABS_PRESSURE_EVENT, ABS_TILT_X_EVENT, ABS_TILT_Y_EVENT, ABS_X_EVENT, ABS_Y_EVENT,
        BTN_EXTRA_EVENT, BTN_LEFT_EVENT, BTN_MIDDLE_EVENT, BTN_RIGHT_EVENT, BTN_SIDE_EVENT,
        BTN_TOUCH_EVENT, REL_HWHEEL_EVENT, REL_HWHEEL_HI_RES_EVENT, REL_WHEEL_EVENT,
        REL_WHEEL_HI_RES_EVENT, REL_X_EVENT, REL_Y_EVENT,
    },
    name_from_str,
};

pub const TOUCHSCREEN_EVENTS: [(u64, u64); 10] = [
    ABS_X_EVENT,
    ABS_Y_EVENT,
    BTN_TOUCH_EVENT,
    // BTN_TOOL_FINGER_EVENT,
    ABS_MT_SLOT_EVENT,
    ABS_MT_TRACKING_ID_EVENT,
    ABS_MT_POSITION_X_EVENT,
    ABS_MT_POSITION_Y_EVENT,
    ABS_MT_PRESSURE_EVENT,
    ABS_MT_TOUCH_MAJOR_EVENT,
    ABS_MT_TOUCH_MINOR_EVENT,
];
pub const MOUSE_EVENTS: [(u64, u64); 11] = [
    BTN_LEFT_EVENT,
    BTN_RIGHT_EVENT,
    REL_X_EVENT,
    REL_Y_EVENT,
    BTN_MIDDLE_EVENT,
    BTN_SIDE_EVENT,
    BTN_EXTRA_EVENT,
    REL_WHEEL_EVENT,
    REL_WHEEL_HI_RES_EVENT,
    REL_HWHEEL_EVENT,
    REL_HWHEEL_HI_RES_EVENT,
];
pub const ABSOLUTE_EVENTS: [(u64, u64); 14] = [
    ABS_X_EVENT,
    ABS_Y_EVENT,
    ABS_PRESSURE_EVENT,
    ABS_TILT_X_EVENT,
    ABS_TILT_Y_EVENT,
    BTN_LEFT_EVENT,
    BTN_MIDDLE_EVENT,
    BTN_RIGHT_EVENT,
    BTN_SIDE_EVENT,
    BTN_EXTRA_EVENT,
    REL_WHEEL_EVENT,
    REL_WHEEL_HI_RES_EVENT,
    REL_HWHEEL_EVENT,
    REL_HWHEEL_HI_RES_EVENT,
];

/// Creates a multitouch touchscreen device.
pub fn touchscreen(
    name: &str,
    max_x: i32,
    max_y: i32,
    max_pressure: i32,
    max_minor: i32,
    max_major: i32,
) -> Result<Device, Error> {
    let mut absmax = [0; 64];

    absmax[ABS_X as usize] = max_x;
    absmax[ABS_Y as usize] = max_y;

    absmax[ABS_MT_POSITION_X as usize] = max_x;
    absmax[ABS_MT_POSITION_Y as usize] = max_y;

    absmax[ABS_MT_SLOT as usize] = 9; // 10 fingers
    absmax[ABS_MT_TRACKING_ID as usize] = 65535; // Tracking IDs just need to be unique per contact until release.
    absmax[ABS_MT_PRESSURE as usize] = max_pressure;
    absmax[ABS_MT_TOUCH_MAJOR as usize] = max_major;
    absmax[ABS_MT_TOUCH_MINOR as usize] = max_minor;

    let info = UInputUserDevice {
        name: name_from_str(name).unwrap(),
        id: input_id {
            bustype: 0x03,
            vendor: 0x1234,
            product: 0x5678,
            version: 1,
        },
        absmax,
        ..Default::default()
    };

    Device::new_custom(&TOUCHSCREEN_EVENTS, &info)
}

/// Creates a mouse device
pub fn mouse(name: &str) -> Result<Device, Error> {
    Device::new_custom(&MOUSE_EVENTS, &UInputUserDevice::with_name(name))
}

/// Simple absolute mouse device
pub fn absolute(name: &str, max_x: i32, max_y: i32) -> Result<Device, Error> {
    let mut absmax = [0; 64];
    let mut absmin = [0; 64];

    absmax[ABS_X as usize] = max_x;
    absmax[ABS_Y as usize] = max_y;

    absmax[ABS_PRESSURE as usize] = 2;
    absmax[ABS_TILT_X as usize] = 63;
    absmax[ABS_TILT_Y as usize] = 63;

    absmin[ABS_TILT_X as usize] = -64;
    absmin[ABS_TILT_Y as usize] = -64;

    let info = UInputUserDevice {
        name: name_from_str(name).unwrap(),
        id: input_id {
            bustype: 0,
            vendor: 0,
            product: 0,
            version: 1,
        },
        absmax,
        absmin,
        ..Default::default()
    };

    Device::new_custom(&ABSOLUTE_EVENTS, &info)
}
