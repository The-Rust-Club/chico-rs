use leptos::prelude::*;
use leptos_router::components::A;
use lucide_leptos::*;

#[component]
pub fn NotFound() -> impl IntoView {
    view! {
        <div class="not-found-container">
            <div class="not-found-content">
                <h1 class="not-found-title">
                    <span class="error-code">"404"</span>
                    <Bug class:error-icon=true />
                </h1>
                <h2>"Page Not Found"</h2>
                <p>"Looks like this page has gone off to learn Rust and hasn't come back yet!"</p>
                <A href="/" class:btn=true class:btn-primary=true>"Go Home"</A>
            </div>
        </div>
    }
}