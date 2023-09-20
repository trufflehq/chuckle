use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestReviewComment {
	pub action: String,
	pub comment: Comment,
	pub pull_request: PullRequest,
	pub repository: Repo,
	pub organization: Organization,
	pub sender: Sender,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Comment {
	pub url: String,
	pub pull_request_review_id: i64,
	pub id: i64,
	pub node_id: String,
	pub diff_hunk: String,
	pub path: String,
	pub commit_id: String,
	pub original_commit_id: String,
	pub user: Sender,
	pub body: String,
	pub created_at: String,
	pub updated_at: String,
	pub html_url: String,
	pub pull_request_url: String,
	pub author_association: String,
	#[serde(rename = "_links")]
	pub links: CommentLinks,
	pub reactions: Reactions,
	pub start_line: Option<serde_json::Value>,
	pub original_start_line: Option<serde_json::Value>,
	pub start_side: Option<serde_json::Value>,
	pub line: Option<i64>,
	pub original_line: i64,
	pub side: String,
	pub original_position: i64,
	pub position: i64,
	pub subject_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommentLinks {
	#[serde(rename = "self")]
	pub links_self: Html,
	pub html: Html,
	pub pull_request: Html,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Html {
	pub href: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reactions {
	pub url: String,
	pub total_count: i64,
	#[serde(rename = "+1")]
	pub the_1: i64,
	#[serde(rename = "-1")]
	pub reactions_1: i64,
	pub laugh: i64,
	pub hooray: i64,
	pub confused: i64,
	pub heart: i64,
	pub rocket: i64,
	pub eyes: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sender {
	pub login: String,
	pub id: i64,
	pub node_id: String,
	pub avatar_url: String,
	pub gravatar_id: String,
	pub url: String,
	pub html_url: String,
	pub followers_url: String,
	pub following_url: String,
	pub gists_url: String,
	pub starred_url: String,
	pub subscriptions_url: String,
	pub organizations_url: String,
	pub repos_url: String,
	pub events_url: String,
	pub received_events_url: String,
	#[serde(rename = "type")]
	pub sender_type: String,
	pub site_admin: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Organization {
	pub login: String,
	pub id: i64,
	pub node_id: String,
	pub url: String,
	pub repos_url: String,
	pub events_url: String,
	pub hooks_url: String,
	pub issues_url: String,
	pub members_url: String,
	pub public_members_url: String,
	pub avatar_url: String,
	pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequest {
	pub url: String,
	pub id: i64,
	pub node_id: String,
	pub html_url: String,
	pub diff_url: String,
	pub patch_url: String,
	pub issue_url: String,
	pub number: i64,
	pub state: String,
	pub locked: bool,
	pub title: String,
	pub user: Sender,
	pub body: Option<serde_json::Value>,
	pub created_at: String,
	pub updated_at: String,
	pub closed_at: Option<serde_json::Value>,
	pub merged_at: Option<serde_json::Value>,
	pub merge_commit_sha: Option<String>,
	pub assignee: Option<serde_json::Value>,
	pub assignees: Vec<Option<serde_json::Value>>,
	pub requested_reviewers: Vec<Option<serde_json::Value>>,
	pub requested_teams: Vec<Option<serde_json::Value>>,
	pub labels: Vec<Option<serde_json::Value>>,
	pub milestone: Option<serde_json::Value>,
	pub draft: bool,
	pub commits_url: String,
	pub review_comments_url: String,
	pub review_comment_url: String,
	pub comments_url: String,
	pub statuses_url: String,
	pub head: Base,
	pub base: Base,
	#[serde(rename = "_links")]
	pub links: PullRequestLinks,
	pub author_association: String,
	pub auto_merge: Option<serde_json::Value>,
	pub active_lock_reason: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Base {
	pub label: String,
	#[serde(rename = "ref")]
	pub base_ref: String,
	pub sha: String,
	pub user: Sender,
	pub repo: Repo,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Repo {
	pub id: i64,
	pub node_id: String,
	pub name: String,
	pub full_name: String,
	pub private: bool,
	pub owner: Sender,
	pub html_url: String,
	pub description: String,
	pub fork: bool,
	pub url: String,
	pub forks_url: String,
	pub keys_url: String,
	pub collaborators_url: String,
	pub teams_url: String,
	pub hooks_url: String,
	pub issue_events_url: String,
	pub events_url: String,
	pub assignees_url: String,
	pub branches_url: String,
	pub tags_url: String,
	pub blobs_url: String,
	pub git_tags_url: String,
	pub git_refs_url: String,
	pub trees_url: String,
	pub statuses_url: String,
	pub languages_url: String,
	pub stargazers_url: String,
	pub contributors_url: String,
	pub subscribers_url: String,
	pub subscription_url: String,
	pub commits_url: String,
	pub git_commits_url: String,
	pub comments_url: String,
	pub issue_comment_url: String,
	pub contents_url: String,
	pub compare_url: String,
	pub merges_url: String,
	pub archive_url: String,
	pub downloads_url: String,
	pub issues_url: String,
	pub pulls_url: String,
	pub milestones_url: String,
	pub notifications_url: String,
	pub labels_url: String,
	pub releases_url: String,
	pub deployments_url: String,
	pub created_at: String,
	pub updated_at: String,
	pub pushed_at: String,
	pub git_url: String,
	pub ssh_url: String,
	pub clone_url: String,
	pub svn_url: String,
	pub homepage: Option<serde_json::Value>,
	pub size: i64,
	pub stargazers_count: i64,
	pub watchers_count: i64,
	pub language: String,
	pub has_issues: bool,
	pub has_projects: bool,
	pub has_downloads: bool,
	pub has_wiki: bool,
	pub has_pages: bool,
	pub has_discussions: bool,
	pub forks_count: i64,
	pub mirror_url: Option<serde_json::Value>,
	pub archived: bool,
	pub disabled: bool,
	pub open_issues_count: i64,
	pub license: Option<serde_json::Value>,
	pub allow_forking: bool,
	pub is_template: bool,
	pub web_commit_signoff_required: bool,
	pub topics: Vec<Option<serde_json::Value>>,
	pub visibility: String,
	pub forks: i64,
	pub open_issues: i64,
	pub watchers: i64,
	pub default_branch: String,
	pub allow_squash_merge: Option<bool>,
	pub allow_merge_commit: Option<bool>,
	pub allow_rebase_merge: Option<bool>,
	pub allow_auto_merge: Option<bool>,
	pub delete_branch_on_merge: Option<bool>,
	pub allow_update_branch: Option<bool>,
	pub use_squash_pr_title_as_default: Option<bool>,
	pub squash_merge_commit_message: Option<String>,
	pub squash_merge_commit_title: Option<String>,
	pub merge_commit_message: Option<String>,
	pub merge_commit_title: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PullRequestLinks {
	#[serde(rename = "self")]
	pub links_self: Html,
	pub html: Html,
	pub issue: Html,
	pub comments: Html,
	pub review_comments: Html,
	pub review_comment: Html,
	pub commits: Html,
	pub statuses: Html,
}

#[cfg(test)]
mod tests {
	use format_serde_error::SerdeError;

	use crate::pull_request_review_comment::PullRequestReviewComment;

	#[test]
	fn test_parse() -> Result<(), anyhow::Error> {
		static DATA: &[u8] = include_bytes!(concat!(
			env!("CARGO_MANIFEST_DIR"),
			"/data/pull_request_review_comment.json"
		));

		let data_string = String::from_utf8_lossy(DATA);

		let data = serde_json::from_slice::<PullRequestReviewComment>(DATA)
			.map_err(|err| SerdeError::new(data_string.to_string(), err))?;
		// assert!(res.is_ok());

		// let data = res.unwrap();
		assert_eq!(data.action, "created");
		assert_eq!(data.comment.user.login, "austinhallock");
		assert_eq!(data.comment.author_association, "CONTRIBUTOR");
		assert_eq!(data.pull_request.number, 116);
		assert_eq!(data.pull_request.base.base_ref, "main");

		Ok(())
	}
}
