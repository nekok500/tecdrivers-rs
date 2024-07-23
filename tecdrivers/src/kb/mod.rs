use anyhow::Result;
use bitflags::bitflags;
use rusb::{Context, DeviceHandle};

pub mod poskeyboard;

bitflags! {
    pub struct KeyFlags: u8 {
        const LEFT_SHIFT =  1 << 0;
        const RIGHT_SHIFT = 1 << 1;
        const LEFT_ALT =    1 << 2;
        const RIGHT_ALT =   1 << 3;
        const LEFT_CTRL =   1 << 4;
        const RIGHT_CTRL =  1 << 5;
    }
}

impl KeyFlags {
    pub fn to_payload(&self, key: u8) -> u16 {
        (self.bits() as u16) << 8 | key as u16
    }
}

pub trait POSKB {
    /// USBデバイスを初期化する。
    fn init(handle: DeviceHandle<Context>, reset: bool) -> Result<Box<Self>>;
}

#[cfg(test)]
mod tests {
    use crate::kb::KeyFlags;

    #[test]
    fn check_bit_flags() {
        assert_eq!(KeyFlags::LEFT_SHIFT.to_payload(73), 0x149);
        assert_eq!(KeyFlags::LEFT_SHIFT.to_payload(63), 0x13f);
        assert_eq!(KeyFlags::LEFT_SHIFT.to_payload(28), 0x11c);
        assert_eq!(KeyFlags::LEFT_ALT.to_payload(8), 0x408);

        assert_eq!(
            (KeyFlags::LEFT_CTRL | KeyFlags::LEFT_ALT).to_payload(107),
            0x146b
        );
    }
}
