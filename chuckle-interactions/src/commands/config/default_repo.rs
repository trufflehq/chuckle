use chuckle_util::{db::get_settings, ChuckleState};
use zephyrus::prelude::*;

use crate::commands::{handle_generic_error, text_response};

#[tracing::instrument(skip(ctx))]
#[command]
#[description = "List the default repository for PR Comments"]
#[only_guilds]
#[error_handler(handle_generic_error)]
pub async fn list(ctx: &SlashContext<ChuckleState>) -> DefaultCommandResult {
	let settings = get_settings(ctx.data, ctx.interaction.guild_id.unwrap()).await?;

	if settings.default_repository.is_none() {
		text_response(
			ctx,
			"There is no default GitHub repository.".to_string(),
			true,
		)
		.await
	} else {
		text_response(
			ctx,
			format!(
				"The current default GitHub repository is `{}`",
				settings.default_repository.unwrap()
			),
			true,
		)
		.await
	}
}

#[tracing::instrument(skip(ctx))]
#[command]
#[description = "Set the default repository for PR Comments"]
#[only_guilds]
#[required_permissions(MANAGE_GUILD)]
#[error_handler(handle_generic_error)]
pub async fn set(
	ctx: &SlashContext<ChuckleState>,
	#[description = "The default repository for PR Comments"] repository: String,
) -> DefaultCommandResult {
	let res = sqlx::query!(
		"update guild_settings set default_repository = $1 where guild_id = $2",
		repository.to_string(),
		ctx.interaction.guild_id.unwrap().to_string()
	)
	.execute(&ctx.data.db)
	.await;

	let (content, ephemeral) = match res {
		Ok(_) => (
			format!(
				"Successfully set the default GitHub repository to `{}`.",
				repository
			),
			false,
		),
		Err(_) => (
			"Failed to set the default GitHub repository.".to_string(),
			true,
		),
	};

	text_response(ctx, content, ephemeral).await
}

#[tracing::instrument(skip(ctx))]
#[command]
#[description = "Unset the default GitHub repository for PR Comments"]
#[only_guilds]
#[error_handler(handle_generic_error)]
pub async fn unset(ctx: &SlashContext<ChuckleState>) -> DefaultCommandResult {
	let res = sqlx::query!(
		"update guild_settings set default_repository = null where guild_id = $1",
		ctx.interaction.guild_id.unwrap().to_string()
	)
	.execute(&ctx.data.db)
	.await;

	let (content, ephemeral) = match res {
		Ok(_) => (
			"Successfully unset the default GitHub repository.".to_string(),
			false,
		),
		Err(_) => (
			"Failed to unset the default GitHub repository.".to_string(),
			true,
		),
	};

	text_response(ctx, content, ephemeral).await
}
