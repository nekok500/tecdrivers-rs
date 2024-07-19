use anyhow::{Context as _, Result};
use clap::Parser;
use rusb::{Context, UsbContext};
use tecdrivers::{
    liu::{liust700::LIUST700, LIU},
    USBDevice,
};
use twilight_cache_inmemory::InMemoryCache;
use twilight_gateway::{Event, Intents, Shard, ShardId};
use twilight_model::{
    gateway::{
        payload::outgoing::request_guild_members::RequestGuildMembersBuilder, presence::Status,
    },
    id::Id,
};

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
    #[arg(long, env)]
    guild_id: u64,
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::fmt().init();
    let args = Args::parse();

    let ctx = Context::new()?;

    let liu = LIUST700::find(&ctx.devices()?, true)
        .context("failed to get device")?
        .context("device not found")?;

    // init end
    let intents = Intents::GUILD_PRESENCES | Intents::GUILD_MEMBERS;
    let mut shard = Shard::new(ShardId::ONE, args.token, intents);
    let cache = InMemoryCache::new();
    tracing::info!("created shard");

    loop {
        let item = shard.next_event().await;
        match item {
            Ok(event) => {
                cache.update(&event);

                match event {
                    Event::Ready(_) => {
                        shard
                            .command(
                                &RequestGuildMembersBuilder::new(Id::new(args.guild_id))
                                    .user_id(Id::new(args.user_id)),
                            )
                            .await?;
                    }
                    Event::PresenceUpdate(i) => {
                        if i.user.id().get() == args.user_id {
                            liu.clear()?;

                            let user_name = if let Some(user) = cache.user(i.user.id()) {
                                user.global_name.clone().unwrap_or(user.name.clone())
                            } else {
                                "".to_string()
                            };

                            liu.write(&format!(
                                "{}: {}\r\n\r\n",
                                user_name,
                                match i.status {
                                    Status::Online => "オンライン",
                                    Status::DoNotDisturb => "取り込み中",
                                    Status::Idle => "離席中",
                                    _ => "オフライン",
                                }
                            ))?;

                            if let Some(activity) = i.activities.get(0) {
                                if let (Some(state), Some(details)) =
                                    (activity.state.clone(), activity.details.clone())
                                {
                                    liu.write(&format!("{}\r\n", details))?;
                                    liu.write(&format!("    {}", state))?;
                                }
                            }

                            tracing::info!("{}", i.user.id())
                        }
                    }
                    _ => {}
                }
            }

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
