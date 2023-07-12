use chuckle_util::ChuckleState;

pub async fn run(state: &ChuckleState) -> anyhow::Result<()> {
	sqlx::query("delete from modal where created_at < now() - interval '5 minutes' returning *")
		.execute(&state.db)
		.await?;

	Ok(())
}
