use super::{handle_generic_error, text_response};
use chuckle_util::ChuckleState;
use zephyrus::prelude::*;

#[command]
#[description = "Ping the bot."]
#[error_handler(handle_generic_error)]
pub async fn ping(ctx: &SlashContext<ChuckleState>) -> DefaultCommandResult {
	text_response(ctx, "Pong!".to_string(), true).await
}