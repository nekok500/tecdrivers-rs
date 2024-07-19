use anyhow::Result;
use rusb::{Context, DeviceHandle};

pub mod liust700;

pub trait LIU {
    /// USBデバイスを初期化する。
    fn init(handle: DeviceHandle<Context>, reset: bool) -> Result<Box<Self>>;
    /// カスタマーディスプレイに文字列を書き込む。
    fn write(self: &Self, text: &str) -> Result<()>;
    /// カスタマーディスプレイの表示をすべてクリアする。
    fn clear(self: &Self) -> Result<()>;
}
