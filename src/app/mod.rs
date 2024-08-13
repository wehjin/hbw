use yew::prelude::*;

use step::Step;

use crate::app::sequence::import_sequence_b;

#[function_component]
pub fn App() -> Html {
	let sequence = import_sequence_b().unwrap();
	let world = use_state(|| sequence);
	let cards = world.exercises.iter().map(|exercise| {
		html! {
			<div class="card" style="break-after:page">
			<div class="card-content">
			    <div class="media">
					<div class="media-content">
						<p class="title">{&exercise.name}</p>
					</div>
					<div class="media-right">
						<p class="tag is-info is-light">{&exercise.target}</p>
					</div>
			    </div>
			    <div class="content">
					<div>
						<p class="tag is-primary">{"Starting Position"}</p>
						if {!exercise.equipment.is_empty()} {
							<p><span class="tag is-info is-medium">{&exercise.equipment}</span></p>
						}
						<div class="pl-6">
							{exercise.start.to_html()}
						</div>
					</div>
					<div>
						<p class="tag is-primary">{"Breathe In"}</p>
						<div class="pl-6 media">
							<figure class="media-left">
								<p class="image is-128x128">
								</p>
							</figure>
							<div class="media-content">
								<p>{&exercise.breath_in}</p>
							</div>
						</div>
					</div>
					<br/>
					<div>
						<p class="tag is-primary">{"Primary Movement"}</p>
						<div class="pl-6">{&exercise.primary}</div>
					</div>
					<br/>
					<div>
						<p class="tag is-primary">{"Extra Movement"}</p>
						if {exercise.extra.is_none()} {
							<div class="pl-6 media">
								<figure class="media-left">
									<p class="image is-128x128">
									</p>
								</figure>
								<div class="media-content">
									<p>{"None"}</p>
								</div>
							</div>
						} else {
							<div class="pl-6">
									{exercise.extra.clone().unwrap().to_html()}
							</div>
						}
					</div>
					<br/>
					<div>
						<p class="tag is-primary">{"Breathe Out"}</p>
						<div class="pl-6">
							{&exercise.breath_out}
						</div>
					</div>
					<br/>
					<div>
						<p class="tag is-primary">{"Next"}</p>
						<div class="pl-6 media">
							<figure class="media-left">
								<p class="image is-128x128">
								</p>
							</figure>
							<div class="media-content">
								<p>{&exercise.next}</p>
							</div>
						</div>
					</div>
				</div>
			</div>
			</div>
		}
	}).collect::<Vec<Html>>();
	html! {
		<section class="section">
		<div class="container">
			<div class="hero is-small is-primary">
			  <div class="hero-body">
			    <p class="title">{"Cycle B"}</p>
			    <p class="subtitle">{"Exercises 7-12"}</p>
			  </div>
			</div>
			<br/>
			{cards}
		</div>
		</section>
	}
}

pub mod step;
pub mod exercise;
pub mod sequence;

impl ToHtml for Step {
	fn to_html(&self) -> Html {
		html! {
			<div class="media">
				<figure class="media-left">
					<p class="image is-128x128 is-square">
						<img src={self.image_url.to_string()} width=128 height=128 />
					</p>
				</figure>
				<div class="media-content">
					<p>{&self.caption}</p>
				</div>
			</div>
		}
	}
}