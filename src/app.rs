use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
	let world = use_state(|| Exercise::from([
		"Arms",
		"Hold a 1 lb weight in each hand. Lie on your back with your arms spread out on both sides like the letter T. The palms of your hands are facing upwards toward the ceiling.",
		"Take a deep breath, tighten the muscles in your legs while keeping the knees straight, and curl your toes toward your head.",
		"Raise your arms up and towards each other while keeping the elbows straight. Stop when your hands are straight above you and your arms are pointing towards the ceiling.",
		"Push your hands up and closer to the ceiling using your shoulders and chest. Stop when your hands are as close to the ceiling as they go.",
		"Relax your shoulders, then move your arms move back to the floor. When your hands reach the floors, uncurl your toes and breathe out.",
		"Continue with Exercise 2."
	]));
	let exercise = &*world;
	html! {
	<>
	<section class="section">
	<div class="container">
		<h1 class="title">{"Happy Body Worksheet"}</h1>
		<p class="subtitle">{"My first website with "}<strong>{"Bulma"}</strong>{"!"}</p>
		<p>{"Base Weight: "}<span class="tag is-primary is-light is-large">{"3 lbs"}</span></p>
		<p class="title is-4">{"Section 2 Exercises"}</p>
		<p>
		<table class="table">
		<thead>
		<tr>
			<th></th>
			<th>
				<p>{"Exercise 1"}</p>
				<p class="tag is-info">{&exercise.target}</p>
			</th>
		</tr>
		</thead>
		<tbody>
		<tr>
			<th>{"Starting Position"}</th>
			<td>{&exercise.start_position}</td>
		</tr>
		<tr>
			<th>{"Breathe In"}</th>
			<td>{&exercise.breath_in}</td>
		</tr>
		<tr>
			<th>{"Primary Movement"}</th>
			<td>{&exercise.primary}</td>
		</tr>
		<tr>
			<th>{"Extra Movement"}</th>
			<td>{&exercise.extra}</td>
		</tr>
		<tr>
			<th>{"Breathe Out"}</th>
			<td>{&exercise.breath_out}</td>
		</tr>
		<tr>
			<th>{"Next"}</th>
			<td>{&exercise.next}</td>
		</tr>
		</tbody>
		</table>
		</p>
	</div>
	</section>
	</>
	}
}


#[derive(Debug, Clone)]
struct Exercise {
	pub target: String,
	pub start_position: String,
	pub breath_in: String,
	pub primary: String,
	pub extra: String,
	pub breath_out: String,
	pub next: String,
}

impl From<[&'static str; 7]> for Exercise {
	fn from(value: [&'static str; 7]) -> Self {
		Self {
			target: value[0].to_string(),
			start_position: value[1].to_string(),
			breath_in: value[2].to_string(),
			primary: value[3].to_string(),
			extra: value[4].to_string(),
			breath_out: value[5].to_string(),
			next: value[6].to_string(),
		}
	}
}
