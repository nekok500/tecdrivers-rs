use anyhow::Result;
use rusb::{Context, DeviceHandle};

pub mod liu7000;

pub trait LIU {
    fn init(handle: DeviceHandle<Context>, reset: bool) -> Result<Box<Self>>;
    fn write(self: &Self, text: &str) -> Result<()>;
    fn clear(self: &Self) -> Result<()>;
}
