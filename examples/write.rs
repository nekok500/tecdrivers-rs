use anyhow::{Context as _l, Result};
use clap::Parser;
use rusb::{Context, UsbContext};
use tecdrivers::{liu::liust700::LIUST700, liu::LIU, USBDevice as _};

#[derive(Parser)]
struct Args {
    #[arg()]
    text: String,
}

fn main() -> Result<()> {
    let args = Args::parse();

    let liu = LIUST700::find(&Context::new()?.devices()?, true)?.context("device not found")?;

    liu.write(&args.text)?;

    Ok(())
}
