use chuckle_interactions::create_lockfile;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
	let content = create_lockfile()
		.await
		.expect("Failed to create lockfile content.");

	let path = concat!(env!("CARGO_MANIFEST_DIR"), "/commands.lock.json").to_string();
	std::fs::write(path, content).unwrap();

	Ok(())
}
