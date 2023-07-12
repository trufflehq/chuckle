use crate::{ChuckleState, Timestamptz};
use twilight_model::id::{marker::GuildMarker, Id};
use uuid::Uuid;

#[derive(Debug)]
pub struct GuildSettingsRow {
	pub id: Uuid,
	pub guild_id: String,
	pub forum_log_channel_id: Option<String>,
	pub default_repository: Option<String>,
	pub default_repository_owner: Option<String>,
	pub created_at: Timestamptz,
}

/// Fetches the guild settings, and creates a new entry if it doesn't exist.
pub async fn get_settings(
	state: &ChuckleState,
	guild_id: Id<GuildMarker>,
) -> anyhow::Result<GuildSettingsRow> {
	let entry = sqlx::query_as!(
		GuildSettingsRow,
		"select * from guild_settings where guild_id = $1",
		guild_id.to_string()
	)
	.fetch_optional(&state.db)
	.await?;

	if entry.is_none() {
		Ok(sqlx::query_as!(
			GuildSettingsRow,
			"insert into guild_settings (guild_id) values ($1) returning *",
			guild_id.to_string()
		)
		.fetch_one(&state.db)
		.await?)
	} else {
		Ok(entry.unwrap())
	}
}
