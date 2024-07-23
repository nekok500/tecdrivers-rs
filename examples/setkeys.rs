use std::{fs, path::PathBuf};

use anyhow::{Context as _, Error, Result};
use clap::Parser;
use rusb::{Context, UsbContext as _};
use tecdrivers::{
    kb::{
        poskeyboard::{USBPOSKeyboard, POSKEYS},
        KeyFlags,
    },
    USBDevice as _,
};

#[derive(Parser)]
struct Args {
    #[clap(short, long, default_value = "./setkb.700s")]
    file: PathBuf,
    #[clap(short, long, help = "構成ファイルに書かれてないキーを無効化します。")]
    missing_disable: bool,
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();

    let kb =
        USBPOSKeyboard::find(&Context::new()?.devices()?, true)?.context("device not found")?;

    let mut replaced: Vec<u8> = Vec::new();
    fs::read_to_string(args.file)?
        .split('\n')
        .filter(|line| !(line.trim().is_empty() || line.starts_with('#')))
        .try_for_each(|line| {
            let mut iter = line.split(',').map(|s| s.trim());
            let pos = iter.next().context("missing position")?.parse()?;
            let bits = iter.next().context("missing flags")?.parse::<u8>()?;
            let key = iter.next().context("missing key")?.parse()?;
            let flags = KeyFlags::from_bits_retain(bits);

            kb.set_key(pos, key, &flags)?;
            tracing::info!(
                "set key: pos={}, key={}, flags={:b}",
                pos,
                key,
                flags.bits()
            );

            replaced.push(pos);

            Ok::<(), Error>(())
        })?;

    if args.missing_disable {
        POSKEYS.iter().try_for_each(|pos| {
            if !replaced.contains(pos) {
                kb.set_key(*pos, 0, &KeyFlags::empty())?;
                tracing::info!("disable key: pos={}", pos);
            }

            Ok::<(), Error>(())
        })?;
    }

    Ok(())
}
