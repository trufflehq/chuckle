use chuckle_util::ChuckleState;
use twilight_model::{
	application::interaction::{application_command::CommandData, Interaction},
	channel::message::{
		component::{ActionRow, TextInput, TextInputStyle},
		Component,
	},
	http::interaction::{InteractionResponse, InteractionResponseType},
};
use twilight_util::builder::InteractionResponseDataBuilder;

pub async fn handle(
	state: ChuckleState,
	interaction: Interaction,
	command: Box<CommandData>,
) -> anyhow::Result<()> {
	let (message_id, message) = command
		.resolved
		.unwrap()
		.messages
		.into_iter()
		.next()
		.unwrap();
	let meta = serde_json::json!({ "message_id": message_id.to_string(), "author_id": message.author.id.to_string() });

	let id = sqlx::query_scalar!(
		"insert into modal (command, meta) values ($1, $2) returning id",
		"circle_back",
		meta
	)
	.fetch_one(&state.db)
	.await?;

	let components = vec![Component::ActionRow(ActionRow {
		components: Vec::from([Component::TextInput(TextInput {
			custom_id: id.to_string(),
			label: "How long until you'd like to be notified?".to_string(),
			required: Some(true),
			style: TextInputStyle::Short,
			placeholder: Some("20m, 1hr, 3hr, 12hr, 2d, etc.".to_string()),
			max_length: Some(10),
			min_length: Some(1),
			value: None,
		})]),
	})];

	let _ = state
		.interactions_client()
		.create_response(
			interaction.id,
			&interaction.token,
			&InteractionResponse {
				kind: InteractionResponseType::Modal,
				data: Some(
					InteractionResponseDataBuilder::new()
						.title("Circle back to this message")
						.components(components)
						.custom_id(id.to_string())
						.build(),
				),
			},
		)
		.await;

	Ok(())
}
