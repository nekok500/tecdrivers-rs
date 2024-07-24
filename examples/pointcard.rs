use core::str;
use std::time::Duration;

use anyhow::{bail, Context as _, Result};
use clap::Parser;
use rusb::{request_type, Context, DeviceHandle, UsbContext};
use tecdrivers::{liu::liust700::LIUST700, liu::LIU as _, USBDevice};

#[derive(Parser)]
struct Args {
    #[arg(id = "loop", short, long)]
    _loop: bool,
}

const TIMEOUT: Duration = Duration::from_secs(15);

fn cmd(handle: &DeviceHandle<Context>, request: u8, value: u16, index: u16) -> Result<()> {
    let mut buf = [0u8; 1];
    let num = handle.read_control(
        request_type(
            rusb::Direction::In,
            rusb::RequestType::Vendor,
            rusb::Recipient::Interface,
        ),
        request,
        value,
        index,
        &mut buf,
        TIMEOUT,
    )?;
    assert_eq!(num, 1);
    assert_eq!(buf, [0]);

    Ok(())
}

fn main() -> Result<()> {
    tracing_subscriber::fmt::init();

    let args = Args::parse();
    let ctx = Context::new()?;
    let kb = ctx
        .open_device_with_vid_pid(0x08a6, 0x000c)
        .context("device not found")?;
    kb.reset()?;

    let liu = LIUST700::find(&ctx.devices()?, true)?.context("liu not found")?;
    cmd(&kb, 212, 0, 0)?;
    cmd(&kb, 211, 0, 0)?;
    cmd(&kb, 222, 0, 0)?;

    loop {
        let result = read_card(&kb);
        liu.clear()?;

        match result {
            Err(e) => {
                tracing::error!(?e);
                liu.write(&format!("error: \r\n{:?}", e))?;
                cmd(&kb, 212, 0, 0)?;
                cmd(&kb, 211, 0, 0)?;
                cmd(&kb, 222, 0, 0)?;
                continue;
            }
            Ok(Some(cardnum)) => {
                tracing::info!("ポイントカード番号: {}", &cardnum);
                liu.write(&format!("ポイントカード番号:\r\n{}", &cardnum))?;
            }
            _ => {}
        }

        if args._loop {
            cmd(&kb, 51, 0, 0)?;
        } else {
            break;
        }
    }

    Ok(())
}

fn read_card(handle: &DeviceHandle<Context>) -> Result<Option<String>> {
    tracing::debug!("wating...");

    let mut buf = vec![0; 256];
    let n = handle.read_bulk(0x81, &mut buf, Duration::from_secs(180))?;
    buf = buf[..n].to_vec();

    buf.iter()
        .enumerate()
        .for_each(|(i, &b)| tracing::debug!("{}: {:x?} {:?}", i, b, char::from_u32(b as u32)));

    if buf[0..=1] != [0x80, 0x62] {
        return Ok(None);
    }

    if buf[2..=3] == [0x73, 0xff] {
        bail!("failed swipe");
    }

    if buf[2..=6] != [0x6e, 0x30, 0x37, 0x32, 0x7f] {
        return Ok(None);
    }

    let cardnum = str::from_utf8(&buf[7..=22])?.to_string();

    Ok(Some(cardnum))
}
