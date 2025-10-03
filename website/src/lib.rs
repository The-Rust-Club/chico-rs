use leptos::prelude::*;
use leptos_meta::*;
use leptos_router::{components::*, path};

// Modules
mod components;
mod pages;
mod api;

// Components
use crate::components::nav::Nav;

// Top-Level pages
use crate::pages::home::Home;
use crate::pages::about::About;
use crate::pages::blog::Blog;
use crate::pages::blog_post::BlogPostPage;
use crate::pages::opensource::OpenSource;
use crate::pages::workshops::Workshops;
use crate::pages::events::Events;
use crate::pages::projects::Projects;
use crate::pages::advocacy::Advocacy;
use crate::pages::not_found::NotFound;

/// An app router which renders the homepage and handles 404's
#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();

    view! {
        <Html attr:lang="en" attr:dir="ltr" attr:data-theme="light" />

        // sets the document title
        <Title text="The Rust Club - Learn, Build, Contribute" />

        // injects metadata in the <head> of the page
        <Meta charset="UTF-8" />
        <Meta name="viewport" content="width=device-width, initial-scale=1.0" />

        <Router>
            <Nav />
            <main class="main-content">
                <Routes fallback=|| view! { <NotFound /> }>
                    <Route path=path!("/") view=Home />
                    <Route path=path!("/about") view=About />
                    <Route path=path!("/blog") view=Blog />
                    <Route path=path!("/blog/:slug") view=BlogPostPage />
                    <Route path=path!("/opensource") view=OpenSource />
                    <Route path=path!("/workshops") view=Workshops />
                    <Route path=path!("/events") view=Events />
                    <Route path=path!("/projects") view=Projects />
                    <Route path=path!("/advocacy") view=Advocacy />
                </Routes>
            </main>
        </Router>
    }
}
