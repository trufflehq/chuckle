use crate::CONFIG;
use once_cell::sync::Lazy;
use std::str::FromStr;
use std::sync::Arc;
use twilight_cache_inmemory::InMemoryCache;
use twilight_http::client::InteractionClient;
use twilight_model::id::{marker::ApplicationMarker, Id};

pub type ChuckleState = Arc<State>;

#[derive(Clone)]
pub struct State {
	pub cache: Arc<InMemoryCache>,
	pub db: sqlx::PgPool,
	pub http_client: Arc<twilight_http::Client>,
}

impl State {
	pub async fn new() -> Self {
		let cache = Arc::new(InMemoryCache::builder().message_cache_size(10).build());
		let db = sqlx::PgPool::connect(&CONFIG.database_url).await.unwrap();

		Self {
			cache,
			db,
			http_client: Arc::new(Self::http_client()),
		}
	}

	pub fn http_client() -> twilight_http::Client {
		twilight_http::Client::new(CONFIG.discord_token.clone())
	}

	pub fn interactions_client(&self) -> Arc<InteractionClient> {
		static ID: Lazy<Id<ApplicationMarker>> = Lazy::new(|| {
			Id::<ApplicationMarker>::from_str(&CONFIG.discord_application_id).unwrap()
		});

		Arc::new(self.http_client.interaction(*ID))
	}
}
