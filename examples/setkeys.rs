use anyhow::{Context as _, Result};
use rusb::{Context, UsbContext as _};
use tecdrivers::{
    kb::{
        poskeyboard::{USBPOSKeyboard, POSKEYS},
        KeyFlags,
    },
    USBDevice as _,
};

const KEY_DELETE: u8 = 76;

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let kb =
        USBPOSKeyboard::find(&Context::new()?.devices()?, true)?.context("device not found")?;

    // Linuxぶっ殺しゾーン
    for pos in POSKEYS {
        kb.set_key(*pos, KEY_DELETE, KeyFlags::LEFT_CTRL | KeyFlags::LEFT_ALT)?;
    }

    Ok(())
}
