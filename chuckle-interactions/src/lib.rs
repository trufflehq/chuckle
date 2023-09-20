#![allow(clippy::unused_unit)] // wtf

#[macro_use]
extern crate serde;

use std::sync::Arc;

use chuckle_util::{
	state::{ChuckleState, State},
	CONFIG,
};
use twilight_model::id::marker::ApplicationMarker;
use twilight_model::id::Id;
use zephyrus::prelude::Framework;

use self::commands::{
	breakout_rooms, config, hexil, link_github, ping, poll, pr_comments, threads,
};

pub mod commands;
pub mod context_menu;
pub mod custom_ids;
pub mod message_components;

pub type ChuckleFramework = Arc<Framework<ChuckleState, ()>>;

pub fn crate_framework(state: ChuckleState) -> anyhow::Result<ChuckleFramework> {
	let http_client = State::http_client();
	let app_id = Id::<ApplicationMarker>::new(CONFIG.discord_application_id.parse()?);

	let framework = Framework::builder(http_client, app_id, state)
		.group(|g| {
			g.name("config")
				.description("Configure the bot for your server.")
				.group(|sub| {
					sub.name("display")
						.description("List the configuration.")
						.command(config::list)
				})
				.group(|sub| {
					sub.name("breakout-category")
						.description("Configure the breakout category.")
						.command(config::breakout_category::list)
						.command(config::breakout_category::set)
						.command(config::breakout_category::unset)
				})
				.group(|sub| {
					sub.name("forum-log")
						.description("Configure the forum log channel.")
						.command(config::forum_log::list)
						.command(config::forum_log::set)
						.command(config::forum_log::unset)
				})
				.group(|sub| {
					sub.name("default-org")
						.description("Configure the default GitHub organization for PR Comments.")
						.command(config::default_org::list)
						.command(config::default_org::set)
						.command(config::default_org::unset)
				})
				.group(|sub| {
					sub.name("default-repo")
						.description("Configure the default GitHub repository for PR Comments.")
						.command(config::default_repo::list)
						.command(config::default_repo::set)
						.command(config::default_repo::unset)
				})
		})
		.group(|g| {
			g.name("breakout-rooms")
				.description("Commands for managing breakout rooms.")
				.command(breakout_rooms::create)
				.command(breakout_rooms::destroy)
		})
		.group(|g| {
			g.name("threads")
				.description("Commands for managing threads.")
				.command(threads::add_role)
		})
		.command(hexil)
		.command(link_github)
		.command(ping)
		.command(poll)
		.command(pr_comments)
		.command(context_menu::circle_back)
		.build();

	Ok(Arc::new(framework))
}

// create the commands lockfile
pub async fn create_lockfile() -> anyhow::Result<String> {
	let state = Arc::new(State::new().await);
	let framework = crate_framework(state)?;

	let commands = framework.twilight_commands();
	println!("{:#?}", commands);

	let json = serde_json::to_string_pretty(&commands)?;

	Ok(json)
}
