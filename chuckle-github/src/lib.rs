pub mod pull_request_review_comment;

const CHUCKLE_USER_AGENT: &str = concat!(
	env!("CARGO_PKG_NAME"),
	"/",
	env!("CARGO_PKG_VERSION"),
	" (",
	env!("CARGO_PKG_HOMEPAGE"),
	")"
);

/// Fetches a raw file fro githubusercontent.com
pub async fn fetch_raw_file(
	token: String,
	owner: String,
	name: String,
	commit: String,
	path: String,
) -> anyhow::Result<String> {
	let file_url =
		format!("https://api.github.com/repos/{owner}/{name}/contents/{path}?ref={commit}",);

	let client = reqwest::Client::new();
	let resp = client
		.get(&file_url)
		.header("Authorization", format!("token {token}"))
		.header("User-Agent", CHUCKLE_USER_AGENT)
		.header("Accept", "application/vnd.github.raw")
		.send()
		.await?;

	let body = resp.text().await?;

	Ok(body)
}

#[cfg(test)]
mod test {
	#[tokio::test]
	async fn test_fetch_raw_file() {
		let token = std::env::var("GITHUB_TOKEN").unwrap();
		let owner = std::env::var("GITHUB_OWNER").unwrap_or("trufflehq".into());
		let name = std::env::var("GITHUB_REPO").unwrap_or("chuckle".into());
		let commit = std::env::var("GITHUB_COMMIT").unwrap_or("HEAD".into());
		let path = std::env::var("GITHUB_PATH").unwrap_or("README.md".into());

		let file = super::fetch_raw_file(token, owner, name, commit, path).await;
		assert!(file.is_ok());
		let file = file.unwrap();

		assert!(file.len() > 0);
	}
}
