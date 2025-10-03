use leptos::prelude::*;
use crate::api::{BlogPost, fetch_markdown_content};

#[component]
pub fn BlogPostRenderer(blog_post: BlogPost) -> impl IntoView {
    let (markdown_content, set_markdown_content) = signal(String::new());
    let (is_loading, set_is_loading) = signal(true);
    let (error, set_error) = signal(Option::<String>::None);

    // Fetch markdown content when component mounts
    Effect::new(move |_| {
        if let Some(markdown_url) = &blog_post.markdown_url {
            let url = markdown_url.clone();
            spawn_local(async move {
                set_is_loading.set(true);
                match fetch_markdown_content(&url).await {
                    Ok(content) => {
                        set_markdown_content.set(content);
                        set_error.set(None);
                    }
                    Err(e) => {
                        set_error.set(Some(format!("Failed to load content: {}", e)));
                    }
                }
                set_is_loading.set(false);
            });
        } else {
            // Use the content field if no markdown_url
            set_markdown_content.set(blog_post.content.clone());
            set_is_loading.set(false);
        }
    });

    view! {
        <article class="blog-post-renderer">
            <Show when=move || is_loading.get() fallback=|| view! {}>
                <div class="loading-spinner">
                    <p>"Loading content..."</p>
                </div>
            </Show>

            <Show when=move || error.get().is_some() fallback=|| view! {}>
                <div class="error-message">
                    <p>{move || error.get().unwrap_or_default()}</p>
                </div>
            </Show>

            <Show when=move || !is_loading.get() && error.get().is_none() fallback=|| view! {}>
                <div class="markdown-content" inner_html=move || {
                    let content = markdown_content.get();
                    if content.is_empty() {
                        "No content available.".to_string()
                    } else {
                        markdown::to_html(&content)
                    }
                }></div>
            </Show>
        </article>
    }
}