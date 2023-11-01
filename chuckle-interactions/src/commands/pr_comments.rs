use chuckle_util::{db::get_settings, ChuckleState};
use vesper::prelude::*;

use super::{handle_generic_error, text_response};

#[tracing::instrument(skip(ctx))]
#[command("pr-comments")]
#[description = "Get the comments for a PR."]
#[only_guilds]
#[error_handler(handle_generic_error)]
pub async fn pr_comments(
	ctx: &SlashContext<ChuckleState>,
	#[description = "The PR number to register for."] pr: i16,
	#[description = "The owner of the repo."] owner: Option<String>,
	#[description = "The repo name."] repo: Option<String>,
) -> DefaultCommandResult {
	let pr = pr as i32;
	let settings = get_settings(ctx.data, ctx.interaction.guild_id.unwrap()).await?;

	let owner = owner.unwrap_or(settings.default_repository_owner.unwrap_or("".to_string()));
	let repo = repo.unwrap_or(settings.default_repository.unwrap_or("".to_string()));
	tracing::info!("Received pr-comments command: {}/{}/{}", owner, repo, pr);

	if owner.is_empty() || repo.is_empty() {
		return text_response(
			ctx,
			"Please set a default repository and owner with `/config default-repo set` and `/config default-org`.".to_string(),
			true,
		)
		.await;
	}

	let thread_id = ctx.interaction.channel.clone().unwrap().id.to_string();

	let existing_entry = sqlx::query!(
		"SELECT * FROM pr_review_output WHERE pr_number = $1 AND repo_owner = $2 AND repo = $3",
		pr,
		owner,
		repo
	)
	.fetch_optional(&ctx.data.db)
	.await?;

	if existing_entry.is_some() {
		let revised_entry = sqlx::query!(
			"update pr_review_output set thread_id = $1 where id = $2 returning id",
			thread_id,
			existing_entry.unwrap().id,
		)
		.fetch_one(&ctx.data.db)
		.await?;

		return text_response(
			ctx,
			format!(
				"Revised the review output for `{}/{}#{}` to post in <#{}> (`{}`).",
				owner, repo, pr, thread_id, revised_entry.id
			),
			false,
		)
		.await;
	}

	let entry = sqlx::query!(
        "insert into pr_review_output (pr_number, repo_owner, repo, thread_id) values ($1, $2, $3, $4) returning id",
        pr,
        owner,
        repo,
        thread_id,
    ).fetch_one(&ctx.data.db).await?;

	text_response(
		ctx,
		format!(
			"Review output for `{}/{}#{}` to post in <#{}> (`{}`).",
			owner, repo, pr, thread_id, entry.id
		),
		false,
	)
	.await
}
