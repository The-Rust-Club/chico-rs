use leptos::prelude::*;
use lucide_leptos::*;

stylance::import_style!(style, "projects.module.scss");

#[derive(Clone)]
struct Project {
    name: String,
    description: String,
    status: String,
    tech_stack: Vec<String>,
    github_url: String,
    contributors_needed: bool,
    skills_needed: Vec<String>,
}

#[component]
pub fn Projects() -> impl IntoView {
    let projects = vec![
        Project {
            name: "The Rust Club Website".to_string(),
            description: "This very website you're looking at! Built with Leptos and Trunk. A practical project for learning modern Rust web development.".to_string(),
            status: "Active".to_string(),
            tech_stack: vec!["Leptos".to_string(), "Trunk".to_string(), "CSS".to_string()],
            github_url: "https://github.com/rust-club/website".to_string(),
            contributors_needed: true,
            skills_needed: vec!["Frontend".to_string(), "CSS".to_string(), "Design".to_string()],
        },
        Project {
            name: "Ferris Bot - Discord Helper".to_string(),
            description: "Our club's Discord bot for event reminders, Rust code snippets, and helping newcomers. Features include daily Rust tips and workshop notifications.".to_string(),
            status: "In Development".to_string(),
            tech_stack: vec!["Serenity".to_string(), "Tokio".to_string(), "SQLite".to_string()],
            github_url: "https://github.com/rust-club/ferris-bot".to_string(),
            contributors_needed: true,
            skills_needed: vec!["Discord API".to_string(), "Async Rust".to_string()],
        },
        Project {
            name: "Campus CLI - University Tools".to_string(),
            description: "Command-line tools for common campus tasks: checking grades, class schedules, dining hall menus, and library availability. Perfect for learning CLI development.".to_string(),
            status: "Planning".to_string(),
            tech_stack: vec!["Clap".to_string(), "Reqwest".to_string(), "Serde".to_string()],
            github_url: "https://github.com/rust-club/campus-cli".to_string(),
            contributors_needed: true,
            skills_needed: vec!["CLI tools".to_string(), "HTTP APIs".to_string(), "JSON parsing".to_string()],
        },
        Project {
            name: "Study Buddy - Peer Learning Platform".to_string(),
            description: "Web app for organizing study groups, sharing notes, and tracking learning progress. Built to support our weekly seminars and study sessions.".to_string(),
            status: "Beta".to_string(),
            tech_stack: vec!["Axum".to_string(), "SQLx".to_string(), "htmx".to_string()],
            github_url: "https://github.com/rust-club/study-buddy".to_string(),
            contributors_needed: false,
            skills_needed: vec![],
        },
        Project {
            name: "RustLearning - Interactive Tutorials".to_string(),
            description: "Interactive coding tutorials and exercises for learning Rust concepts. Complements our workshop materials with hands-on practice.".to_string(),
            status: "Active".to_string(),
            tech_stack: vec!["Yew".to_string(), "WASM".to_string(), "Monaco Editor".to_string()],
            github_url: "https://github.com/rust-club/rust-learning".to_string(),
            contributors_needed: true,
            skills_needed: vec!["WASM".to_string(), "Frontend".to_string(), "Education".to_string()],
        },
        Project {
            name: "Course Proposal Generator".to_string(),
            description: "Tool to help generate and format course proposals for accepting Rust in academic assignments. Supports our advocacy efforts!".to_string(),
            status: "Planning".to_string(),
            tech_stack: vec!["Leptos".to_string(), "PDF generation".to_string()],
            github_url: "https://github.com/rust-club/course-proposals".to_string(),
            contributors_needed: true,
            skills_needed: vec!["Document generation".to_string(), "Forms".to_string()],
        },
    ];

    view! {
        <div class={style::page_container}>
            <div class={style::page_header}>
                <h1>"Official Projects"</h1>
                <p class={style::page_subtitle}>
                    "Real-world projects built and maintained by The Rust Club. "
                    "Contribute to these projects to gain hands-on experience and make an impact."
                </p>
            </div>

            <div class={style::projects_showcase}>
                {projects.into_iter().map(|project| {
                    view! {
                        <div class={style::project_card}>
                            <div class={style::project_header}>
                                <h3 class={style::project_name}>{project.name}</h3>
                                <span class={format!("{} status-{}", style::status_badge, project.status.to_lowercase().replace(" ", "-"))}>
                                    {project.status.clone()}
                                </span>
                            </div>
                            <p class={style::project_description}>{project.description}</p>
                            <div class={style::tech_stack}>
                                <span class={style::stack_label}>"Tech Stack:"</span>
                                <div class={style::tech_tags}>
                                    {project.tech_stack.into_iter().map(|tech| {
                                        view! { <span class={style::tech_tag}>{tech}</span> }
                                    }).collect_view()}
                                </div>
                            </div>
                            <div class={style::contributors_section}>
                                {if project.contributors_needed {
                                    Some(view! {
                                        <div class={style::contributors_needed}>
                                            <span class={style::need_badge}>
                                                <Megaphone size=16 style:display="inline" style:margin-right="8px" />
                                                "Contributors Needed"
                                            </span>
                                            <div class={style::skills_needed}>
                                                {project.skills_needed.into_iter().map(|skill| {
                                                    view! { <span class={style::skill_tag}>{skill}</span> }
                                                }).collect_view()}
                                            </div>
                                        </div>
                                    })
                                } else {
                                    None
                                }}
                            </div>
                            <div class={style::project_actions}>
                                <a href={project.github_url} target="_blank" class={format!("{} {}", style::btn, style::btn_secondary)}>
                                    "View on GitHub"
                                </a>
                                <div class={style::action_buttons}>
                                    {if project.contributors_needed {
                                        Some(view! {
                                            <button class={format!("{} {}", style::btn, style::btn_primary)}>"Join Project"</button>
                                        })
                                    } else {
                                        None
                                    }}
                                </div>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>

            <section class={style::contribution_guide}>
                <h2>"How to Contribute"</h2>
                <div class={style::guide_steps}>
                    <div class={style::step_card}>
                        <div class={style::step_number}>"1"</div>
                        <h3>"Choose a Project"</h3>
                        <p>"Pick a project that matches your interests and skill level. Don't worry if you're new - we'll help you get started!"</p>
                    </div>
                    <div class={style::step_card}>
                        <div class={style::step_number}>"2"</div>
                        <h3>"Join the Team"</h3>
                        <p>"Click 'Join Project' or reach out on Discord. We'll add you to the project team and get you up to speed."</p>
                    </div>
                    <div class={style::step_card}>
                        <div class={style::step_number}>"3"</div>
                        <h3>"Pick an Issue"</h3>
                        <p>"Browse open issues labeled 'good first issue' or 'help wanted'. Comment on the issue to claim it."</p>
                    </div>
                    <div class={style::step_card}>
                        <div class={style::step_number}>"4"</div>
                        <h3>"Code & Review"</h3>
                        <p>"Write your code, submit a PR, and participate in code reviews. Learn from feedback and improve!"</p>
                    </div>
                </div>
            </section>

            <div class={style::project_ideas}>
                <h2>"Have a Project Idea?"</h2>
                <p>"We're always looking for new project ideas that can help our members learn and contribute to the Rust ecosystem."</p>
                <div class={style::idea_categories}>
                    <div class={style::idea_card}>
                        <h4>"Web Applications"</h4>
                        <p>"Full-stack apps, APIs, microservices"</p>
                    </div>
                    <div class={style::idea_card}>
                        <h4>"CLI Tools"</h4>
                        <p>"Developer tools, utilities, automation"</p>
                    </div>
                    <div class={style::idea_card}>
                        <h4>"Systems Software"</h4>
                        <p>"OS components, drivers, embedded"</p>
                    </div>
                    <div class={style::idea_card}>
                        <h4>"Game Development"</h4>
                        <p>"Games, engines, graphics programming"</p>
                    </div>
                </div>
                <button class={format!("{} {} {}", style::btn, style::btn_primary, style::btn_large)}>"Submit Project Idea"</button>
            </div>
        </div>
    }
}