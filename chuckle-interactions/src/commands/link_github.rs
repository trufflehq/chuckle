use chuckle_util::ChuckleState;
use vesper::prelude::*;

use super::{handle_generic_error, text_response, user_from_interaction};

#[tracing::instrument]
async fn fetch_user_id(username: &String) -> anyhow::Result<i32> {
	let url = format!("https://api.github.com/users/{}", username);
	let resp = reqwest::Client::new()
		.get(url)
		.header("User-Agent", "chuckle-bot (github.com/trufflehq/chuckle)")
		.send()
		.await?;
	let user: serde_json::Value = resp.json().await?;

	let user_id = user.get("id").unwrap().as_i64().unwrap() as i32;

	Ok(user_id)
}

#[tracing::instrument(skip(ctx))]
#[command("link-github")]
#[description = "Set a custom role color."]
#[only_guilds]
#[error_handler(handle_generic_error)]
pub async fn link_github(
	ctx: &SlashContext<ChuckleState>,
	#[description = "Your GitHub username."] username: String,
) -> DefaultCommandResult {
	let user = user_from_interaction(&ctx.interaction);

	let github_user = match fetch_user_id(&username).await {
		Ok(n) => n,
		Err(err) => {
			tracing::error!("Error fetching GitHub user: {:?}", err);

			return text_response(
				ctx,
				format!(
					r#"
                        Couldn't find the GitHub user `{}`.
                        ```
                        {:#?}
                        ```
                    "#,
					username, err
				),
				true,
			)
			.await;
		}
	};

	let existing_entry = sqlx::query!(
		r#"SELECT * FROM "user" WHERE discord_id = $1"#,
		user.id.to_string()
	)
	.fetch_optional(&ctx.data.db)
	.await?;

	if existing_entry.is_some() {
		let _ = sqlx::query!(
			r#"update "user" set github_id = $1 where id = $2 returning id"#,
			github_user,
			existing_entry.unwrap().id,
		)
		.fetch_one(&ctx.data.db)
		.await?;
	} else {
		let _ = sqlx::query!(
			r#"insert into "user" (discord_id, github_id) values ($1, $2)"#,
			user.id.to_string(),
			github_user,
		)
		.fetch_one(&ctx.data.db)
		.await?;
	}

	text_response(
		ctx,
		format!(
			"Successfully registered your GitHub username as `{}` (`{}`).",
			username, github_user
		),
		false,
	)
	.await
}
