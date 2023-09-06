use chuckle_util::ChuckleState;
use twilight_model::id::{marker::UserMarker, Id};

pub async fn run(state: &ChuckleState) -> anyhow::Result<()> {
	let rows = sqlx::query!(
		"select id, user_id, author_id, guild_id, channel_id, message_id, notify_at from notifications where notify_at < now() and completed = false",
	)
	.fetch_all(&state.db).await?;

	for row in rows {
		// change `completed` to `true`
		sqlx::query!(
			"update notifications set completed = true where id = $1",
			row.id
		)
		.execute(&state.db)
		.await?;

		let dm_channel = state
			.http_client
			.create_private_channel(Id::<UserMarker>::new(row.user_id as u64))
			.await?
			.model()
			.await?;

		state
			.http_client
			.create_message(dm_channel.id)
			.content(
				format!(
					"Time to circle back to https://discord.com/channels/{}/{}/{} from <@{}>!",
					row.guild_id, row.channel_id, row.message_id, row.author_id
				)
				.as_str(),
			)?
			.await?;
	}

	Ok(())
}
