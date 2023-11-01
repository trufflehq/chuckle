use chuckle_util::{db::get_settings, ChuckleState};
use twilight_model::id::{marker::ChannelMarker, Id};
use vesper::prelude::*;

use crate::commands::{handle_generic_error, text_response};

#[tracing::instrument(skip(ctx))]
#[command]
#[description = "List the current Breakout Rooms category"]
#[only_guilds]
#[error_handler(handle_generic_error)]
pub async fn list(ctx: &SlashContext<ChuckleState>) -> DefaultCommandResult {
	let settings = get_settings(ctx.data, ctx.interaction.guild_id.unwrap()).await?;

	if settings.breakout_rooms_category_id.is_none() {
		text_response(
			ctx,
			"There is no current Breakout Rooms category.".to_string(),
			true,
		)
		.await
	} else {
		text_response(
			ctx,
			format!(
				"The current default current Breakout Rooms category is <#{}>.",
				settings.breakout_rooms_category_id.unwrap()
			),
			true,
		)
		.await
	}
}

#[tracing::instrument(skip(ctx))]
#[command]
#[description = "Set the Breakout Rooms category"]
#[only_guilds]
#[required_permissions(MANAGE_GUILD)]
#[error_handler(handle_generic_error)]
pub async fn set(
	ctx: &SlashContext<ChuckleState>,
	#[description = "The current Breakout Rooms category"] category: Id<ChannelMarker>,
) -> DefaultCommandResult {
	let res = sqlx::query!(
		"update guild_settings set breakout_rooms_category_id = $1 where guild_id = $2",
		category.to_string(),
		ctx.interaction.guild_id.unwrap().to_string()
	)
	.execute(&ctx.data.db)
	.await;

	let (content, ephemeral) = match res {
		Ok(_) => (
			format!("Successfully set the current Breakout Rooms category to <#{category}>.",),
			false,
		),
		Err(_) => (
			"Failed to set the current Breakout Rooms category.".to_string(),
			true,
		),
	};

	text_response(ctx, content, ephemeral).await
}

#[tracing::instrument(skip(ctx))]
#[command]
#[description = "Unset the current Breakout Rooms category"]
#[only_guilds]
#[error_handler(handle_generic_error)]
pub async fn unset(ctx: &SlashContext<ChuckleState>) -> DefaultCommandResult {
	let res = sqlx::query!(
		"update guild_settings set breakout_rooms_category_id = null where guild_id = $1",
		ctx.interaction.guild_id.unwrap().to_string()
	)
	.execute(&ctx.data.db)
	.await;

	let (content, ephemeral) = match res {
		Ok(_) => (
			"Successfully unset the current Breakout Rooms category.".to_string(),
			false,
		),
		Err(_) => (
			"Failed to unset the current Breakout Rooms category.".to_string(),
			true,
		),
	};

	text_response(ctx, content, ephemeral).await
}
