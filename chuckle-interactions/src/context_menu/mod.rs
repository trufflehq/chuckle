use chuckle_util::ChuckleState;
use twilight_model::application::interaction::{application_command::CommandData, Interaction};

mod circle_back;

pub async fn handle(
	state: ChuckleState,
	interaction: Interaction,
	data: Box<CommandData>,
) -> anyhow::Result<()> {
	let raw = serde_json::to_string_pretty(&data).unwrap();
	tracing::debug!("received context menu command: {}", raw);

	match data.name.as_str() {
		"Circle back" => circle_back::handle(state, interaction, data).await,
		_ => Ok(()),
	}
}
