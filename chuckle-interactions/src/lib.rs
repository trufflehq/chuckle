#![allow(clippy::unused_unit)] // wtf
use std::{error::Error, sync::Arc};

use chuckle_util::{
	state::{ChuckleState, State},
	CONFIG,
};
use twilight_model::application::command::{
	Command, CommandOption, CommandOptionType, CommandType,
};
use twilight_model::id::Id;
use twilight_util::builder::command::CommandBuilder;
use zephyrus::{
	argument::CommandArgument,
	group::{GroupParent, ParentType},
	prelude::Framework,
	twilight_exports::ApplicationMarker,
};

use self::commands::{config, hexil, link_github, ping, pr_comments};

pub mod commands;
pub mod context_menu;
pub mod modals;

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
		.command(hexil)
		.command(link_github)
		.command(ping)
		.command(pr_comments)
		.build();

	Ok(Arc::new(framework))
}

// create the commands lockfile
#[cfg(feature = "lockfile")]
pub async fn create_lockfile() -> anyhow::Result<String> {
	let state = Arc::new(State::new().await);
	let framework = crate_framework(state)?;

	let mut commands = build_commands_object(&framework);
	println!("{:#?}", commands);

	let notification = CommandBuilder::new("Circle back", "", CommandType::Message).build();
	commands.push(notification);

	let json = serde_json::to_string_pretty(&commands)?;
	// println!("{}", json);

	Ok(json)
}

// Since zephyrus doesn't support context-menu commands yet,
// we have to do some hacky shit to help with the migration of **all**
// of our commands/interactions.
#[cfg(feature = "lockfile")]
fn build_commands_object(framework: &ChuckleFramework) -> Vec<Command> {
	let mut commands = Vec::new();

	for cmd in framework.commands.values() {
		let mut command = CommandBuilder::new(cmd.name, cmd.description, CommandType::ChatInput);

		// add options to command

		for i in &cmd.arguments {
			command = command.option(i.as_option());
		}

		if let Some(permissions) = &cmd.required_permissions {
			command = command.default_member_permissions(*permissions);
		}

		commands.push(command.build());
	}

	for group in framework.groups.values() {
		let options = create_group(group);
		let mut command =
			CommandBuilder::new(group.name, group.description, CommandType::ChatInput);

		for i in options {
			command = command.option(i);
		}

		if let Some(permissions) = &group.required_permissions {
			command = command.default_member_permissions(*permissions);
		}

		commands.push(command.build());
	}

	commands
}

fn create_group(
	group: &GroupParent<Arc<State>, (), Box<dyn Error + Send + Sync>>,
) -> Vec<CommandOption> {
	if let ParentType::Group(map) = &group.kind {
		let mut subgroups = Vec::new();
		for group in map.values() {
			let mut subcommands = Vec::new();
			for sub in group.subcommands.values() {
				subcommands.push(create_subcommand(sub))
			}

			subgroups.push(CommandOption {
				kind: CommandOptionType::SubCommandGroup,
				name: group.name.to_string(),
				description: group.description.to_string(),
				options: Some(subcommands),
				autocomplete: None,
				choices: None,
				required: None,
				channel_types: None,
				description_localizations: None,
				max_length: None,
				max_value: None,
				min_length: None,
				min_value: None,
				name_localizations: None,
			});
		}
		subgroups
	} else if let ParentType::Simple(map) = &group.kind {
		let mut subcommands = Vec::new();
		for sub in map.values() {
			subcommands.push(create_subcommand(sub));
		}

		subcommands
	} else {
		unreachable!()
	}
}

fn create_subcommand(
	cmd: &zephyrus::command::Command<Arc<State>, (), Box<dyn Error + Send + Sync>>,
) -> CommandOption {
	CommandOption {
		kind: CommandOptionType::SubCommand,
		name: cmd.name.to_string(),
		description: cmd.description.to_string(),
		options: Some(arg_options(&cmd.arguments)),
		autocomplete: None,
		choices: None,
		required: None,
		channel_types: None,
		description_localizations: None,
		max_length: None,
		max_value: None,
		min_length: None,
		min_value: None,
		name_localizations: None,
	}
}

fn arg_options(arguments: &Vec<CommandArgument<Arc<State>>>) -> Vec<CommandOption> {
	let mut options = Vec::with_capacity(arguments.len());

	for arg in arguments {
		options.push(arg.as_option());
	}

	options
}
