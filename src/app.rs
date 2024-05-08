use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
	let world = use_state(|| [
		Exercise::new(
			"Exercise 7",
			"Arms",
			Step::new(
				"assets/ex07_start.png",
				"Hold a 1 lb weight in each hand. Lie on your back with your arms straight, spread out on both sides like the letter T. The palms of your hands are facing upwards toward the ceiling.",
			),
			"Take a deep breath and tighten the muscles in your legs while keeping your knees straight. Curl your toes toward your face.",
			vec![
				Step::new(
					"assets/ex07_motion.png",
					"Raise your arms up and towards each other while keeping the elbows straight. Stop when your hands are straight above you and your arms are pointing towards the ceiling.",
				),
			],
			Some(Step::new(
				"assets/ex07_extra.png",
				"Push your hands up and closer to the ceiling using your shoulders and chest. Stop when your hands are as close to the ceiling as possible.",
			)),
			vec![
				Step::new(
					"assets/ex07_motion.png",
					"1. Relax your shoulders.",
				),
				Step::new(
					"assets/ex07_start.png",
					"2. Move your arms move back to the floor. When your hands reach the floors, uncurl your toes and breathe out.",
				),
			],
			"Train Exercise 7 six times, then move on to Exercise 8.",
			"1-lb x 2",
		),
		Exercise::new(
			"Exercise 8",
			"Abdomen",
			Step::new(
				"assets/ex08_start.png",
				"Lie on your back with your legs and the bottoms of you feet pointing up towards the ceiling. Place both arms on the floor above your head.",
			),
			"Take a deep breath, straighten your knee, and tighten the muscles in your legs. Curl your toes toward your face.",
			vec![
				Step::new(
					"assets/ex08_motion.png",
					"Move your left arm near the floor on the your left side while reaching your right hand towards the toes of your left foot. Stop when your right hand reaches your toes.",
				),
			],
			Some(Step::new(
				"assets/ex08_extra.png",
				"Push your right hand further up towards the ceiling, trying to reach beyond the toes of your left foot.",
			)),
			vec![
				Step::new(
					"assets/ex08_motion.png",
					"Relax your stretch.",
				),
				Step::new(
					"assets/ex08_start.png",
					"Move both hands back to the floor about your head.  When your hands reach the floor, uncurl your toes and breathe out.",
				),
			],
			"Train Exercise 8 six times, then move on to Exercise 9.",
			"",
		),
		Exercise::new(
			"Exercise 9",
			"Back",
			Step::new(
				"assets/ex09_start.png",
				"Lie on your stomach with your palms touching the floor near your ears. Turn your head to look towards your right hand (left cheek on the floor).",
			),
			"Take a deep breath and tighten the muscles in your legs while keeping your knees straight. Point your toes away from your face.",
			vec![
				Step::new(
					"assets/ex09_motion.png",
					"Turn your face to look straight ahead and raise your head and chest off the floor. Stop when you are looking at the wall in front of you.",
				),
			],
			Some(Step::new(
				"assets/ex09_extra.png",
				"Tighten your back and tilt your head even further. Stop when you are looking at the spot where the wall in front of you meets the ceiling.",
			)),
			vec![
				Step::new(
					"assets/ex09_motion.png",
					"Look back down at the wall in front of you.",
				),
				Step::new(
					"assets/ex09_end.png",
					"Relax your back and move your head back to the floor, this time looking towards your left hand (right cheek on the floor). When your cheek is back on the floor, relax your legs and breathe out.",
				),
			],
			"Train Exercise 9 six times, then move on to Exercise 10.",
			"",
		),
		Exercise::new(
			"Exercise 10",
			"Arms",
			Step::new(
				"assets/ex10_start.png",
				"Place one 8-lb weight on the ground and stand over it with hips and both knees bent. If you let your arms down, your hands should touch the weight, but start with both hands resting on your knees.",
			),
			"Take a deep breath and tighten the muscles in your legs while keeping your knees bent. Curl your toes toward your face.",
			vec![
				Step::new(
					"assets/ex10_motion_a.png",
					"Move your right hand down (leave the left hand on your leg) and grab the weight with your right hand. Bring the weight straight up to your chest using your right arm. Stop when your right elbow is fully bent and the weight is near your chest.",
				),
				Step::new(
					"assets/ex10_motion_b.png",
					"View from above.",
				),
			],
			Some(Step::new(
				"assets/ex10_extra.png",
				"Turn your entire upper body to your right and look at the wall on your right. The weight should move upwards as your torso rotates. Stop when your torso is facing the right wall,",
			)),
			vec![
				Step::new(
					"assets/ex10_motion_b.png",
					"Rotate your arm and torso back to the center position.",
				),
				Step::new(
					"assets/ex10_motion_a.png",
					"View from the side.",
				),
				Step::new(
					"assets/ex10_start.png",
					"Relax your arm and move the weight back to the floor in front of you. When the weight is on the floor, uncurl your toes and breathe out. Switch to your left arm for the next iteration.",
				),
			],
			"Train Exercise 10 six times (3 times right arm, 3 times left arm), then move on to Exercise 11.",
			"8-lb x 1",
		),
		Exercise::new(
			"Exercise 11",
			"Legs",
			Step::new(
				"assets/ex11_start.png",
				"Pick up two 3-lb weights, one in each hand. Stand with legs shoulder-width apart and weights at your side by your legs.",
			),
			"Take a deep breath and tighten the muscles in your legs while keeping your legs straight. Curl your toes upward.",
			vec![
				Step::new(
					"assets/ex11_motion_a.png",
					"1. Bend your elbows and bring the weights forward in front of you eyes.",
				),
				Step::new(
					"assets/ex11_motion_b.png",
					"2. Then rotate your arms outwards and stop when the weights are by your ears.",
				),
				Step::new(
					"assets/ex11_motion_c.png",
					"3. Push the weights straight up towards the ceiling and stop when your elbows are straight.",
				),
				Step::new(
					"assets/ex11_motion_d.png",
					"4. Squat down by bending your knees and pushing your butt backwards. Keep your arms straight and above your head. Stop when your knees are fully bent.",
				),
				Step::new(
					"assets/ex11_motion_e.png",
					"5. Stand back up while keeping your arms straight and above your head.",
				),
			],
			None,
			vec![
				Step::new(
					"assets/ex11_motion_b.png",
					"1. Bend your elbows and bring the weights down and stop when they are near your ears.",
				),
				Step::new(
					"assets/ex11_motion_a.png",
					"2. Rotate your arms towards each other and stop when your hands and the weights are directly in front of your eyes.",
				),
				Step::new(
					"assets/ex11_start.png",
					"3. Drop your hands and until the weights are at your side by your legs. 4. Uncurl your toes and breathe out.",
				),
			],
			"Train Exercise 11 six times, then move on to Exercise 12.",
			"3-lb x 2",
		),
		Exercise::new(
			"Exercise 12",
			"Lower Back",
			Step::new(
				"assets/ex12_start.png",
				"Pick up two 5-lb weights, one in each hand. Stand with legs should-width apart and weights by your legs slightly in front of you.",
			),
			"Take a deep breath and tighten the muscles in your legs while keeping your legs straight.  Curl your toes upward.",
			vec![
				Step::new(
					"assets/ex12_motion_a.png",
					"1. Keeping your arms and legs straight and hanging, bend at the hip and allow the weights to descend toward the floor.",
				),
				Step::new(
					"assets/ex12_motion_b.png",
					"2. When the weights touch the floor or are as close to the floor as is comfortable, stand back up while still keeping both arms and legs straight and hanging.",
				),
			],
			Some(Step::new(
				"assets/ex12_extra.png",
				"1. Move the weights up by lifting your shoulders up (while keeping arms straight and hanging down). 2. Stand up on your toes.",
			)),
			vec![
				Step::new(
					"assets/ex11_start.png",
					"1. Stop standing on your toes and drop your shoulders. 2. Uncurl your toes and breathe out.",
				),
			],
			"Train Exercise 12 six times, then start again at Exercise 7 for 2 more rounds of Cycle B.",
			"5-lb x 2",
		),
	]);
	let exercises = &*world;
	let rows = exercises.iter().map(|exercise| {
		html! {
			<tr>
				<th>
					<p>{&exercise.name}</p>
					<p class="tag is-info is-light">{&exercise.target}</p>
				</th>
				<td>
					if {!exercise.equipment.is_empty()} {
						<p><span class="tag is-info is-medium">{&exercise.equipment}</span></p>
					}
					{exercise.start.to_html()}
				</td>
				<td>{&exercise.breath_in}</td>
				<td>
					{&exercise.primary}
				</td>
				<td>
					if {exercise.extra.is_none()} {
						{"None"}
					} else {
						{exercise.extra.clone().unwrap().to_html()}
					}
				</td>
				<td>{&exercise.breath_out}</td>
				<td>{&exercise.next}</td>
			</tr>
		}
	}).collect::<Vec<Html>>();
	html! {
	<>
	<section class="section">
	<div class="container">
		<p class="title">{"Cycle B Exercises"}</p>
		<div class="table-container">
			<style>{"table, td, th {width: 12.5%}"}</style>
			<table class="table is-bordered is-striped is-fullwidth">
				<thead>
					<tr>
						<th>{"Name"}</th>
						<th>{"Starting Position"}</th>
						<th>{"Breathe In"}</th>
						<th>{"Movement"}</th>
						<th>{"Extra Movement"}</th>
						<th>{"Breathe Out"}</th>
						<th>{"Next"}</th>
					</tr>
				</thead>
				<tbody>{rows}</tbody>
			</table>
		</div>
	</div>
	</section>
	</>
	}
}


#[derive(Debug, Clone)]
struct Step {
	pub caption: String,
	pub image_url: String,
}

impl Step {
	pub fn new(image_url: &str, caption: &str) -> Self {
		Self { caption: caption.to_string(), image_url: image_url.to_string() }
	}
}

impl ToHtml for Step {
	fn to_html(&self) -> Html {
		html! {
			<>
			<figure class="image is-square">
				<img src={self.image_url.to_string()} />
			</figure>
			<p>{&self.caption}</p>
			</>
		}
	}
}

#[derive(Debug, Clone)]
struct Exercise {
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
