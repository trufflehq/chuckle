use std::str::FromStr;

use crate::Result;
use axum::{
	extract::State,
	http::StatusCode,
	response::{IntoResponse, Response},
	routing::post,
	Router,
};
use chuckle_github::{fetch_raw_file, pull_request_review_comment::PullRequestReviewComment};
use chuckle_util::{ChuckleState, CONFIG};
use github_webhooks::common::GithubWebhook;
use once_cell::sync::Lazy;
use ring::hmac;
use twilight_model::id::{marker::ChannelMarker, Id};

pub fn router() -> Router<ChuckleState> {
	Router::new().route("/webhooks/github", post(handle_webhook))
}

static KEY: Lazy<hmac::Key> =
	Lazy::new(|| hmac::Key::new(hmac::HMAC_SHA256, CONFIG.github_webhook_secret.as_bytes()));

#[axum::debug_handler]
async fn handle_webhook(
	State(state): State<ChuckleState>,
	webhook: GithubWebhook,
) -> Result<Response> {
	let event = match webhook.to_event(&KEY) {
		Ok(e) => e,
		Err(_) => return Ok(StatusCode::BAD_REQUEST.into_response()),
	};

	tracing::info!("{:#?}", serde_json::to_string_pretty(&event).unwrap());

	#[allow(clippy::single_match)]
	match webhook.event_type.as_str() {
		"pull_request_review_comment" => {
			let data: PullRequestReviewComment = serde_json::from_value(event).unwrap();

			let _ = handle_pr_review_comment(state, data).await;
		}
		_ => {}
	};

	Ok(StatusCode::OK.into_response())
}

async fn code_block(data: PullRequestReviewComment) -> anyhow::Result<String> {
	let start_line = data
		.comment
		.start_line
		.and_then(|x| x.as_i64())
		.or(Some(data.comment.original_line));
	let end_line = data.comment.line.unwrap_or(data.comment.original_line);

	let content = fetch_raw_file(
		CONFIG.github_access_token.clone(),
		data.repository.owner.login,
		data.repository.name,
		data.comment.commit_id,
		data.comment.path.clone(),
	)
	.await?;

	let (skip, take) = if start_line.is_none() || start_line.unwrap() == end_line {
		// only one line was commented on
		// so, take the line noted in `end_line`, and UP TO the four lines before it
		(end_line as usize - 1, 5_usize)
	} else {
		(
			start_line.unwrap() as usize - 1,
			end_line as usize - start_line.unwrap() as usize + 1,
		)
	};

	let lines = content
		.lines()
		.skip(skip)
		.take(take)
		.collect::<Vec<_>>()
		.join("\n");

	let ext = data
		.comment
		.path
		.split('.')
		.last()
		.map(|x| x.to_lowercase())
		.unwrap_or_else(|| "txt".to_string());

	let block = format!("```{}\n{}\n```", ext, lines);

	Ok(block)
}

async fn handle_pr_review_comment(
	state: ChuckleState,
	data: PullRequestReviewComment,
) -> Result<Response> {
	let output = sqlx::query!(
		"select * from pr_review_output where pr_number = $1 and repo_owner = $2 and repo = $3;",
		data.pull_request.number as i32,
		data.repository.owner.login,
		data.repository.name
	)
	.fetch_optional(&state.db)
	.await?;

	if output.is_none() {
		return Ok(StatusCode::OK.into_response());
	}
	let output = output.unwrap();

	let author = sqlx::query!(
		r#"select * from "user" where github_id = $1"#,
		data.comment.user.id as i32
	)
	.fetch_optional(&state.db)
	.await?;

	let user_string = match author {
		Some(user) => format!("<@{}>", user.discord_id.unwrap()),
		None => "Unknown".to_string(),
	};

	let codeblock = code_block(data.clone())
		.await
		.unwrap_or_else(|_| String::from(""));

	let header = format!(
		"### [New Comment](<{}>) from {}",
		user_string, data.comment.links.html.href
	);
	let file_url = format!(
		"https://github.com/{}/{}/blob/{}/{}",
		data.repository.owner.login,
		data.repository.name,
		data.pull_request.head.base_ref,
		data.comment.path,
	);

	let mut subheader = format!("[`{}`]({})", data.comment.path, file_url);
	if let (Some(start), Some(end)) = (
		data.comment.start_line.and_then(|x| x.as_i64()),
		data.comment.line,
	) {
		let comment = format!(" `(L{}-{})`", start, end);
		subheader.push_str(&comment);
	}

	let comment = data.comment.body;
	let content = format!("{header}\n{comment}\n\n{subheader}\n{codeblock}");

	let thread_id = Id::<ChannelMarker>::from_str(&output.thread_id).unwrap();
	let msg = state
		.http_client
		.create_message(thread_id)
		.content(&content)
		.unwrap();

	let res = msg.await.unwrap();
	tracing::debug!("{:#?}", res);

	Ok(StatusCode::OK.into_response())
}
