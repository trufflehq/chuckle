use chuckle_interactions::ChuckleFramework;
use twilight_model::{
	application::interaction::{InteractionData, InteractionType},
	gateway::payload::incoming::InteractionCreate,
};

pub async fn handle(
	framework: ChuckleFramework,
	event: Box<InteractionCreate>,
) -> anyhow::Result<()> {
	if !event.is_guild() {
		return Ok(()); // dms
	}
	let interaction = event.0;
	tracing::info!("Received an interaction {:#?}", interaction.kind);

	match interaction.kind {
		InteractionType::Ping => unimplemented!("should be unnecessary via gateway"),
		InteractionType::ApplicationCommand => match interaction.clone().data {
			Some(InteractionData::ApplicationCommand(data)) => {
				tracing::info!("received application command: {:?}", data.kind);
				framework.process(interaction).await;

				Ok(())
			}
			_ => Ok(()),
		},
		InteractionType::ModalSubmit => {
			framework.process(interaction).await;

			Ok(())
		}
		InteractionType::MessageComponent => {
			chuckle_interactions::message_components::handle(framework, interaction).await?;

			Ok(())
		}
		InteractionType::ApplicationCommandAutocomplete => unimplemented!(""),
		_ => unimplemented!(),
	}
}
