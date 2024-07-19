//! # tecdrivers-rs
//!
//! `tecdrivers` は東芝テック社のPOS周辺機器をクロスプラットフォームでRustから操作できるようにするクレートです。
//!
//! 本クレートはホビー用途での利用のみ想定されています。

use anyhow::Result;
use rusb::{Context, DeviceList};

pub mod liu;

pub trait USBDevice {
    /// USBデバイスを検索し操作可能なデバイスを返す。
    fn find(list: &DeviceList<Context>, reset: bool) -> Result<Option<Box<Self>>>;
}
