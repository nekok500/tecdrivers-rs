use anyhow::{Context as _l, Result};
use clap::Parser;
use rusb::{Context, UsbContext};
use tecdrivers::{liu::liu7000::LIU7000, liu::LIU, USBDevice as _};

#[derive(Parser)]
struct Args {
    #[arg()]
    text: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let liu = LIU7000::find(&Context::new()?.devices()?, true)?.context("device not found")?;

    liu.write(&args.text)?;

    Ok(())
}
