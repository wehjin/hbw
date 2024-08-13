use yew::prelude::*;
use yew_router::prelude::*;

use step::Step;

use crate::app::exercise::Exercise;
use crate::app::sequence::{Cycle, import_cycle};

#[function_component]
pub fn App() -> Html {
	html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[derive(Clone, Routable, PartialEq)]
enum Route {
	#[at("/")]
	Home,
	#[at("/c")]
	C,
}

fn switch(routes: Route) -> Html {
	let cycle = match routes {
		Route::Home => Cycle::B,
		Route::C => Cycle::C,
	};
	sequence(cycle)
}

fn sequence(cycle: Cycle) -> Html {
	let sequence = import_cycle(cycle).unwrap();
	let title = format!("Cycle {}", sequence.name.to_uppercase());
	let subtitle = format!("Ex {}-{}", sequence.start_num, sequence.end_num);
	let hero_color = match cycle {
		Cycle::B => "is-primary",
		Cycle::C => "is-info",
	};
	let hero_class = format!("hero is-small {}", hero_color);
	let cards = sequence.exercises.iter().map(exercise).collect::<Vec<Html>>();
	let (b_class, c_class) = match cycle {
		Cycle::B => ("is-active", ""),
		Cycle::C => ("", "is-active"),
	};
	html! {
		<section class="section">
		<div class="tabs">
		  <ul>
		    <li class={b_class}><Link<Route> to={Route::Home}>{"Cycle B"}</Link<Route>></li>
		    <li class={c_class}><Link<Route> to={Route::C}>{"Cycle C"}</Link<Route>></li>
		  </ul>
		</div>
		<div class="container">
			<div class={hero_class}>
			  <div class="hero-body">
			    <p class="title">{title}</p>
			    <p class="subtitle">{subtitle}</p>
			  </div>
			</div>
			<br/>
			{cards}
		</div>
		</section>
	}
}

fn exercise(exercise: &Exercise) -> Html {
	let html: Html = html! {
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
		};
	html
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
						<img style="border: 1px solid #555;" src={self.image_url.to_string()} width=128 height=128 />
					</p>
				</figure>
				<div class="media-content">
					<p>{&self.caption}</p>
				</div>
			</div>
		}
	}
}