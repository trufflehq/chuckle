use chuckle_util::{db::get_settings, ChuckleState};
use std::str::FromStr;
use twilight_model::{
	channel::ChannelType,
	gateway::payload::incoming::ThreadCreate,
	id::{marker::ChannelMarker, Id},
};

pub async fn handle(state: ChuckleState, event: Box<ThreadCreate>) -> anyhow::Result<()> {
	if event.parent_id.is_none() && event.guild_id.is_none() {
		return Ok(()); // not a thread
	}

	if !event.newly_created.unwrap_or(false) {
		return Ok(()); // not a new thread
	}

	let parent = state.http_client.channel(event.parent_id.unwrap()).await;
	let parent = match parent {
		Ok(parent) => parent.model().await?,
		Err(_) => return Ok(()), // parent channel not found
	};

	if parent.kind != ChannelType::GuildForum {
		return Ok(()); // non-forum
	}

	let settings = get_settings(&state, event.guild_id.unwrap()).await?;
	if settings.forum_log_channel_id.is_none() {
		return Ok(()); // no forum log channel set
	}
	let log_id: Id<ChannelMarker> =
		Id::<ChannelMarker>::from_str(&settings.forum_log_channel_id.unwrap()).unwrap();

	let content = format!(
		"<@{}> created <#{}> in <#{}>",
		event.owner_id.unwrap(),
		event.id,
		parent.id
	);
	let _ = state
		.http_client
		.create_message(log_id)
		.content(&content)?
		.await;

	Ok(())
}
