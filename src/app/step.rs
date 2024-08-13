#[derive(Debug, Clone)]
pub struct Step {
	pub caption: String,
	pub image_url: String,
}

impl Step {
	pub fn new(image_url: &str, caption: &str) -> Self {
		Self { caption: caption.to_string(), image_url: image_url.to_string() }
	}
}
