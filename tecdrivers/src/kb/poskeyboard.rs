use std::time::Duration;

use anyhow::{bail, Result};
use rusb::{Context, DeviceHandle};

use crate::USBDevice;

use super::{KeyFlags, POSKB};

const REQUEST_TYPE: u8 = 0xc1;
const REQUEST: u8 = 219;
const TIMEOUT: Duration = Duration::from_millis(500);
const VID: u16 = 0x08a6;
const PID: u16 = 0x000c;
pub const POSKEYS: &[u8] = &[
    107, 106, 105, 104, 103, 102, 108, 109, 110, 111, // 1段目
    91, 90, 89, 88, 87, 86, 92, 93, 94, 95, // 2段目
    75, 74, 73, 72, 71, 70, 76, 77, 78, 79, // 3段目
    59, 58, 57, 56, 55, 54, 60, 61, 62, 63, // 4段目
    43, 42, 41, 40, 39, 38, 44, 45, 46, 47, // 5段目
    27, 26, 25, 24, 23, 22, 28, 29, 30, 31, // 6段目
    11, 10, 9, 8, 7, 6, 12, 13, 14, 15, // 7段目
];

pub struct USBPOSKeyboard {
    pub handle: DeviceHandle<Context>,
}

impl POSKB for USBPOSKeyboard {
    fn init(handle: DeviceHandle<Context>, reset: bool) -> Result<Box<Self>> {
        if reset {
            handle.reset()?;
        }

        Ok(Box::new(Self { handle }))
    }
}

impl USBDevice for USBPOSKeyboard {
    fn find(list: &rusb::DeviceList<Context>, reset: bool) -> Result<Option<Box<Self>>> {
        let Some(device) = list.iter().find(|d| {
            // TODO: error handling
            let desc = d.device_descriptor().expect("failed to get descriptor");
            desc.vendor_id() == VID && desc.product_id() == PID
        }) else {
            return Ok(None);
        };

        Ok(Some(Self::init(device.open()?, reset)?))
    }
}

impl USBPOSKeyboard {
    /// キー配置を設定します。
    ///
    /// # Arguments
    /// * `pos` - キーの位置
    /// * `key` - キーコード
    /// * `flags` - キーの修飾キー
    pub fn set_key(&self, pos: u8, key: u8, flags: KeyFlags) -> Result<()> {
        if !POSKEYS.contains(&pos) {
            bail!("invalid key position");
        }

        let buf = [0u8; 1];
        let num = self.handle.read_control(
            REQUEST_TYPE,
            REQUEST,
            flags.to_payload(pos),
            key as u16,
            &mut [0u8; 1],
            TIMEOUT,
        )?;
        assert_eq!(num, 1);
        assert_eq!(buf, [0x00]);

        Ok(())
    }
}
