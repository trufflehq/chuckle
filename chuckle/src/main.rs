use chuckle_interactions::crate_framework;
use chuckle_util::{state::State, ChuckleState};
use std::sync::Arc;
use tracing_subscriber::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let registry = tracing_subscriber::registry().with(
		tracing_subscriber::EnvFilter::try_from_default_env()
			.unwrap_or_else(|_| "debug,hyper=info,tower_http=info,rustls=info".into()),
	);

	// #[cfg(not(debug_assertions))]
	// let registry = registry.with(tracing_subscriber::fmt::layer().json());
	#[cfg(debug_assertions)]
	let registry = registry.with(tracing_subscriber::fmt::layer());

	registry.init();

	let state: ChuckleState = Arc::new(State::new().await);
	let framework = crate_framework(state.clone()).expect("Failed to create zephryus framework");

	let commands = framework
		.commands
		.values()
		.map(|c| c.name)
		.collect::<Vec<_>>()
		.join(", ");
	tracing::info!("Loaded commands: {:#?}", commands);

	let groups = framework
		.groups
		.values()
		.map(|g| g.name)
		.collect::<Vec<_>>()
		.join(", ");
	tracing::info!("Loaded groups: {:#?}", groups);

	tokio::spawn(chuckle_jobs::start(state.clone()));
	tokio::spawn(chuckle_http::serve(state.clone()));
	let _ = tokio::spawn(chuckle_gateway::create_gateway(state, framework)).await;

	Ok(())
}
