use chuckle_interactions::context_menu;
use chuckle_interactions::modals;
use chuckle_interactions::ChuckleFramework;
use chuckle_util::ChuckleState;
use twilight_model::{
	application::{
		command::CommandType,
		interaction::{InteractionData, InteractionType},
	},
	gateway::payload::incoming::InteractionCreate,
};

pub async fn handle(
	state: ChuckleState,
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
				match data.kind {
					CommandType::ChatInput => {
						tracing::info!("Received a command: {:?}", data.name);
						framework.process(interaction).await;

						Ok(())
					}
					CommandType::Message => context_menu::handle(state, interaction, data).await,
					_ => Ok(()),
				}
			}
			_ => Ok(()),
		},
		InteractionType::ModalSubmit => {
			let data = match interaction.data.clone() {
				Some(InteractionData::ModalSubmit(data)) => Some(data),
				_ => None,
			}
			.expect("`InteractionType::ModalSubmit` has data");

			modals::handle(state, interaction, data).await
		}
		InteractionType::MessageComponent => unimplemented!(),
		InteractionType::ApplicationCommandAutocomplete => unimplemented!(""),
		_ => unimplemented!(),
	}
}
