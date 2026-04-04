use leptos::prelude::*;

pub mod components;
pub mod pages;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <main>
            <p>"Hello world!"</p>
        </main>
    }
}
