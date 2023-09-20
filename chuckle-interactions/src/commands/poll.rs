use crate::custom_ids::{PollClearCustomId, PollOptionCustomId};
use chuckle_util::{db::get_settings, ChuckleState};
use indoc::formatdoc;
use once_cell::sync::Lazy;
use rand::prelude::SliceRandom;
use twilight_util::builder::embed::EmbedBuilder;
use uuid::Uuid;
use zephyrus::{
	prelude::*,
	twilight_exports::{
		CommandInteractionDataResolved, CommandOptionType, CommandOptionValue, InteractionResponse,
		InteractionResponseData, InteractionResponseType,
	},
};

use super::{handle_generic_error, update_response};
use twilight_model::channel::message::{
	component::{ActionRow, Button, ButtonStyle, Component, ComponentType},
	MessageFlags, ReactionType,
};

#[derive(Debug)]
struct PollOptions(pub i8);

#[async_trait]
impl Parse<ChuckleState> for PollOptions {
	async fn parse(
		http_client: &WrappedClient,
		data: &ChuckleState,
		value: Option<&CommandOptionValue>,
		resolved: Option<&mut CommandInteractionDataResolved>,
	) -> Result<Self, ParseError> {
		let p = i8::parse(http_client, data, value, resolved).await?;

		if p < 2 || p > 24 {
			return Err(ParseError::Parsing {
				argument_name: "options".to_string(),
				argument_type: "PollOptions".to_string(),
				error: "Polls must have between 2 and 25 options.".to_string(),
				required: true,
			});
		} else {
			Ok(PollOptions(p))
		}
	}

	fn kind() -> CommandOptionType {
		CommandOptionType::Integer
	}

	fn limits() -> Option<ArgumentLimits> {
		use twilight_model::application::command::CommandOptionValue;

		Some(ArgumentLimits {
			min: Some(CommandOptionValue::Integer(2)),
			max: Some(CommandOptionValue::Integer(24)),
		})
	}
}

static EMOJIS: Lazy<Vec<char>> = Lazy::new(|| {
	vec![
		'🍎', '🥑', '🍌', '🍒', '🍇', '🍏', '🥝', '🍋', '🥭', '🍈', '🍑', '🍐', '🍍', '🍓', '🍊',
		'🍅', '🍉', '🥥', '🌽', '🫑', '🍆', '🫒', '🥔', '🧅', '🧄', '🍠', '🌽',
	]
});

#[tracing::instrument(skip(ctx))]
#[command("poll")]
#[description = "Creates a simple poll with up to 25 options."]
#[only_guilds]
#[error_handler(handle_generic_error)]
pub async fn poll(
	ctx: &SlashContext<ChuckleState>,
	#[description = "The amount of options this poll can have (2-24)"] options: PollOptions,
	#[description = "The amount of votes each user can cast (max: options - 1) (default: 1)"] votes: Option<i8>,
) -> DefaultCommandResult {
	ctx.defer(true).await?;

	let options = options.0;
	let votes = votes.unwrap_or(1);
	let channel = ctx.interaction.channel.clone().unwrap();
	let author = ctx.interaction.author().unwrap();

	if votes < 1 || votes > options - 1 {
		return update_response(
			ctx,
			"Votes must be greater than `0` and at least one less than `options`.".to_string(),
		)
		.await;
	}

	let poll_id = Uuid::new_v4();

	// there can be a max of 5 buttons per row, 5 rows per message. 25 total options.
	// turn options into chunks of 5
	let options = (1..=options)
		.map(|o| (o, Uuid::new_v4()))
		.collect::<Vec<_>>();
	let chunks = options.chunks(5).collect::<Vec<_>>();
	let shuffled_emojis = EMOJIS
		.choose_multiple(&mut rand::thread_rng(), options.len())
		.cloned()
		.collect::<Vec<_>>();

	let (mut rows, options): (Vec<Component>, Vec<Vec<(Uuid, PollOptionCustomId)>>) = chunks
		.iter()
		.map(|options| {
			let (components, custom_ids) = options
				.iter()
				.map(|(option, id)| {
					let id_data = PollOptionCustomId {
						id: *id,
						poll_id,
						option: *option,
					};
					let custom_id = Uuid::new_v4();

					(
						Component::Button(Button {
							custom_id: Some(custom_id.to_string()),
							disabled: false,
							label: Some(option.to_string()),
							style: ButtonStyle::Secondary,
							emoji: Some(
								ReactionType::Unicode {
									name: shuffled_emojis[*option as usize - 1].to_string(),
								}
								.into(),
							),
							url: None,
						}),
						(custom_id, id_data),
					)
				})
				.unzip::<_, _, Vec<_>, Vec<_>>();

			(Component::ActionRow(ActionRow { components }), custom_ids)
		})
		.unzip::<_, _, Vec<_>, Vec<_>>();

	let delete_id = Uuid::new_v4();
	let delete_button = Component::Button(Button {
		custom_id: Some(delete_id.to_string()),
		disabled: false,
		label: Some("Clear Votes".to_string()),
		style: ButtonStyle::Danger,
		emoji: None,
		url: None,
	});
	// add the delete button to the last row.
	// if the last row already has 5 buttons, add a new row.
	let last_row = rows.last_mut().unwrap();
	if let Component::ActionRow(row) = last_row {
		if row.components.len() == 5 {
			rows.push(Component::ActionRow(ActionRow {
				components: vec![delete_button],
			}));
		} else {
			row.components.push(delete_button);
		}
	}

	let votes_word = if votes == 1 { "vote" } else { "votes" };
	tracing::info!("here!");
	let embed = EmbedBuilder::new()
		.title("Poll")
		.color(0xf75fa7)
		.description(formatdoc! {"
				@{} created a new poll!
				Vote for up to `{}` option{} by clicking the buttons below.
				### Information

				- If you've already voted, and you'd like to change your {votes_word}:
				  - votes = 1: click a different option
				  - votes > 1: click the trash can button to reset your votes
				- If you want to clear your {votes_word}, click the trash can button.
				- If you have multiple votes, you can vote for the same option multiple times.
			",
			author.name,
			votes,
			if votes == 1 { "" } else { "s" },
		})
		.build();

	let msg = ctx
		.http_client()
		.create_message(channel.id)
		.embeds(&[embed])?
		.components(&rows)?
		.await?
		.model()
		.await?;
	let guild_id = ctx.interaction.guild_id.unwrap();

	// create the poll itself
	sqlx::query!(
		"
		insert into poll (id, guild_id, channel_id, message_id, creator_id, votes_per_user)
		values ($1, $2, $3, $4, $5, $6) returning *;",
		poll_id,
		guild_id.to_string(),
		msg.channel_id.to_string(),
		msg.id.to_string(),
		author.id.to_string(),
		votes as i32,
	)
	.fetch_one(&ctx.data.db)
	.await?;

	let flat_options = options.iter().flatten().collect::<Vec<_>>();

	// create the poll options
	let ids = flat_options
		.iter()
		.map(|(_, data)| data.id)
		.collect::<Vec<_>>();
	let poll_ids = std::iter::repeat(poll_id)
		.take(ids.len())
		.collect::<Vec<_>>();
	let option_ints = flat_options
		.iter()
		.map(|(_, data)| data.option as i64)
		.collect::<Vec<_>>();
	sqlx::query!(
		"
			insert into poll_option (id, poll_id, option)
			select * from unnest($1::uuid[], $2::uuid[], $3::int8[])
		",
		&ids,
		&poll_ids,
		&option_ints,
	)
	.fetch_optional(&ctx.data.db)
	.await?;

	// create the custom ids
	let ids = flat_options.iter().map(|d| d.0).collect::<Vec<_>>();
	let kind = std::iter::repeat("poll_option".to_string())
		.take(ids.len())
		.collect::<Vec<_>>();
	let data = flat_options
		.iter()
		.map(|(_, data)| serde_json::to_value(data).unwrap())
		.collect::<Vec<_>>();
	sqlx::query!(
		"insert into custom_id(id, kind, data)
		select * from unnest($1::uuid[], $2::text[], $3::jsonb[]);",
		&ids,
		&kind,
		&data,
	)
	.fetch_optional(&ctx.data.db)
	.await?;

	// create the delete button custom id
	let data = serde_json::to_value(&PollClearCustomId { poll_id }).unwrap();
	sqlx::query!(
		"insert into custom_id(id, kind, data) values ($1::uuid, $2, $3);",
		delete_id,
		"poll_clear",
		data,
	)
	.fetch_optional(&ctx.data.db)
	.await?;

	Ok(())
}
