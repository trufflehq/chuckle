use chuckle_util::ChuckleState;
use std::str::FromStr;
use twilight_model::application::interaction::{modal::ModalInteractionData, Interaction};
use uuid::Uuid;

mod circle_back;

pub async fn handle(
	state: ChuckleState,
	interaction: Interaction,
	data: ModalInteractionData,
) -> anyhow::Result<()> {
	let row = sqlx::query!(
		"select * from modal where id = $1",
		Uuid::from_str(&data.custom_id).unwrap()
	)
	.fetch_optional(&state.db)
	.await?;

	if let Some(row) = row {
		match row.command.as_str() {
			"circle_back" => circle_back::handle(state, interaction, data).await,
			_ => Ok(()),
		}
	} else {
		Ok(())
	}
}
