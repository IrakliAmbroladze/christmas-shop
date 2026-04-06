use leptos::prelude::*;

#[component]
pub fn Navigation() -> impl IntoView {
    view! {
        <nav class="navigation">
            <ul class="nav-items">
                <li class="nav-item">gifts</li>
                <li class="nav-item">about</li>
                <li class="nav-item">best</li>
                <li class="nav-item">contacts</li>
            </ul>
            <div class="burger-container">
                <div class="burger-menu"></div>
            </div>
        </nav>
    }
}
