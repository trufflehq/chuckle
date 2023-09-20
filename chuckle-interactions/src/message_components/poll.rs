use std::{collections::HashMap, str::FromStr};

use super::text_response;
use crate::{
	custom_ids::{PollClearCustomId, PollOptionCustomId},
	message_components::{defer, update_response},
	ChuckleFramework,
};
use anyhow::Context;
use indoc::indoc;
use itertools::Itertools;
use twilight_model::{
	application::interaction::{message_component::MessageComponentInteractionData, Interaction},
	id::{
		marker::{ChannelMarker, MessageMarker},
		Id,
	},
};
use twilight_util::builder::embed::EmbedFieldBuilder;
use uuid::Uuid;

#[derive(Debug, sqlx::FromRow)]
struct PollOption {
	id: Uuid,
	poll_id: Uuid,
	option: i32,
	created_at: Option<time::OffsetDateTime>,
}

#[derive(Debug, sqlx::FromRow)]
struct PollOptionVote {
	id: Uuid,
	poll_id: Uuid,
	poll_option_id: Uuid,
	user_id: String,
	created_at: Option<time::OffsetDateTime>,
}

pub async fn poll_option(
	framework: ChuckleFramework,
	interaction: Interaction,
	_data: MessageComponentInteractionData,
	custom_data: PollOptionCustomId,
) -> anyhow::Result<()> {
	defer(&framework.interaction_client(), &interaction, true).await?;

	let poll = sqlx::query!("select * from poll where id = $1", custom_data.poll_id)
		.fetch_optional(&framework.data.db)
		.await?
		.context("poll not found")?;

	let author = interaction.author().unwrap();
	let user_votes = sqlx::query!(
		"select * from poll_option_vote where poll_id = $1 and user_id = $2",
		custom_data.poll_id,
		author.id.to_string(),
	)
	.fetch_all(&framework.data.db)
	.await?;

	// the user hit the vote limit and the amount it allows is gte 1 (require Clear Votes)
	if user_votes.len() >= poll.votes_per_user as usize && poll.votes_per_user != 1 {
		return update_response(
			&framework.interaction_client(),
			&interaction,
			indoc! {"
				You have already voted the maximum number of times.
				Please click `Clear Votes` to clear your votes.
			"}
			.to_string(),
		)
		.await;
	}

	tracing::info!("here 1");
	// change the vote to thi	s option
	if user_votes.len() == 1 && poll.votes_per_user == 1 {
		let vote = &user_votes[0];
		sqlx::query!(
			"update poll_option_vote set poll_option_id = $1 where id = $2",
			custom_data.id,
			vote.id,
		)
		.execute(&framework.data.db)
		.await?;
	} else {
		sqlx::query!(
			"insert into poll_option_vote (poll_id, poll_option_id, user_id) values ($1, $2, $3)",
			custom_data.poll_id,
			custom_data.id,
			author.id.to_string(),
		)
		.execute(&framework.data.db)
		.await?;
	}
	tracing::info!("here 1.2");

	let fr = framework.clone();
	tokio::spawn(async move {
		let _ = update_message(fr, poll.id).await;
	});

	update_response(
		&framework.interaction_client(),
		&interaction,
		"Your vote has been tallied!".to_string(),
	)
	.await
}

pub async fn poll_clear(
	framework: ChuckleFramework,
	interaction: Interaction,
	_data: MessageComponentInteractionData,
	custom_data: PollClearCustomId,
) -> anyhow::Result<()> {
	let author = interaction.author().unwrap();
	let user_votes = sqlx::query!(
		"select count(*) from poll_option_vote where poll_id = $1 and user_id = $2",
		custom_data.poll_id,
		author.id.to_string(),
	)
	.fetch_one(&framework.data.db)
	.await?;
	let count = user_votes.count.unwrap_or(0);

	// the user has no votes
	let res = if count == 0 {
		text_response(
			&framework.interaction_client(),
			&interaction,
			"You have no votes to clear!".to_string(),
			true,
		)
		.await
	} else {
		sqlx::query!(
			"delete from poll_option_vote where poll_id = $1 and user_id = $2",
			custom_data.poll_id,
			author.id.to_string(),
		)
		.execute(&framework.data.db)
		.await?;

		text_response(
			&framework.interaction_client(),
			&interaction,
			format!("Your `{}` votes have been cleared!", count),
			true,
		)
		.await
	};

	let fr = framework.clone();
	tokio::spawn(async move {
		let _ = update_message(fr, custom_data.poll_id).await;
	});

	res
}

async fn update_message(framework: ChuckleFramework, poll_id: Uuid) -> anyhow::Result<()> {
	let poll = sqlx::query!("select * from poll where id = $1", poll_id)
		.fetch_optional(&framework.data.db)
		.await?
		.context("poll not found")?;

	let channel_id = Id::<ChannelMarker>::from_str(&poll.channel_id)?;
	let message_id = Id::<MessageMarker>::from_str(&poll.message_id)?;
	let message = framework
		.http_client()
		.message(channel_id, message_id)
		.await?
		.model()
		.await?;

	let options = sqlx::query_as!(
		PollOption,
		"select * from poll_option where poll_id = $1",
		poll_id,
	)
	.fetch_all(&framework.data.db)
	.await?;

	let votes_query = sqlx::query_as!(
		PollOptionVote,
		"select * from poll_option_vote where poll_id = $1",
		poll_id,
	)
	.fetch_all(&framework.data.db)
	.await?;

	tracing::info!("here 2");
	tracing::info!("options: {:#?}", options);
	tracing::info!("votes: {:#?}", votes_query);

	let mut sorted_votes = HashMap::<Uuid, Vec<PollOptionVote>>::new();
	for vote in votes_query {
		let votes = sorted_votes
			.entry(vote.poll_option_id)
			.or_insert_with(|| vec![]);
		votes.push(vote);
	}
	let mut sorted_votes = sorted_votes.into_values().collect::<Vec<_>>();
	// let mut sorted_votes: Vec<Vec<PollOptionVote>> = vec![];
	// for (_id, group) in &votes_query.into_iter().group_by(|v| v.poll_option_id) {
	// 	// FIXME: bring back when i wanna use sort vote users by most votes
	// 	// // group the votes by `vote_user_id`
	// 	// let mut group_users: Vec<Vec<Vote>> = vec![];
	// 	// for (_, users) in &group.into_iter().group_by(|v| v.vote_user_id.clone()) {
	// 	// 	let mut users = users.collect::<Vec<_>>();
	// 	// 	users.sort_by(|a, b| a.vote_created_at.cmp(&b.vote_created_at));

	// 	// 	group_users.push(users);
	// 	// }
	// 	sorted_votes.push(group.collect::<Vec<_>>())
	// }
	// then sort each chunk by most votes
	sorted_votes.sort_by(|a, b| b.len().cmp(&a.len()));
	tracing::info!("sorted votes: {:#?}", sorted_votes);

	// the votes field is the only field in the embed
	// the content is formatted like this:
	// \`x.` (#total): @username (#votes), @username (#votes), @username (#votes)
	// where x is the option number, total is the total number of votes, and votes is the number of votes for that user
	// each user's votes are sorted by who voted the most then by who voted first
	// it will only show options that have votes
	// it will only show the first 10 users then it will say `and x more`
	let list_entries = sorted_votes
		.into_iter()
		.map(|entries| {
			let mut user_groups: Vec<Vec<PollOptionVote>> = vec![];
			let option_id = entries[0].poll_option_id;
			for (_id, group) in &entries.into_iter().group_by(|v| v.user_id.clone()) {
				let mut users = group.collect::<Vec<_>>();
				users.sort_by(|a, b| a.created_at.cmp(&b.created_at));

				user_groups.push(users);
			}
			let option = &options.iter().find(|o| o.id == option_id).unwrap();
			let total = user_groups.iter().map(|users| users.len()).sum::<usize>();

			format!(
				"`{}.` {} (`{}` vote{})",
				option.option,
				user_groups
					.iter()
					.take(10)
					.map(|users| { format!("<@{}> ({})", users[0].user_id, users.len()) })
					.join(", "),
				total,
				if total == 1 { "" } else { "s" },
			)
		})
		.collect::<Vec<String>>();

	let field = EmbedFieldBuilder::new("Results", list_entries.join("\n")).build();
	let embed = &mut message.embeds[0].clone();
	embed.fields = vec![field];
	tracing::info!("embed: {:#?}", embed);

	// edit the message
	let x = framework
		.http_client()
		.update_message(channel_id, message_id)
		.embeds(Some(&[embed.clone()]))?
		.await?;
	tracing::info!("edited message: {:#?}", x);

	Ok(())
}
