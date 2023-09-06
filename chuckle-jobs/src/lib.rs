use std::time::Duration;

use chuckle_util::ChuckleState;
use clokwerk::{AsyncScheduler, TimeUnits};
use tokio::time::sleep;

mod circle_back;
mod sweep_modals;
mod sweep_notifications;

pub async fn start(state: ChuckleState) {
	tracing::debug!("Starting workers");

	let mut scheduler = AsyncScheduler::new();

	let state_clone = state.clone();
	scheduler.every(30.seconds()).run(move || {
		let state = state_clone.clone();
		async move {
			circle_back::run(&state).await.unwrap();
		}
	});

	let state_clone = state.clone();
	scheduler.every(5.minutes()).run(move || {
		let state = state_clone.clone();
		async move {
			sweep_modals::run(&state).await.unwrap();
		}
	});

	let state_clone = state.clone();
	scheduler.every(5.minutes()).run(move || {
		let state = state_clone.clone();
		async move {
			sweep_notifications::run(&state).await.unwrap();
		}
	});

	loop {
		scheduler.run_pending().await;
		sleep(Duration::from_millis(500)).await;
	}
}
