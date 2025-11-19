// Cross-reference this with other implementations
use std::{
    ffi::CString, fs::{File, OpenOptions}, io::{self, Result}, os::fd::{AsRawFd, RawFd}
};

use libc::{input_event, timeval, uinput_user_dev};

// These constants come from <linux/uinput.h>
//
pub const UI_SET_EVBIT: u64 = 0x40045564;
pub const UI_SET_KEYBIT: u64 = 0x40045565;
pub const UI_SET_RELBIT: u64 = 0x40045566;
pub const UI_SET_ABSBIT: u64 = 0x40045567;
pub const UI_SET_MSCBIT: u64 = 0x40045568;
pub const UI_SET_LEDBIT: u64 = 0x40045569;
pub const UI_SET_SNDBIT: u64 = 0x4004556A;
pub const UI_SET_FFBIT: u64 = 0x4004556B;
pub const UI_SET_PHYS: u64 = 0x4004556C;
pub const UI_SET_SWBIT: u64 = 0x4004556D;

// For absolute axes setup (ABS ranges: min/max/etc.)
pub const UI_ABS_SETUP: u64 = 0x401855CB;

pub const UI_DEV_CREATE: u64 = 0x5501;
pub const UI_DEV_DESTROY: u64 = 0x5502;

pub const EV_KEY: u16 = 0x01;
pub const EV_SYN: u16 = 0x00;
pub const EV_REL: u16 = 0x02;
pub const EV_ABS: u16 = 0x03;
pub const EV_MSC: u16 = 0x04;
pub const EV_SW: u16 = 0x05;
pub const EV_LED: u16 = 0x11;
pub const EV_SND: u16 = 0x12;
pub const EV_REP: u16 = 0x14;
pub const EV_FF: u16 = 0x15;

pub const SYN_REPORT: u16 = 0;

fn ioctl(fd: RawFd, req: u64, arg: u64) -> Result<()> {
    let ret = unsafe { libc::ioctl(fd, req, arg) };
    if ret < 0 {
        Err(io::Error::last_os_error())
    } else {
        Ok(())
    }
}

/// Open the device writer
fn open_uinput() -> Result<File> {
    OpenOptions::new()
        .read(true)
        .write(true)
        .open("/dev/uinput")
}

/// Enable specific key for the device.
/// event_type: Event type. It's a really small number.
/// You can use the constants here EV_*
fn enable_key(fd: RawFd, event_type: u64, key: u64) -> Result<()> {
    ioctl(fd, UI_SET_EVBIT, event_type)?;

    let req = match event_type as u16 {
        EV_KEY => UI_SET_KEYBIT,
        EV_REL => UI_SET_RELBIT,
        EV_ABS => UI_SET_ABSBIT,
        EV_MSC => UI_SET_MSCBIT,
        EV_LED => UI_SET_LEDBIT,
        EV_SND => UI_SET_SNDBIT,
        EV_SW => UI_SET_SWBIT,
        EV_FF => UI_SET_FFBIT,
        _ => {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                format!("Unsupported event type {event_type}"),
            ));
        }
    };
    ioctl(fd, req, key)
}

fn write_device(fd: RawFd, device: &UInputUserDevice) {
    let device = device.as_uinput_user_dev();
    unsafe {
        let ptr = &device as *const _ as *const u8;
        let size = std::mem::size_of::<uinput_user_dev>();
        libc::write(fd, ptr as *const _, size);
    }
}

fn send_event(fd: RawFd, event_type: u16, code: u16, value: i32) -> Result<()> {
    let ev = input_event {
        time: timeval {
            tv_sec: 0,
            tv_usec: 0,
        },
        type_: event_type,
        code,
        value,
    };

    send_event_custom(fd, ev)
}

fn send_event_custom(fd: RawFd, event: input_event) -> Result<()> {
    let size = std::mem::size_of::<input_event>();
    let ptr = &event as *const input_event as *const _;
    let written = unsafe { libc::write(fd, ptr, std::mem::size_of::<input_event>()) };
    if written == size as isize {
        Ok(())
    } else if written >= 0 {
        // Extremely unlikely for a char device, but still correct handling.
        Err(io::Error::new(
            io::ErrorKind::WriteZero,
            format!("partial write: {written} / {size} bytes"),
        ))
    } else {
        Err(io::Error::last_os_error())
    }
}


pub fn name_from_str(name: &str) -> Result<[i8; 80]> {
    let mut name_list = [0i8; 80];

    let c_name = CString::new(name).map_err(|_| {
        io::Error::new(io::ErrorKind::InvalidInput, "Device name contains null byte")
    })?;

    let bytes = c_name.as_bytes_with_nul();
    if bytes.len() > name_list.len() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Device name too long"));
    }

    for (i, &b) in bytes.iter().enumerate() {
        name_list[i] = b as i8;
    }

    Ok(name_list)
}

#[repr(C)]
pub struct UInputUserDevice {
    pub name: [i8; 80],
    pub id: libc::input_id,
    pub ff_effects_max: u32,
    pub absmax: [i32; 64],
    pub absmin: [i32; 64],
    pub absfuzz: [i32; 64],
    pub absflat: [i32; 64],
}

impl UInputUserDevice {
    pub fn as_uinput_user_dev(&self) -> uinput_user_dev {
        uinput_user_dev { name: self.name, id: self.id, ff_effects_max: self.ff_effects_max, absmax: self.absmax, absmin: self.absmin, absfuzz: self.absfuzz, absflat: self.absflat }
    }
}

impl Default for UInputUserDevice {
    fn default() -> Self {
        Self {
            name: name_from_str("rusty-device").unwrap(),
            id: libc::input_id {
                bustype: 0x03, // BUS_USB
                vendor: 0x1,
                product: 0x1,
                version: 1,
            },
            ff_effects_max: 0,
            absmax: [0; 64],
            absmin: [0; 64],
            absfuzz: [0; 64],
            absflat: [0; 64],
        }
    }
}

pub struct Device {
    file: File,
}

impl Device {
    /// Create new default device
    pub fn new(events: Vec<(u64, u64)>) -> Result<Self> {
        let file = open_uinput()?;

        for (event_type, key) in events {
            enable_key(file.as_raw_fd(), event_type, key)?;
        }

        write_device(file.as_raw_fd(), &UInputUserDevice::default());

        ioctl(file.as_raw_fd(), UI_DEV_CREATE, 0)?;

        Ok(Device { file })
    }

    /// Create new device with custom properties
    pub fn new_custom(events: Vec<(u64, u64)>, device: &UInputUserDevice) -> Result<Self> {
        let file = open_uinput()?;

        for (event_type, key) in events {
            enable_key(file.as_raw_fd(), event_type, key)?;
        }

        write_device(file.as_raw_fd(), device);

        ioctl(file.as_raw_fd(), UI_DEV_CREATE, 0)?;

        Ok(Device { file })
    }

    /// Remember to call sync to send the events.
    pub fn emit(&self, event_type: u16, code: u16, value: i32) -> Result<()> {
        send_event(self.file.as_raw_fd(), event_type, code, value)
    }

    pub fn emit_silent(&self, event_type: u16, code: u16, value: i32) {
        self.emit(event_type, code, value).unwrap();
    }

    /// Remember to call sync to send the events.
    pub fn emit_custom(&self, event: input_event) -> Result<()> {
        send_event_custom(self.file.as_raw_fd(), event)
    }

    pub fn sync(&self) -> Result<()> {
        self.emit(EV_SYN, SYN_REPORT, 0)
    }

    pub fn sync_silent(&self) {
        self.sync().unwrap()
    }

    /// Destroys the current device.
    /// Don't call more than once.
    /// This is called on drop. (Automatically)
    unsafe fn destroy(&mut self) -> Result<()> {
        ioctl(self.file.as_raw_fd(), UI_DEV_DESTROY, 0)
    }
}

impl Drop for Device {
    fn drop(&mut self) {
        let _ = unsafe { self.destroy() };
    }
}
