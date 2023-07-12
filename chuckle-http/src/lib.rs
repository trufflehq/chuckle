use anyhow::Context;
use axum::Router;
use chuckle_util::{ChuckleState, CONFIG};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

mod routes;
mod util;

pub use util::error::{Error, ResultExt};

use self::routes::{status, webhooks};

pub type Result<T, E = Error> = std::result::Result<T, E>;

pub async fn serve(state: ChuckleState) -> anyhow::Result<()> {
	let app = Router::new()
		.merge(routes(state))
		.layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()));

	tracing::info!("Server running on http://172.30.0.1:{}", CONFIG.port);

	axum::Server::bind(&format!("0.0.0.0:{}", CONFIG.port).parse()?)
		.serve(app.into_make_service())
		.await
		.context("Server crashed")
}

fn routes(state: ChuckleState) -> Router<()> {
	Router::new()
		.nest("/api", webhooks::router().merge(status::router()))
		.with_state(state)
}
