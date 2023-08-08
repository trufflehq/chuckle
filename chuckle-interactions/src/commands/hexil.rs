use chuckle_util::ChuckleState;
use std::str::FromStr;
use twilight_model::{
	guild::Permissions,
	id::{marker::RoleMarker, Id},
};
use zephyrus::prelude::*;

use super::{handle_generic_error, text_response, user_from_interaction};

#[tracing::instrument(skip(ctx))]
#[command]
#[description = "Set a custom role color."]
#[only_guilds]
#[error_handler(handle_generic_error)]
pub async fn hexil(
	ctx: &SlashContext<ChuckleState>,
	#[rename = "hex"]
	#[description = "A hex code to set your role color to."]
	hex_input: String,
) -> DefaultCommandResult {
	let user = user_from_interaction(&ctx.interaction);
	let hex_input = hex_input.replace('#', "");

	// check if the hex code is valid color
	let hex = match hex::decode(hex_input.clone()) {
		Ok(h) => h,
		Err(_) => {
			return text_response(
				ctx,
				format!("`#{}` is not a valid hex code.", hex_input,),
				false,
			)
			.await;
		}
	};
	let hex_int = hex.iter().fold(0, |acc, &x| (acc << 8) + x as u32);
	let guild_id = ctx.interaction.guild_id.unwrap();

	let existing = sqlx::query!(
		r#"SELECT * FROM "hexil" WHERE guild_id = $1 and user_id = $2"#,
		guild_id.to_string(),
		user.id.to_string()
	)
	.fetch_optional(&ctx.data.db)
	.await?;

	if existing.is_some() {
		let role_id = Id::<RoleMarker>::from_str(&existing.unwrap().role_id).unwrap();
		// change the color of the role
		let _ = ctx
			.http_client()
			.update_role(guild_id, role_id)
			.color(Some(hex_int))
			.name(Some(&user.name));
	} else {
		// create role then create database entry
		let role = ctx
			.http_client()
			.create_role(guild_id)
			.color(hex_int)
			.name(&user.name)
			.permissions(Permissions::from_bits_truncate(0))
			.await?
			.model()
			.await?;

		let _ = sqlx::query!(
			r#"insert into "hexil" (guild_id, user_id, role_id) values ($1, $2, $3)"#,
			guild_id.to_string(),
			user.id.to_string(),
			role.id.to_string()
		)
		.execute(&ctx.data.db)
		.await?;
	}

	text_response(
		ctx,
		format!("Successfully set your role color to `#{}`.", hex_input),
		true,
	)
	.await
}
