use axum::{
	http::StatusCode,
	response::{IntoResponse, Response},
	routing::any,
	Router,
};
use chuckle_util::ChuckleState;

use crate::Result;

pub fn router() -> Router<ChuckleState> {
	Router::new().route("/status", any(status))
}

#[axum::debug_handler]
async fn status() -> Result<Response> {
	Ok(StatusCode::OK.into_response())
}
