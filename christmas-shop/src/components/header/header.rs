use leptos::prelude::*;

#[component]
pub fn Header() -> impl IntoView {
    view! {
        <header class="header">
            <div class="container">
                <img src="/assets/icons/snowflake.svg" alt="logo" />
                <div class="burger-container">
                    <div class="burger-menu"></div>
                </div>
            </div>
        </header>
    }
}
