pub mod events;

use crate::events::{interaction_create, thread_create};
use anyhow::Result;
use chuckle_interactions::ChuckleFramework;
use chuckle_util::{ChuckleState, CONFIG};
use twilight_gateway::{Config, Event, EventTypeFlags, Intents, Shard, ShardId};

const BOT_EVENTS: EventTypeFlags = EventTypeFlags::from_bits_truncate(
	EventTypeFlags::READY.bits()
		| EventTypeFlags::GUILD_CREATE.bits()
		| EventTypeFlags::GUILD_DELETE.bits()
		| EventTypeFlags::THREAD_CREATE.bits()
		| EventTypeFlags::CHANNEL_CREATE.bits()
		| EventTypeFlags::CHANNEL_UPDATE.bits()
		| EventTypeFlags::CHANNEL_DELETE.bits()
		| EventTypeFlags::INTERACTION_CREATE.bits()
		| EventTypeFlags::GUILD_VOICE_STATES.bits()
		| EventTypeFlags::GUILD_MEMBERS.bits(),
);

pub async fn create_gateway(state: ChuckleState, framework: ChuckleFramework) -> Result<()> {
	let config = Config::builder(
		CONFIG.discord_token.clone(),
		Intents::GUILDS | Intents::GUILD_VOICE_STATES | Intents::GUILD_MEMBERS,
	)
	.event_types(BOT_EVENTS)
	.build();
	let mut shard = Shard::with_config(ShardId::ONE, config);

	loop {
		let event = match shard.next_event().await {
			Ok(event) => event,
			Err(source) => {
				tracing::warn!(?source, "error recieving event");
				continue;
			}
		};

		let state = state.clone();
		let framework = framework.clone();
		tokio::spawn(handle_event(state, framework, event));
	}
}

#[allow(clippy::unit_arg)]
pub async fn handle_event(
	state: ChuckleState,
	framework: ChuckleFramework,
	event: Event,
) -> Result<()> {
	let shard_id = 1;
	state.cache.update(&event);

	match event {
		Event::GatewayHeartbeat(heart) => {
			Ok(tracing::debug!("Shard {shard_id} heartbeat: {heart}"))
		}
		Event::ThreadCreate(event) => thread_create::handle(state, event).await,
		Event::InteractionCreate(event) => interaction_create::handle(framework, event).await,
		Event::Ready(_) => Ok(tracing::info!("Shard {shard_id} connected; client ready!")),
		Event::GatewayReconnect => Ok(tracing::info! {
			target: "gateway_reconnect",
			"shard {shard_id} gateway reconnecting"
		}),
		Event::GuildCreate(guild) => Ok(tracing::info!(
			"guild_create: received {} ({})",
			guild.name,
			guild.id,
		)),
		Event::GuildDelete(guild) => Ok(tracing::info!(
			"[event::guilddelete] shard {shard_id} guild delete {}",
			guild.id
		)),
		_ => Ok(tracing::debug!(
			"shard {shard_id} emitted {:?}",
			event.kind()
		)),
	}
}
