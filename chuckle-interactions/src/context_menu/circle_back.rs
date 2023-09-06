use chuckle_util::ChuckleState;
use ms::*;
use time::{Duration, OffsetDateTime};
use twilight_model::application::interaction::InteractionData;
use zephyrus::prelude::*;

use crate::commands::{create_followup, handle_generic_error, user_from_interaction};

#[derive(Modal, Debug)]
#[modal(title = "Circle Back")]
struct CircleBackModal {
	#[modal(
		label = "How long until you'd like to be notified?",
		placeholder = "20m, 1hr, 3hr, 12hr, 2d, etc.",
		min_length = 2,
		max_length = 10
	)]
	til_notify: String,
}

#[command(message, name = "Circle Back")]
#[description = "Circle back to this message in a given amount of time"]
#[only_guilds]
#[error_handler(handle_generic_error)]
pub async fn circle_back(ctx: &SlashContext<ChuckleState>) -> DefaultCommandResult {
	let data = match &ctx.interaction.data {
		Some(InteractionData::ApplicationCommand(data)) => data,
		_ => return Ok(()),
	};
	let (message_id, message) = data
		.clone()
		.resolved
		.unwrap()
		.messages
		.into_iter()
		.next()
		.unwrap();
	let author_id = message.author.id;

	let modal_waiter = ctx.create_modal::<CircleBackModal>().await?;
	let output = modal_waiter.await?;

	let time = if let Some(time) = ms!(&output.til_notify) {
		time
	} else {
		return create_followup(
			ctx,
			"The duration you provided was invalid. Please try again with something like [this](https://github.com/nesso99/ms-rust/blob/5a579c9f5b45851086ace2bfa506f541e49b3bbd/tests/main.rs#L6-L22)".to_string(),
			false,
		)
		.await;
	};

	let notify_at = OffsetDateTime::now_utc() + Duration::milliseconds(time as i64);
	let _ = sqlx::query!(
        "insert into notifications (author_id, user_id, guild_id, channel_id, message_id, notify_at) values ($1, $2, $3, $4, $5, $6) returning id",
        author_id.get() as i64,
        user_from_interaction(&ctx.interaction).id.get() as i64,
        ctx.interaction.guild_id.unwrap().get() as i64,
        ctx.interaction.channel.clone().unwrap().id.get() as i64,
        message_id.get() as i64,
        notify_at
    ).fetch_one(&ctx.data.db).await?;

	create_followup(
		ctx,
		format!("Sounds good! I'll remind you in {}.", ms!(time, true)),
		true,
	)
	.await
}
