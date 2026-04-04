use leptos::prelude::*;

pub mod components;
pub mod pages;

use components::header::Header;

#[component]
pub fn App() -> impl IntoView {
    view! {
        <>
            <Header />
            <main>
                <p>"Hello world!"</p>
            </main>
        </>
    }
}
