use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
	let world = use_state(|| "World".to_string());
	html! {
		<>
		<section class="section">
			<div class="container">
			<h1 class="title">
				{format!("Hello {}", *world)}
			</h1>
				<p class="subtitle">
				{"My first website with "}<strong>{"Bulma"}</strong>{"!"}
				</p>
			</div>
		</section>
		</>
    }
}
