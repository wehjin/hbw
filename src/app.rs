use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
	let world = use_state(|| {
		[
			Exercise::from([
				"Exercise 1",
				"Arms",
				"Hold a 1 lb weight in each hand. Lie on your back with your arms spread out on both sides like the letter T. The palms of your hands are facing upwards toward the ceiling.",
				"Take a deep breath, tighten the muscles in your legs while keeping the knees straight, and curl your toes toward your head.",
				"Raise your arms up and towards each other while keeping the elbows straight. Stop when your hands are straight above you and your arms are pointing towards the ceiling.",
				"Push your hands up and closer to the ceiling using your shoulders and chest. Stop when your hands are as close to the ceiling as they go.",
				"Relax your shoulders, then move your arms move back to the floor. When your hands reach the floors, uncurl your toes and breathe out.",
				"Continue with Exercise 2."
			]),
		]
	});
	let exercises = &*world;
	let headers = exercises.iter().map(|exercise| {
		html! {
			<th>
				<p>{&exercise.name}</p>
				<p class="tag is-info">{&exercise.target}</p>
			</th>
		}
	}).collect::<Vec<Html>>();
	let starts = exercises.iter().map(|exercise| {
		html! {
			<td>{&exercise.start_position}</td>
		}
	}).collect::<Vec<Html>>();
	let breath_ins = exercises.iter().map(|exercise| {
		html! {
			<td>{&exercise.breath_in}</td>
		}
	}).collect::<Vec<Html>>();
	let primaries = exercises.iter().map(|exercise| {
		html! {
			<td>{&exercise.primary}</td>
		}
	}).collect::<Vec<Html>>();
	let extras = exercises.iter().map(|exercise| {
		html! {
			<td>{&exercise.extra}</td>
		}
	}).collect::<Vec<Html>>();
	let breath_outs = exercises.iter().map(|exercise| {
		html! {
			<td>{&exercise.breath_out}</td>
		}
	}).collect::<Vec<Html>>();
	let nexts = exercises.iter().map(|exercise| {
		html! {
			<td>{&exercise.next}</td>
		}
	}).collect::<Vec<Html>>();
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
			<th></th>{headers}
		</tr>
		</thead>
		<tbody>
		<tr><th>{"Starting Position"}</th>{starts}</tr>
		<tr><th>{"Breathe In"}</th>{breath_ins}</tr>
		<tr><th>{"Primary Movement"}</th>{primaries}</tr>
		<tr><th>{"Extra Movement"}</th>{extras}</tr>
		<tr><th>{"Breathe Out"}</th>{breath_outs}</tr>
		<tr><th>{"Next"}</th>{nexts}</tr>
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
	pub name: String,
	pub target: String,
	pub start_position: String,
	pub breath_in: String,
	pub primary: String,
	pub extra: String,
	pub breath_out: String,
	pub next: String,
}

impl From<[&'static str; 8]> for Exercise {
	fn from(value: [&'static str; 8]) -> Self {
		Self {
			name: value[0].to_string(),
			target: value[1].to_string(),
			start_position: value[2].to_string(),
			breath_in: value[3].to_string(),
			primary: value[4].to_string(),
			extra: value[5].to_string(),
			breath_out: value[6].to_string(),
			next: value[7].to_string(),
		}
	}
}
