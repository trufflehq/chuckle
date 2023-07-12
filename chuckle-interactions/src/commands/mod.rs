use chuckle_util::ChuckleState;
use twilight_model::{
	application::interaction::Interaction,
	channel::message::MessageFlags,
	http::interaction::{InteractionResponse, InteractionResponseData, InteractionResponseType},
	user::User,
};
use zephyrus::{framework::DefaultError, prelude::*};

pub mod config;
mod hexil;
mod link_github;
mod ping;
mod pr_comments;

pub use {hexil::hexil, link_github::link_github, ping::ping, pr_comments::pr_comments};

pub fn user_from_interaction(interaction: &Interaction) -> User {
	if interaction.guild_id.is_some() {
		return interaction.member.clone().unwrap().user.unwrap();
	}

	interaction.user.clone().unwrap()
}

#[check]
/// Check if the command has been executed inside a guild.
pub async fn only_guilds(ctx: &SlashContext<ChuckleState>) -> Result<bool, DefaultError> {
	// Only pass the check if the command has been executed inside a guild
	Ok(ctx.interaction.guild_id.is_some())
}

#[error_handler]
async fn handle_generic_error(ctx: &SlashContext<ChuckleState>, err: DefaultError) {
	let _ = text_response(
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
