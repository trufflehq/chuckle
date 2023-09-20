use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct PollOptionCustomId {
	/// The id of the `poll_option` row
	pub id: Uuid,
	/// The id of the `poll` row
	pub poll_id: Uuid,
	/// The option number
	pub option: i8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub struct PollClearCustomId {
	/// The id of the `poll` row
	pub poll_id: Uuid,
}
