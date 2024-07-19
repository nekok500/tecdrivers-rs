use anyhow::{Context as _, Result};
use clap::Parser;
use rusb::{Context, UsbContext};
use tecdrivers::{
    liu::{liu7000::LIU7000, LIU},
    USBDevice,
};
use twilight_gateway::{Event, Intents, Shard, ShardId};

#[derive(Parser, Debug)]
#[command(
    version,
    about = "人のステータスをカスタマーディスプレイに表示します。"
)]
struct Args {
    #[arg(long, env, hide_env_values = true)]
    token: String,
    #[arg(long, env)]
    user_id: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    let args = Args::parse();

    let ctx = Context::new()?;

    let liu = LIU7000::find(&ctx.devices()?, true)
        .context("failed to get device")?
        .context("device not found")?;

    // init end
    let intents = Intents::GUILD_PRESENCES;
    let mut shard = Shard::new(ShardId::ONE, args.token, intents);
    tracing::info!("created shard");

    loop {
        let item = shard.next_event().await;
        match item {
            Ok(Event::PresenceUpdate(i)) => {
                if i.user.id().get() == args.user_id {
                    liu.clear()?;

                    if let Some(activity) = i.activities.get(0) {
                        if let (Some(state), Some(details)) =
                            (activity.state.clone(), activity.details.clone())
                        {
                            liu.write(&format!("{}\r\n", details))?;
                            liu.write(&format!("    {}\r\n", state))?;
                        }
                    }
                    tracing::info!("{}", i.user.id())
                }
            }
            Ok(_) => {}
            Err(err) => {
                if err.is_fatal() {
                    tracing::error!(?err, "fatal error");
                    return Ok(());
                }
                tracing::warn!(?err, "error receiving event")
            }
        }
    }
}
