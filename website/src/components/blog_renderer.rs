use leptos::prelude::*;
use leptos::task::spawn_local;
use markdown;

stylance::import_style!(style, "blog_renderer.module.scss");

#[component]
pub fn BlogRenderer(
    /// Markdown content to render
    markdown_content: String,
    #[prop(optional)] class: Option<String>,
) -> impl IntoView {
    let html_content = markdown::to_html(&markdown_content);

    view! {
        <div class={format!("{} {}", style::blog_content, class.unwrap_or_default())}
             inner_html=html_content>
        </div>
    }
}

#[component]
pub fn BlogPostViewer(
    /// Blog post slug for fetching content
    slug: String,
) -> impl IntoView {
    let markdown_content = RwSignal::new(String::new());
    let loading = RwSignal::new(true);
    let error = RwSignal::new(Option::<String>::None);

    // Fetch markdown content when component loads
    Effect::new(move |_| {
        let slug = slug.clone();
        spawn_local(async move {
            loading.set(true);
            error.set(None);

            // First fetch the blog post metadata to get the markdown URL
            match crate::api::fetch_blog_post_by_slug(&slug).await {
                Ok(Some(blog_post)) => {
                    // Then fetch the markdown content from the URL
                    match crate::api::fetch_markdown_content(&blog_post.markdown_url).await {
                        Ok(content) => {
                            markdown_content.set(content);
                            loading.set(false);
                        }
                        Err(e) => {
                            error.set(Some(format!("Failed to load markdown content: {}", e)));
                            loading.set(false);
                        }
                    }
                }
                Ok(None) => {
                    error.set(Some("Blog post not found".to_string()));
                    loading.set(false);
                }
                Err(e) => {
                    error.set(Some(format!("Failed to load blog post: {}", e)));
                    loading.set(false);
                }
            }
        });
    });

    view! {
        <div class={style::blog_post_viewer}>
            {move || {
                if loading.get() {
                    view! {
                        <div class={style::loading}>
                            <div class={style::spinner}></div>
                            <p>"Loading blog post..."</p>
                        </div>
                    }.into_any()
                } else if let Some(error_msg) = error.get() {
                    view! {
                        <div class={style::error}>
                            <h2>"Error"</h2>
                            <p>{error_msg}</p>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <BlogRenderer markdown_content={markdown_content.get()} />
                    }.into_any()
                }
            }}
        </div>
    }
}