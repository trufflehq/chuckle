use chuckle_util::ChuckleState;
use twilight_model::{
	application::interaction::Interaction,
	channel::message::MessageFlags,
	http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
	user::User,
};
use zephyrus::{framework::DefaultError, prelude::*};

// groups
pub mod breakout_rooms;
pub mod config;
pub mod threads;

mod hexil;
mod link_github;
mod ping;
mod poll;
mod pr_comments;

pub use {
	hexil::hexil, link_github::link_github, ping::ping, poll::poll, pr_comments::pr_comments,
};

pub fn user_from_interaction(interaction: &Interaction) -> User {
	if interaction.guild_id.is_some() {
		return interaction.member.clone().unwrap().user.unwrap();
	}

	interaction.user.clone().unwrap()
}

#[error_handler]
async fn handle_generic_error(ctx: &SlashContext<ChuckleState>, err: DefaultError) {
	tracing::error!("An unknown error occurred: {:#?}", err);

	let _ = create_followup(
		ctx,
		format!(
			r#"
An unknown error occurred:
```rs
{:#?}
```
"#,
			err
		),
		true,
	)
	.await;
}

/// Shorthand for editing the response to an interaction after being deferred.
pub async fn edit_response(
	ctx: &SlashContext<'_, ChuckleState>,
	text: String,
) -> DefaultCommandResult {
	ctx.interaction_client
		.update_response(&ctx.interaction.token)
		.content(Some(&text))
		.unwrap()
		.await?;

	Ok(())
}

/// Shorthand to creating a text response to an interaction.
pub async fn text_response(
	ctx: &SlashContext<'_, ChuckleState>,
	text: String,
	ephemeral: bool,
) -> DefaultCommandResult {
	ctx.interaction_client
		.create_response(
			ctx.interaction.id,
			&ctx.interaction.token,
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

/// Shorthand to creating a text response to an interaction.
pub async fn create_followup(
	ctx: &SlashContext<'_, ChuckleState>,
	text: String,
	ephemeral: bool,
) -> DefaultCommandResult {
	let mut builder = ctx
		.interaction_client
		.create_followup(&ctx.interaction.token)
		.content(&text)?;

	if ephemeral {
		builder = builder.flags(MessageFlags::EPHEMERAL);
	}

	builder.await?;

	Ok(())
}

/// Shorthand for updating the response to an interaction after being deferred.
pub async fn update_response(
	ctx: &SlashContext<'_, ChuckleState>,
	text: String,
) -> DefaultCommandResult {
	let mut builder = ctx
		.interaction_client
		.update_response(&ctx.interaction.token);
	builder = builder.content(Some(&text)).unwrap();

	builder.await?;

	Ok(())
}
