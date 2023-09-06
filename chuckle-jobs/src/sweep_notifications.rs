use chuckle_util::ChuckleState;

/// Sweep completed `notifications` rows older than 24 hours.
pub async fn run(state: &ChuckleState) -> anyhow::Result<()> {
	sqlx::query("delete from notifications where completed = true and notify_at < now() - interval '24 hours';")
		.execute(&state.db)
		.await?;

	Ok(())
}
