use serde::{Deserialize, Serialize};

use crate::app::exercise::Exercise;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sequence {
	pub exercises: Vec<Exercise>,
}

pub fn import_sequence_b() -> anyhow::Result<Sequence> {
	let sequence_b = serde_json::from_str::<Sequence>(include_str!("../data/sequence_b.json"))?;
	Ok(sequence_b)
}

