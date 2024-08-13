use serde::{Deserialize, Serialize};

use crate::app::exercise::Exercise;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Sequence {
	pub name: String,
	pub start_num: usize,
	pub end_num: usize,
	pub exercises: Vec<Exercise>,
}

#[derive(Debug, Copy, Clone)]
pub enum Cycle {
	B,
	C,
}

pub fn import_cycle(cycle: Cycle) -> anyhow::Result<Sequence> {
	let json = match cycle {
		Cycle::B => include_str!("../data/sequence_b.json"),
		Cycle::C => include_str!("../data/sequence_c.json"),
	};
	let sequence = serde_json::from_str::<Sequence>(json)?;
	Ok(sequence)
}

