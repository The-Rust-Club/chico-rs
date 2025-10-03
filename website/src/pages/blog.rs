use leptos::prelude::*;
use leptos::task::spawn_local;
use lucide_leptos::*;
use crate::api::client::{fetch_blog_posts, fetch_featured_blog_posts};
use shared::DifficultyLevel;

stylance::import_style!(style, "blog.module.scss");

#[component]
pub fn Blog() -> impl IntoView {
    let (blog_posts, set_blog_posts) = RwSignal::new(Vec::new()).split();
    let (featured_posts, set_featured_posts) = RwSignal::new(Vec::new()).split();
    let (loading, set_loading) = RwSignal::new(true).split();
    let (error, set_error) = RwSignal::new(None::<String>).split();

    Effect::new(move |_| {
        spawn_local(async move {
            set_loading.set(true);
            set_error.set(None);

            // Fetch both resources concurrently
            let blog_result = fetch_blog_posts().await;
            let featured_result = fetch_featured_blog_posts().await;

            match blog_result {
                Ok(posts) => set_blog_posts.set(posts),
                Err(e) => set_error.set(Some(format!("Failed to load blog posts: {}", e))),
            }

            match featured_result {
                Ok(posts) => set_featured_posts.set(posts),
                Err(e) => set_error.set(Some(format!("Failed to load featured posts: {}", e))),
            }

            set_loading.set(false);
        });
    });

    view! {
        <div class={style::page_container}>
            <div class={style::page_header}>
                <h1>"Blog & Tutorials"</h1>
                <p class={style::page_subtitle}>
                    "In-depth tutorials, guides, and member showcases. Learn from the community "
                    "and share your own Rust journey with fellow students."
                </p>
            </div>

            // Featured Posts Section
            <section class={style::featured_posts}>
                <h2>"Featured Posts"</h2>
                <div class={style::featured_posts_grid}>
                    {move || {
                        if loading.get() {
                            view! { <div>"Loading featured posts..."</div> }.into_any()
                        } else if let Some(err) = error.get() {
                            view! { <div class="error">"Error loading featured posts: " {err}</div> }.into_any()
                        } else {
                            view! {
                                {featured_posts.get().into_iter().map(|post| {
                                                let difficulty_badge = if let Some(level) = &post.difficulty_level {
                                                    view! {
                                                        <span class={format!("{} {}", style::difficulty_badge, match level {
                                                                DifficultyLevel::Easy => style::difficulty_easy,
                                                                DifficultyLevel::Medium => style::difficulty_medium,
                                                                DifficultyLevel::Hard => style::difficulty_hard,
                                                            })}>
                                                            {match level {
                                                                DifficultyLevel::Easy => "Beginner",
                                                                DifficultyLevel::Medium => "Intermediate",
                                                                DifficultyLevel::Hard => "Advanced",
                                                            }}
                                                        </span>
                                                    }.into_any()
                                                } else {
                                                    view! { <span></span> }.into_any()
                                                };

                                                view! {
                                                    <article class={style::featured_post_card}>
                                                        <div class={style::post_header}>
                                                            <div class={style::post_badges}>
                                                                <span class={format!("{} {}", style::post_type_badge, match post.post_type.to_string().to_lowercase().replace(" ", "-").replace("&", "and").as_str() {
                                                                    "tutorial" => style::type_tutorial,
                                                                    "guide" => style::type_guide,
                                                                    "show-and-tell" => style::type_show_and_tell,
                                                                    "tech-talk" => style::type_tech_talk,
                                                                    "news" => style::type_news,
                                                                    _ => style::type_tutorial,
                                                                })}>
                                                                    {post.post_type.to_string()}
                                                                </span>
                                                                {difficulty_badge}
                                                            </div>
                                                            <div class={style::post_meta}>
                                                                <span class={style::read_time}>
                                                                    <Clock size=14 />
                                                                    {format!("{} min read", post.estimated_read_time)}
                                                                </span>
                                                            </div>
                                                        </div>
                                                        <h3 class={style::post_title}>{post.title.clone()}</h3>
                                                        <p class={style::post_excerpt}>{post.excerpt}</p>
                                                        <div class={style::post_author}>
                                                            <div class={style::author_info}>
                                                                <span class={style::author_name}>{post.author_name.clone()}</span>
                                                                <span class={style::publish_date}>{post.published_at.clone()}</span>
                                                            </div>
                                                        </div>
                                                        <div class={style::post_tags}>
                                                            {post.tags.into_iter().take(3).map(|tag| {
                                                                view! { <span class={style::tag}>{"#"}{tag}</span> }
                                                            }).collect_view()}
                                                        </div>
                                                        <a href={format!("/blog/{}", post.slug)} class={format!("{} {} {}", style::btn, style::btn_secondary, style::btn_sm)}>
                                                            "Read More"
                                                        </a>
                                                    </article>
                                                }
                                            }).collect_view()}
                                        }.into_any()
                                        }
                                    }}
                </div>
            </section>

            // Filter Section
            <section class={style::blog_filters}>
                <h2>"All Posts"</h2>
                <div class={style::filter_controls}>
                    <div class={style::filter_group}>
                        <label>"Filter by Type:"</label>
                        <div class={style::filter_buttons}>
                            <button class={format!("{} {}", style::filter_btn, style::active)}>"All"</button>
                            <button class={style::filter_btn}>"Tutorials"</button>
                            <button class={style::filter_btn}>"Guides"</button>
                            <button class={style::filter_btn}>"Show & Tell"</button>
                            <button class={style::filter_btn}>"Tech Talks"</button>
                            <button class={style::filter_btn}>"News"</button>
                        </div>
                    </div>
                    <div class={style::filter_group}>
                        <label>"Filter by Category:"</label>
                        <div class={style::filter_buttons}>
                            <button class={format!("{} {}", style::filter_btn, style::active)}>"All"</button>
                            <button class={style::filter_btn}>"Fundamentals"</button>
                            <button class={style::filter_btn}>"Web Dev"</button>
                            <button class={style::filter_btn}>"CLI"</button>
                            <button class={style::filter_btn}>"Systems"</button>
                        </div>
                    </div>
                    <div class={style::filter_group}>
                        <label>"Filter by Level:"</label>
                        <div class={style::filter_buttons}>
                            <button class={format!("{} {}", style::filter_btn, style::active)}>"All"</button>
                            <button class={style::filter_btn}>"Beginner"</button>
                            <button class={style::filter_btn}>"Intermediate"</button>
                            <button class={style::filter_btn}>"Advanced"</button>
                        </div>
                    </div>
                </div>
            </section>

            // All Posts Section
            <section class={style::all_posts}>
                <div class={style::posts_grid}>
                    {move || {
                        if loading.get() {
                            view! { <div>"Loading blog posts..."</div> }.into_any()
                        } else if let Some(err) = error.get() {
                            view! { <div class="error">"Error loading blog posts: " {err}</div> }.into_any()
                        } else {
                            view! {
                                {blog_posts.get().into_iter().map(|post| {
                                                let difficulty_badge = if let Some(level) = &post.difficulty_level {
                                                    view! {
                                                        <span class={format!("{} {}", style::difficulty_badge, match level {
                                                                DifficultyLevel::Easy => style::difficulty_easy,
                                                                DifficultyLevel::Medium => style::difficulty_medium,
                                                                DifficultyLevel::Hard => style::difficulty_hard,
                                                            })}>
                                                            {match level {
                                                                DifficultyLevel::Easy => "Beginner",
                                                                DifficultyLevel::Medium => "Intermediate",
                                                                DifficultyLevel::Hard => "Advanced",
                                                            }}
                                                        </span>
                                                    }.into_any()
                                                } else {
                                                    view! { <span></span> }.into_any()
                                                };

                                                let series_info = if let Some(series) = &post.series {
                                                    view! {
                                                        <div class={style::series_info}>
                                                            <BookOpen size=14 />
                                                            <span>{format!("{} (Part {} of {})",
                                                                series.title,
                                                                series.part,
                                                                series.total_parts.unwrap_or(series.part)
                                                            )}</span>
                                                        </div>
                                                    }.into_any()
                                                } else {
                                                    view! { <div></div> }.into_any()
                                                };

                                                view! {
                                                    <article class={style::post_card}>
                                                        <div class={style::post_header}>
                                                            <div class={style::post_badges}>
                                                                <span class={format!("{} {}", style::post_type_badge, match post.post_type.to_string().to_lowercase().replace(" ", "-").replace("&", "and").as_str() {
                                                                    "tutorial" => style::type_tutorial,
                                                                    "guide" => style::type_guide,
                                                                    "show-and-tell" => style::type_show_and_tell,
                                                                    "tech-talk" => style::type_tech_talk,
                                                                    "news" => style::type_news,
                                                                    _ => style::type_tutorial,
                                                                })}>
                                                                    {post.post_type.to_string()}
                                                                </span>
                                                                {difficulty_badge}
                                                            </div>
                                                            <div class={style::post_meta}>
                                                                <span class={style::read_time}>
                                                                    <Clock size=14 />
                                                                    {format!("{} min read", post.estimated_read_time)}
                                                                </span>
                                                            </div>
                                                        </div>
                                                        <h3 class={style::post_title}>{post.title.clone()}</h3>
                                                        {series_info}
                                                        <p class={style::post_excerpt}>{post.excerpt}</p>
                                                        <div class={style::post_author}>
                                                            <div class={style::author_info}>
                                                                <User size=14 />
                                                                <span class={style::author_name}>{post.author_name.clone()}</span>
                                                                <span class={style::publish_date}>{post.published_at.clone()}</span>
                                                            </div>
                                                        </div>
                                                        <div class={style::post_tags}>
                                                            {post.tags.into_iter().take(3).map(|tag| {
                                                                view! { <span class={style::tag}>{"#"}{tag}</span> }
                                                            }).collect_view()}
                                                        </div>
                                                        <a href={format!("/blog/{}", post.slug)} class={format!("{} {} {}", style::btn, style::btn_secondary, style::btn_sm)}>
                                                            "Read More"
                                                        </a>
                                                    </article>
                                                }
                                            }).collect_view()}
                                        }.into_any()
                                        }
                                    }}
                </div>
            </section>

            // Contributing Section
            <section class={style::blog_cta}>
                <h2>"Share Your Knowledge"</h2>
                <p>"Have you built something cool with Rust? Learned a new concept? Share it with the community!"</p>
                <div class={style::cta_buttons}>
                    <button class={format!("{} {}", style::btn, style::btn_primary)}>"Submit a Post"</button>
                    <button class={format!("{} {}", style::btn, style::btn_secondary)}>"Writing Guidelines"</button>
                </div>
            </section>
        </div>
    }
}
