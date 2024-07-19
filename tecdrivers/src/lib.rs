use anyhow::Result;
use rusb::{Context, DeviceList};

pub mod liu;

pub trait USBDevice {
    fn find(list: &DeviceList<Context>, reset: bool) -> Result<Option<Box<Self>>>;
}
