use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="header">
            <div class="container">
                <div class="logo">
                    <img src="/assets/icons/snowflake.svg" alt="logo" />
                    <p class="text">the gifts</p>
                </div>
                <div class="burger-container">
                    <div class="burger-menu"></div>
                </div>
            </div>
        </header>
    }
}
