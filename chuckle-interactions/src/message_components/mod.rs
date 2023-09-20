use crate::{
	custom_ids::{PollClearCustomId, PollOptionCustomId},
	ChuckleFramework,
};
use twilight_model::{application::interaction::Interaction, channel::message::MessageFlags};
use uuid::Uuid;
use zephyrus::twilight_exports::{
	InteractionClient, InteractionData, InteractionResponse, InteractionResponseData,
	InteractionResponseType,
};

mod poll;

pub async fn handle(framework: ChuckleFramework, interaction: Interaction) -> anyhow::Result<()> {
	let data = match &interaction.data {
		Some(InteractionData::MessageComponent(data)) => data,
		_ => unreachable!(),
	};

	let custom_id = match Uuid::try_parse(&data.custom_id) {
		Ok(id) => id,
		Err(_) => return Ok(()),
	};

	let custom_data = match sqlx::query!("select * from custom_id where id = $1", custom_id)
		.fetch_optional(&framework.data.db)
		.await?
	{
		Some(data) => data,
		None => return Ok(()),
	};

	match custom_data.kind.as_str() {
		"poll_option" => {
			let custom_data: PollOptionCustomId = serde_json::from_value(custom_data.data)?;

			poll::poll_option(framework, interaction.clone(), data.clone(), custom_data).await?;
		}
		"poll_clear" => {
			let custom_data: PollClearCustomId = serde_json::from_value(custom_data.data)?;

			poll::poll_clear(framework, interaction.clone(), data.clone(), custom_data).await?;
		}
		_ => return Ok(()),
	}

	Ok(())
}

pub async fn text_response(
	client: &InteractionClient<'_>,
	interaction: &Interaction,
	text: String,
	ephemeral: bool,
) -> anyhow::Result<()> {
	client
		.create_response(
			interaction.id,
			&interaction.token,
			&InteractionResponse {
				kind: InteractionResponseType::ChannelMessageWithSource,
				data: Some(InteractionResponseData {
					content: Some(text),
					flags: if ephemeral {
						Some(MessageFlags::EPHEMERAL)
					} else {
						None
					},
					..Default::default()
				}),
			},
		)
		.await?;

	Ok(())
}

pub async fn defer(
	client: &InteractionClient<'_>,
	interaction: &Interaction,
	ephemeral: bool,
) -> anyhow::Result<()> {
	client
		.create_response(
			interaction.id,
			&interaction.token,
			&InteractionResponse {
				kind: InteractionResponseType::DeferredChannelMessageWithSource,
				data: Some(InteractionResponseData {
					content: None,
					flags: if ephemeral {
						Some(MessageFlags::EPHEMERAL)
					} else {
						None
					},
					..Default::default()
				}),
			},
		)
		.await?;

	Ok(())
}

/// Shorthand for updating the response to an interaction after being deferred.
pub async fn update_response(
	client: &InteractionClient<'_>,
	interaction: &Interaction,
	text: String,
) -> anyhow::Result<()> {
	client
		.update_response(&interaction.token)
		.content(Some(&text))?
		.await?;

	Ok(())
}
