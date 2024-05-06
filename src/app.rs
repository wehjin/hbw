use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
	let world = use_state(|| "World".to_string());
	html! {
	<>
	<section class="section">
	<div class="container">
		<h1 class="title">{"Happy Body Worksheet"}</h1>
		<p class="subtitle">{"My first website with "}<strong>{"Bulma"}</strong>{"!"}</p>
		<p>{"Base Weight: "}<span class="tag is-primary is-light is-large">{"3lbs"}</span></p>
		<p class="title is-4">{"Section 2 Exercises"}</p>
		<p>
		<table class="table">
		<thead>
		<tr>
			<th></th>
			<th>{"Exercise 1"}</th>
		</tr>
		</thead>
		<tbody>
		<tr>
			<th>{"Starting Position"}</th>
			<td>{"Hold a 1 lb weight in each hand. Lie on your back with your arms spread out on both sides like the letter T. The palms of your hands are facing upwards toward the ceiling."}</td>
		</tr>
		<tr>
			<th>{"Breathe In"}</th>
			<td>{"Take a deep breath, tighten the muscles in your legs while the knees straight, and curl your toes toward your head."}</td>
		</tr>
		<tr>
			<th>{"Primary Movement"}</th>
			<td>{"Raise your arms up and towards each other while keeping the elbows straight. Stop when your hands are straight above you and your arms are pointing towards the ceiling."}</td>
		</tr>
		<tr>
			<th>{"Extra Movement"}</th>
			<td>{"Push your hands up and closer to the ceiling using your shoulders and chest. Stop when your hands are as close to the ceiling as they go."}</td>
		</tr>
		<tr>
			<th>{"Breathe Out"}</th>
			<td>{"Relax your shoulders, then move your arms move back to the floor. When your hands reach the floors, uncurl your toes and breathe out."}</td>
		</tr>
		<tr>
			<th>{"Next"}</th>
			<td>{"Continue with Exercise 2."}</td>
		</tr>
		</tbody>
		</table>
		</p>
	</div>
	</section>
	</>
	}
}
