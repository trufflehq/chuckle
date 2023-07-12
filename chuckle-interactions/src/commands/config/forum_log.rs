use chuckle_util::{db::get_settings, ChuckleState};
use twilight_model::id::{marker::ChannelMarker, Id};
use zephyrus::prelude::*;

use crate::commands::{handle_generic_error, only_guilds, text_response};

#[command]
#[description = "List the forum log channel."]
#[checks(only_guilds)]
#[error_handler(handle_generic_error)]
pub async fn list(ctx: &SlashContext<ChuckleState>) -> DefaultCommandResult {
	let settings = get_settings(ctx.data, ctx.interaction.guild_id.unwrap()).await?;

	if settings.forum_log_channel_id.is_none() {
		text_response(ctx, "No forum log channel set.".to_string(), true).await
	} else {
		text_response(
			ctx,
			format!(
				"The current forum log channel is <#{}>",
				settings.forum_log_channel_id.unwrap()
			),
			true,
		)
		.await
	}
}

#[command]
#[description = "Set the forum log channel."]
#[checks(only_guilds)]
#[required_permissions(MANAGE_GUILD)]
#[error_handler(handle_generic_error)]
pub async fn set(
	ctx: &SlashContext<ChuckleState>,
	#[description = "`The channel to use as the forum log."] channel: Id<ChannelMarker>,
) -> DefaultCommandResult {
	let res = sqlx::query!(
		"update guild_settings set forum_log_channel_id = $1 where guild_id = $2",
		channel.to_string(),
		ctx.interaction.guild_id.unwrap().to_string()
	)
	.execute(&ctx.data.db)
	.await;

	let (content, ephemeral) = match res {
		Ok(_) => (
			format!("Successfully set the forum log channel to <#{}>", channel),
			false,
		),
		Err(_) => ("Failed to set the forum log channel.".to_string(), true),
	};

	text_response(ctx, content, ephemeral).await
}

#[command]
#[description = "Unset the forum log channel."]
#[checks(only_guilds)]
#[error_handler(handle_generic_error)]
pub async fn unset(ctx: &SlashContext<ChuckleState>) -> DefaultCommandResult {
	let res = sqlx::query!(
		"update guild_settings set forum_log_channel_id = null where guild_id = $1",
		ctx.interaction.guild_id.unwrap().to_string()
	)
	.execute(&ctx.data.db)
	.await;

	let (content, ephemeral) = match res {
		Ok(_) => (
			"Successfully unset the forum log channel.".to_string(),
			false,
		),
		Err(_) => ("Failed to unset the forum log channel.".to_string(), true),
	};

	text_response(ctx, content, ephemeral).await
}
