use leptos::prelude::*;
use leptos::task::spawn_local;
use leptos_router::hooks::use_params_map;
use lucide_leptos::*;
use crate::components::blog_renderer::BlogPostViewer;
use crate::api::fetch_blog_post_by_slug;
use shared::BlogPost;

stylance::import_style!(style, "blog_post.module.scss");

#[component]
pub fn BlogPostPage() -> impl IntoView {
    let params = use_params_map();
    let slug = RwSignal::new(params.get().get("slug").map(|s| s.clone()).unwrap_or_default());

    let blog_post = RwSignal::new(Option::<BlogPost>::None);
    let loading = RwSignal::new(true);
    let error = RwSignal::new(Option::<String>::None);

    // Fetch blog post metadata when component loads
    Effect::new(move |_| {
        let slug_value = slug.get();
        spawn_local(async move {
            loading.set(true);
            error.set(None);

            match fetch_blog_post_by_slug(&slug_value).await {
                Ok(Some(post)) => {
                    blog_post.set(Some(post));
                    loading.set(false);
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
        <div class={style::page_container}>
            {move || {
                if loading.get() {
                    view! {
                        <div class={style::loading_container}>
                            <div class={style::loading_spinner}></div>
                            <p>"Loading blog post..."</p>
                        </div>
                    }.into_any()
                } else if let Some(error_msg) = error.get() {
                    view! {
                        <div class={style::error_container}>
                            <X size=48 />
                            <h1>"Error"</h1>
                            <p>{error_msg}</p>
                            <a href="/blog" class={style::back_button}>
                                <ArrowLeft size=16 />
                                "Back to Blog"
                            </a>
                        </div>
                    }.into_any()
                } else if let Some(post) = blog_post.get() {
                    view! {
                        <div class={style::blog_post_container}>
                            // Blog post header
                            <header class={style::post_header}>
                                <div class={style::breadcrumb}>
                                    <a href="/blog" class={style::breadcrumb_link}>
                                        <ArrowLeft size=16 />
                                        "Blog"
                                    </a>
                                    <span class={style::breadcrumb_separator}>"/"</span>
                                    <span class={style::breadcrumb_current}>{post.title.clone()}</span>
                                </div>

                                <div class={style::post_badges}>
                                    <span class={format!("{} {}", style::post_type_badge,
                                        match post.post_type.to_string().to_lowercase().replace(" ", "-").replace("&", "and").as_str() {
                                            "tutorial" => style::type_tutorial,
                                            "guide" => style::type_guide,
                                            "show-and-tell" => style::type_show_and_tell,
                                            "tech-talk" => style::type_tech_talk,
                                            "news" => style::type_news,
                                            "review" => style::type_review,
                                            _ => style::type_tutorial,
                                        })}>
                                        {post.post_type.to_string()}
                                    </span>

                                    {post.difficulty_level.map(|level| {
                                        view! {
                                            <span class={format!("{} {}", style::difficulty_badge, match level {
                                                shared::DifficultyLevel::Easy => style::difficulty_easy,
                                                shared::DifficultyLevel::Medium => style::difficulty_medium,
                                                shared::DifficultyLevel::Hard => style::difficulty_hard,
                                            })}>
                                                {match level {
                                                    shared::DifficultyLevel::Easy => "Beginner",
                                                    shared::DifficultyLevel::Medium => "Intermediate",
                                                    shared::DifficultyLevel::Hard => "Advanced",
                                                }}
                                            </span>
                                        }
                                    })}
                                </div>

                                <h1 class={style::post_title}>{post.title.clone()}</h1>

                                <div class={style::post_meta}>
                                    <div class={style::author_info}>
                                        <User size=16 />
                                        <span class={style::author_name}>{post.author_name.clone()}</span>
                                        {post.author_github.as_ref().map(|github| {
                                            view! {
                                                <a href={format!("https://github.com/{}", github)}
                                                   target="_blank"
                                                   class={style::github_link}>
                                                    <Github size=14 />
                                                    {format!("@{}", github)}
                                                </a>
                                            }
                                        })}
                                    </div>
                                    <div class={style::post_stats}>
                                        <span class={style::read_time}>
                                            <Clock size=14 />
                                            {format!("{} min read", post.estimated_read_time)}
                                        </span>
                                        <span class={style::publish_date}>
                                            <Calendar size=14 />
                                            {post.published_at.clone()}
                                        </span>
                                        <span class={style::view_count}>
                                            <Eye size=14 />
                                            {format!("{} views", post.views)}
                                        </span>
                                        <span class={style::like_count}>
                                            <Heart size=14 />
                                            {format!("{} likes", post.likes)}
                                        </span>
                                    </div>
                                </div>

                                {post.series.as_ref().map(|series| {
                                    view! {
                                        <div class={style::series_info}>
                                            <BookOpen size=16 />
                                            <span>
                                                "Part " {series.part.to_string()}
                                                {series.total_parts.map(|total| format!(" of {}", total)).unwrap_or_default()}
                                                " in " {series.title.clone()}
                                            </span>
                                        </div>
                                    }
                                })}

                                <div class={style::excerpt}>
                                    <p>{post.excerpt.clone()}</p>
                                </div>

                                <div class={style::tags}>
                                    {post.tags.into_iter().map(|tag| {
                                        view! {
                                            <span class={style::tag}>{"#"}{tag}</span>
                                        }
                                    }).collect_view()}
                                </div>
                            </header>

                            // Blog post content
                            <main class={style::post_content}>
                                <BlogPostViewer slug={slug.get()} />
                            </main>

                            // External links
                            {if !post.external_links.is_empty() {
                                view! {
                                    <aside class={style::external_links}>
                                        <h3>"Related Links"</h3>
                                        <ul>
                                            {post.external_links.into_iter().map(|link| {
                                                view! {
                                                    <li>
                                                        <a href={link.url} target="_blank" class={style::external_link}>
                                                            <ExternalLink size=14 />
                                                            {link.title}
                                                        </a>
                                                    </li>
                                                }
                                            }).collect_view()}
                                        </ul>
                                    </aside>
                                }.into_any()
                            } else {
                                view! { <div></div> }.into_any()
                            }}

                            // Navigation
                            <nav class={style::post_navigation}>
                                <a href="/blog" class={style::back_to_blog}>
                                    <ArrowLeft size=16 />
                                    "Back to All Posts"
                                </a>
                            </nav>
                        </div>
                    }.into_any()
                } else {
                    view! {
                        <div class={style::error_container}>
                            <X size=48 />
                            <h1>"Blog post not found"</h1>
                            <p>"The blog post you're looking for doesn't exist."</p>
                            <a href="/blog" class={style::back_button}>
                                <ArrowLeft size=16 />
                                "Back to Blog"
                            </a>
                        </div>
                    }.into_any()
                }
            }}
        </div>
    }
}