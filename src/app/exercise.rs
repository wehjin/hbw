use crate::app::step::Step;

#[derive(Debug, Clone)]
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

impl Exercise {
	pub fn new(name: &str, target: &str, start: Step, energize: &str, primary: Vec<Step>, extra: Option<Step>, relax: Vec<Step>, next: &str, equipment: &str) -> Self {
		Self {
			name: name.to_string(),
			target: target.to_string(),
			start,
			breath_in: energize.to_string(),
			primary,
			extra,
			breath_out: relax,
			next: next.to_string(),
			equipment: equipment.to_string(),
		}
	}
}