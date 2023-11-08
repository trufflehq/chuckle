use std::str::FromStr;

use chuckle_util::{
	chunkify::{chunkify, RemainderStrategy},
	db::get_settings,
	ChuckleState,
};
use rand::seq::SliceRandom;
use twilight_model::{channel::Channel, id::Id};
use vesper::{prelude::*, twilight_exports::ChannelMarker};

use super::{edit_response, handle_generic_error, text_response};

#[tracing::instrument(skip(ctx))]
#[command]
#[description = "Separate the people in a voice channel into breakout rooms."]
#[only_guilds]
#[error_handler(handle_generic_error)]
pub async fn create(
	ctx: &SlashContext<ChuckleState>,
	#[description = "Which voice channel to select people from"] channel: Id<ChannelMarker>,
	#[description = "How many people per room"] size: u8,
	#[description = "What to do with people who don't fit into a room"]
	remainder_strategy: RemainderStrategy,
) -> DefaultCommandResult {
	let settings = get_settings(ctx.data, ctx.interaction.guild_id.unwrap()).await?;
	if settings.breakout_rooms_category_id.is_none() {
		return text_response(ctx, "No breakout rooms category set.".to_string(), true).await;
	}

	let mut voice_states = ctx
		.data
		.cache
		.voice_channel_states(channel)
		.map_or(Vec::new(), |states| {
			states.into_iter().map(|s| s.user_id()).collect()
		});
	if voice_states.len() < size.into() {
		return text_response(
			ctx,
			format!(
				"There are fewer people in <#{}> than the size of your breakout rooms.",
				channel
			),
			true,
		)
		.await;
	}
	voice_states.shuffle(&mut rand::thread_rng());

	let rooms = chunkify(voice_states, size.into(), remainder_strategy).chunks;

	// for every room, create a new voice channel under the breakout rooms category
	// then, move the people in that room into the new voice channel
	for (i, members) in rooms.iter().enumerate() {
		let room_name = format!("Breakout Room {i}");

		let room = ctx
			.http_client()
			.create_guild_channel(ctx.interaction.guild_id.unwrap(), &room_name)?
			.kind(twilight_model::channel::ChannelType::GuildVoice)
			.parent_id(
				Id::<ChannelMarker>::from_str(
					&settings.breakout_rooms_category_id.clone().unwrap(),
				)
				.unwrap(),
			)
			.await?
			.model()
			.await?;

		for member in members {
			ctx.http_client()
				.update_guild_member(ctx.interaction.guild_id.unwrap(), *member)
				.channel_id(Some(room.id))
				.await?;
		}
	}

	text_response(ctx, "Pong!".to_string(), true).await
}

#[tracing::instrument(skip(ctx))]
#[command]
#[description = "Close breakout rooms and bring everyone back."]
#[only_guilds]
#[error_handler(handle_generic_error)]
pub async fn destroy(
	ctx: &SlashContext<ChuckleState>,
	#[description = "Where to bring everyone back to"] channel: Id<ChannelMarker>,
) -> DefaultCommandResult {
	let settings = get_settings(ctx.data, ctx.interaction.guild_id.unwrap()).await?;
	if settings.breakout_rooms_category_id.is_none() {
		return text_response(ctx, "No breakout rooms category set.".to_string(), true).await;
	}
	ctx.defer(false).await?;
	let breakout_rooms_category_id = settings.breakout_rooms_category_id.unwrap();

	let voice_states = ctx
		.data
		.cache
		.guild_voice_states(ctx.interaction.guild_id.unwrap())
		.map_or(Vec::new(), |states| {
			states
				.value()
				.iter()
				.map(|s| {
					ctx.data
						.cache
						.voice_state(*s, ctx.interaction.guild_id.unwrap())
						.unwrap()
				})
				.collect()
		});
	tracing::debug!("found {} voice states", voice_states.len());
	tracing::debug!("{:#?}", voice_states);

	let mut voice_channels: Vec<Channel> = Vec::new();
	for state in voice_states {
		tracing::debug!("checking state {}", state.session_id());
		let channel_id = state.channel_id();
		let voice_channel = ctx.data.cache.channel(channel_id).unwrap();
		if voice_channel.parent_id.is_none() {
			tracing::debug!("channel {:?} has no parent", voice_channel.name);
			continue;
		}

		if voice_channel.parent_id.unwrap().to_string() != breakout_rooms_category_id {
			tracing::debug!(
				"channel {:?} is not in breakout rooms category",
				voice_channel.name
			);
			continue;
		}
		tracing::debug!("found channel: {:?}", voice_channel.name);

		if !voice_channels.contains(&voice_channel) {
			voice_channels.push(voice_channel.to_owned());
		}

		// move the member back to home
		tracing::info!("moving user: {:?}", state.user_id());
		let _ = tokio::spawn(async move {
			let res = ctx
			.http_client()
			.update_guild_member(ctx.interaction.guild_id.unwrap(), state.user_id())
			.channel_id(Some(channel))
			.await;
		});

		tracing::info!("successfully moved user: {:?}", res);
	}

	// delete the voice channels
	for channel in voice_channels {
		ctx.http_client().delete_channel(channel.id).await?;
	}

	edit_response(ctx, "Pong!".to_string()).await
}
