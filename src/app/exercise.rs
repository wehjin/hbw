use serde::{Deserialize, Serialize};

use crate::app::step::Step;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Exercise {
	pub name: String,
	pub target: String,
	pub start: Step,
	pub breath_in: String,
	pub primary: Vec<Step>,
	pub extra: Option<Step>,
	pub breath_out: Vec<Step>,
	pub next: String,
	pub equipment: String,
}
