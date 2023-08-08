use chuckle_util::{db::get_settings, ChuckleState};
use zephyrus::prelude::*;

use super::{handle_generic_error, text_response};

pub mod breakout_category;
pub mod default_org;
pub mod default_repo;
pub mod forum_log;

#[tracing::instrument(skip(ctx))]
#[command("do")]
#[description = "List the configuration."]
#[only_guilds]
#[error_handler(handle_generic_error)]
pub async fn list(ctx: &SlashContext<ChuckleState>) -> DefaultCommandResult {
	let settings = get_settings(ctx.data, ctx.interaction.guild_id.unwrap()).await?;
	let guild = ctx
		.http_client()
		.guild(ctx.interaction.guild_id.unwrap())
		.await?
		.model()
		.await?;

	let content = format!(
		r#"
**Configuration for {}**

Breakout category: <#{}>
Forum log channel: <#{}>
Default organization: `{}`
Default repository: `{}`
"#,
		guild.name,
		settings
			.breakout_rooms_category_id
			.unwrap_or("None".to_string()),
		settings.forum_log_channel_id.unwrap_or("None".to_string()),
		settings
			.default_repository_owner
			.unwrap_or("None".to_string()),
		settings.default_repository.unwrap_or("None".to_string()),
	);

	text_response(ctx, content, false).await
}
