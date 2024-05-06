use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
	let world = use_state(|| {
		[
			Exercise::from([
				"Exercise 7",
				"Arms",
				"Hold a 1 lb weight in each hand. Lie on your back with your arms spread out on both sides like the letter T. The palms of your hands are facing upwards toward the ceiling.",
				"Take a deep breath and tighten the muscles in your legs while keeping your knees straight. Curl your toes toward your face.",
				"Raise your arms up and towards each other while keeping the elbows straight. Stop when your hands are straight above you and your arms are pointing towards the ceiling.",
				"Push your hands up and closer to the ceiling using your shoulders and chest. Stop when your hands are as close to the ceiling as they go.",
				"Relax your shoulders, then move your arms move back to the floor. When your hands reach the floors, uncurl your toes and breathe out.",
				"Practice Exercise 7 six times, then move on to Exercise 8."
			]),
			Exercise::from([
				"Exercise 8",
				"Abdomen",
				"Lie on your back with your legs and the bottoms of you feet pointing up towards the ceiling. Place both arms on the floor above your head.",
				"Take a deep breath, straighten your knee, and tighten the muscles in your legs. Curl your toes toward your face.",
				"Move your left arm to the floor on the your left side while reaching your right hand towards the toes of your left foot. Stop when your right hand reaches your toes.",
				"Push your right hand further up towards the ceiling, trying to reach beyond the toes of your left foot.",
				"Relax your abdomen and move both hands back to the floor about your head.  When your hands reach the floor, uncurl your toes and breathe out.",
				"Practice Exercise 8 six times, then move on to Exercise 9."
			]),
			Exercise::from([
				"Exercise 9",
				"Back",
				"Lie on your stomach with your palms touching the floor near your ears. Turn your head to look towards your right hand (left cheek on the floor).",
				"Take a deep breath and tighten the muscles in your legs while keeping your knees straight. Point your toes away from your face.",
				"Turn your face to look straight ahead and raise your head and chest off the floor. Stop when you are looking at the wall in front of you.",
				"Tighten your back and tilt your head even further. Stop when you are looking at the spot where the wall in front of you meets the ceiling.",
				"Relax your back and move your head back to the floor, this time looking towards your left hand (right cheek on the floor). When your cheek is back on the floor, relax your legs and breathe out.",
				"Train Exercise 9 six times, then move on to Exercise 10."
			]),
			Exercise::from([
				"Exercise 10",
				"Arms",
				"Place one 8-lb weight on the ground and stand above it with hips and both knees bent. If you let your arms down, your hands should touch the weight, but start with both hands resting on your knees.",
				"Take a deep breath and tighten the muscles in your legs while keeping your knees bent. Curl your toes toward your face.",
				"Move your right hand down (leave the left hand on your leg) and grab the weight with your right hand. Bring the weight straight up to your chest using your right arm. Stop when your right elbow is fully bent and the weight is near your chest.",
				"Turn your entire upper body to your right and look at the wall on your right while keeping the weight at your chest. The weight should move upwards as your torso rotates. Stop when your torso is facing the right wall,",
				"Relax your torso and arm, and move the weight back to the floor in front of you. When the weight is on the floor, uncurl your toes and breathe out.",
				"Practice Exercise 10 six times (3 times right, 3 times left), then move on to Exercise 11."
			]),
			Exercise::from([
				"Exercise 11",
				"Legs",
				"Stand with legs shoulder-width apart and hands down by your side hold a 3-lb weight in each hand.",
				"Take a deep breath and tighten the muscles in your legs while keeping your legs straight. Curl your toes toward your face.",
				"1. Bend your elbows and bring the weights forward in front of you eyes. 2. Then rotate your arms outwards and stop when the weights are by your ears. 3. Push the weights straight up towards the ceiling and stop when your elbows are straight. 4. Squat down by bending your knees and pushing your butt backwards. Keep your arms straight and above your head. Stop when your knees are fully bent. 5. Stand back up while keeping your arms straight and above your head.",
				"",
				"1. Bend your elbows and bring the weights down and stop when they are near your ears. 2. Rotate your arms towards each other and stop when your hands and the weights are directly in front of your eyes. 3. Drop your hands and until the weights are at your side by your legs. 4. Uncurl your toes and breathe out.",
				"Practice Exercise 11 six times, then move on to Exercise 12."
			]),
			Exercise::from([
				"Exercise 12",
				"Arms",
				"",
				"",
				"",
				"",
				"",
				"Practice Exercise 12 six times, then start again at Exercise 12 for 2 more cycles."
			]),
		]
	});
	let exercises = &*world;
	let headers = exercises.iter().map(|exercise| {
		html! {
			<th>
				<p class="tag is - info">{&exercise.target}</p>
				<p>{&exercise.name}</p>
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
		<p>{"Base Weight: "}<span class="tag is - primary is - light is - large">{"3 lbs"}</span></p>
		<p class="title is - 4">{"Section 2 Exercises"}</p>
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
