use chuckle_util::{db::get_settings, ChuckleState};
use twilight_model::http::interaction::{
	InteractionResponse, InteractionResponseData, InteractionResponseType,
};
use twilight_model::{channel::message::AllowedMentions, id::Id};
use zephyrus::{prelude::*, twilight_exports::RoleMarker};

use super::handle_generic_error;

// create a function that will take an array of things that implement to_string or debug
// join them all on the provided separator
// and, for the last one, use the word and instead of the separator
// so, for example, if you have a list of 3 things, you'd get "a, b, and c"
// if you have a list of 2 things, you'd get "a and b"
// if you have a list of 1 thing, you'd get "a"
// if you have a list of 0 things, you'd get ""
fn join_with_and<T: std::fmt::Display>(items: &[T], separator: &str) -> String {
	match items.len() {
		0 => "".to_string(),
		1 => items[0].to_string(),
		2 => format!("{} and {}", items[0], items[1]),
		_ => format!(
			"{} and {}",
			items[..items.len() - 1]
				.iter()
				.map(|i| i.to_string())
				.collect::<Vec<_>>()
				.join(separator),
			items[items.len() - 1]
		),
	}
}

#[tracing::instrument(skip(ctx))]
#[command]
#[description = "Add all people from a provided role to this thread."]
#[only_guilds]
#[error_handler(handle_generic_error)]
pub async fn add_role(
	ctx: &SlashContext<ChuckleState>,
	#[description = "Which role to add from"] role: Id<RoleMarker>,
) -> DefaultCommandResult {
	let _settings = get_settings(ctx.data, ctx.interaction.guild_id.unwrap()).await?;

	let members = ctx
		.http_client()
		.guild_members(ctx.interaction.guild_id.unwrap())
		.limit(500)
		.unwrap()
		.await
		.unwrap()
		.model()
		.await
		.unwrap();
	let role_members = members
		.into_iter()
		.filter(|m| m.roles.contains(&role))
		.collect::<Vec<_>>();

	for member in &role_members {
		let res = ctx
			.http_client()
			.add_thread_member(ctx.interaction.channel.clone().unwrap().id, member.user.id)
			.await;
		if let Err(e) = res {
			tracing::warn!(?e, "error adding thread member");
		}
	}

	let content = format!(
		"Successfully added {} member{} to the thread: {}",
		role_members.len(),
		// plurality
		if role_members.len() == 1 { "" } else { "s" },
		join_with_and(
			&role_members
				.iter()
				.map(|m| format!("<@{}>", m.user.id))
				.collect::<Vec<_>>(),
			", "
		)
	);

	ctx.interaction_client
		.create_response(
			ctx.interaction.id,
			&ctx.interaction.token,
			&InteractionResponse {
				kind: InteractionResponseType::ChannelMessageWithSource,
				data: Some(InteractionResponseData {
					content: Some(content),
					flags: None,
					allowed_mentions: Some(AllowedMentions::default()),
					..Default::default()
				}),
			},
		)
		.await?;

	Ok(())
}
