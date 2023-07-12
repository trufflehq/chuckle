pub mod pull_request_review_comment;

/// Fetches a raw file fro githubusercontent.com
pub async fn fetch_raw_file(
	token: String,
	owner: String,
	name: String,
	r#ref: String,
	path: String,
) -> anyhow::Result<String> {
	let file_url = format!(
		"https://raw.githubusercontent.com/{}/{}/{}/{}",
		owner, name, r#ref, path
	);

	let client = reqwest::Client::new();
	let resp = client
		.get(&file_url)
		.header("Authorization", format!("Token {token}"))
		.send()
		.await?;

	let body = resp.text().await?;

	Ok(body)
}
