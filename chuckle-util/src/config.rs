use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};

pub static CONFIG: Lazy<Config> = Lazy::new(|| Config::new().expect("Unable to retrieve config"));

/// Application Config
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
	/// The port to run the server on
	pub port: u16,
	/// The environment
	pub env: String,
	/// The Discord token
	pub discord_token: String,
	/// The Discord ID
	pub discord_application_id: String,
	/// The database URL
	pub database_url: String,
	/// The GitHub webhook secret
	pub github_webhook_secret: String,
	/// The GitHub token
	pub github_access_token: String,
}

impl Config {
	/// Create a new `Config`
	pub fn new() -> anyhow::Result<Self> {
		let config = envy::from_env::<Self>()?;

		Ok(config)
	}

	pub fn is_dev(&self) -> bool {
		self.env == "development"
	}
}

/// Get the default static `Config`
pub fn get_config() -> &'static Config {
	&CONFIG
}
