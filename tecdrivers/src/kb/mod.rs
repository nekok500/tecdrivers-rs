use anyhow::Result;
use bitflags::bitflags;
use rusb::{Context, DeviceHandle};

pub mod poskeyboard;

bitflags! {
    pub struct KeyFlags: u16 {
        const LEFT_SHIFT =  0b00000001_00000000;
        const RIGHT_SHIFT = 0b00000010_00000000;
        const LEFT_ALT =    0b00000100_00000000;
        const RIGHT_ALT =   0b00001000_00000000;
        const LEFT_CTRL =   0b00010000_00000000;
        const RIGHT_CTRL =  0b00100000_00000000;
    }
}

pub trait POSKB {
    /// USBデバイスを初期化する。
    fn init(handle: DeviceHandle<Context>, reset: bool) -> Result<Box<Self>>;
}
