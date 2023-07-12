use chuckle_util::ChuckleState;
use ms::*;
use std::str::FromStr;
use time::{Duration, OffsetDateTime};
use twilight_model::{
	application::interaction::{modal::ModalInteractionData, Interaction},
	channel::message::MessageFlags,
	http::interaction::{InteractionResponse, InteractionResponseType},
	id::{
		marker::{MessageMarker, UserMarker},
		Id,
	},
	user::User,
};
use twilight_util::builder::InteractionResponseDataBuilder;
use uuid::Uuid;

fn user_from_interaction(interaction: &Interaction) -> User {
	if interaction.guild_id.is_some() {
		return interaction.member.clone().unwrap().user.unwrap();
	}

	interaction.user.clone().unwrap()
}

pub async fn handle(
	state: ChuckleState,
	interaction: Interaction,
	command: ModalInteractionData,
) -> anyhow::Result<()> {
	let time = command.components[0].components[0].value.clone().unwrap();

	let time = if let Some(time) = ms!(&time) {
		time
	} else {
		let _ = state
            .interactions_client()
            .create_response(
                interaction.id,
                &interaction.token,
                &InteractionResponse {
                    kind: InteractionResponseType::ChannelMessageWithSource,
                    data: Some(
                        InteractionResponseDataBuilder::new()
                            .content("The duration you provided was invalid. Please try again with something like [this](https://github.com/nesso99/ms-rust/blob/5a579c9f5b45851086ace2bfa506f541e49b3bbd/tests/main.rs#L6-L22)")
                            .build(),
                    ),
                },
            )
            .await;

		return Ok(());
	};

	let modal_row = sqlx::query!(
		"select meta from modal where id = $1",
		Uuid::from_str(&command.custom_id).unwrap()
	)
	.fetch_one(&state.db)
	.await?;

	let meta = modal_row.meta.expect("meta was null");

	let message_id = if let Some(id) = meta.get("message_id") {
		Id::<MessageMarker>::from_str(id.as_str().unwrap())?
	} else {
		Err(anyhow::anyhow!("message_id was null"))?
	};

	let author_id = if let Some(id) = meta.get("author_id") {
		Id::<UserMarker>::from_str(id.as_str().unwrap())?
	} else {
		Err(anyhow::anyhow!("author_id was null"))?
	};

	let notify_at = OffsetDateTime::now_utc() + Duration::milliseconds(time as i64);
	let _ = sqlx::query!(
        "insert into notifications (author_id, user_id, guild_id, channel_id, message_id, notify_at) values ($1, $2, $3, $4, $5, $6) returning id",
        author_id.get() as i64,
        user_from_interaction(&interaction).id.get() as i64,
        interaction.guild_id.unwrap().get() as i64,
        interaction.channel.unwrap().id.get() as i64,
        message_id.get() as i64,
        notify_at
    ).fetch_one(&state.db).await?;

	let _ = state
		.interactions_client()
		.create_response(
			interaction.id,
			&interaction.token,
			&InteractionResponse {
				kind: InteractionResponseType::ChannelMessageWithSource,
				data: Some(
					InteractionResponseDataBuilder::new()
						.flags(MessageFlags::EPHEMERAL)
						.content(
							format!("Sounds good! I'll remind you in {}.", ms!(time, true))
								.as_str(),
						)
						.build(),
				),
			},
		)
		.await;

	Ok(())
}
