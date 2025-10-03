use leptos::prelude::*;
use lucide_leptos::*;

stylance::import_style!(style, "workshops.module.scss");

#[derive(Clone)]
struct Workshop {
    title: String,
    date: String,
    time: String,
    location: String,
    level: String,
    description: String,
    topics: Vec<String>,
}

#[component]
pub fn Workshops() -> impl IntoView {
    let upcoming_workshops = vec![
        Workshop {
            title: "Spring 2025 Workshop #1: Rust Fundamentals & CLI Tools".to_string(),
            date: "February 15, 2025".to_string(),
            time: "1:00 PM - 5:00 PM".to_string(),
            location: "Engineering Building, Room 301".to_string(),
            level: "Beginner".to_string(),
            description: "Our first workshop this semester! Perfect for newcomers to Rust. Learn the fundamentals and build your first CLI tool to add to your portfolio.".to_string(),
            topics: vec![
                "Setting up Rust development environment".to_string(),
                "Understanding ownership and borrowing".to_string(),
                "Working with Cargo and crates.io".to_string(),
                "Building a command-line todo application".to_string(),
                "Error handling with Result<T, E>".to_string(),
                "Testing your Rust code".to_string(),
            ],
        },
        Workshop {
            title: "Spring 2025 Workshop #2: Web Development & Deployment".to_string(),
            date: "April 12, 2025".to_string(),
            time: "1:00 PM - 5:00 PM".to_string(),
            location: "Engineering Building, Room 301".to_string(),
            level: "Intermediate".to_string(),
            description: "Our second and final workshop this semester. Build a complete web application with Rust and deploy it to the cloud. Great for your resume!".to_string(),
            topics: vec![
                "Web frameworks: Actix-Web vs Axum vs Rocket".to_string(),
                "RESTful API design and implementation".to_string(),
                "Database integration with SQLx".to_string(),
                "Frontend integration (connecting with React/Vue)".to_string(),
                "Deployment to cloud platforms (AWS/Railway/Shuttle)".to_string(),
                "Adding your project to your portfolio".to_string(),
            ],
        },
    ];

    let past_workshops = vec![
        Workshop {
            title: "Fall 2024 Workshop #2: Game Development with Bevy".to_string(),
            date: "November 16, 2024".to_string(),
            time: "1:00 PM - 5:00 PM".to_string(),
            location: "Engineering Building, Room 301".to_string(),
            level: "Intermediate".to_string(),
            description: "Built a complete 2D game using the Bevy engine. Students created portfolio projects they could show to potential employers.".to_string(),
            topics: vec![
                "Bevy ECS (Entity Component System)".to_string(),
                "Asset loading and sprite rendering".to_string(),
                "Game logic and state management".to_string(),
                "Input handling and physics".to_string(),
                "Packaging and distribution".to_string(),
            ],
        },
        Workshop {
            title: "Fall 2024 Workshop #1: Rust Fundamentals & HTTP Servers".to_string(),
            date: "September 21, 2024".to_string(),
            time: "1:00 PM - 5:00 PM".to_string(),
            location: "Engineering Building, Room 301".to_string(),
            level: "Beginner".to_string(),
            description: "Introduction to Rust for beginners, ending with building a simple HTTP server. Great first stepping stone into systems programming.".to_string(),
            topics: vec![
                "Rust installation and cargo basics".to_string(),
                "Ownership, borrowing, and lifetimes".to_string(),
                "Pattern matching and error handling".to_string(),
                "Building a simple HTTP server".to_string(),
                "Introduction to async programming".to_string(),
            ],
        },
        Workshop {
            title: "Summer 2024 Workshop #2: CLI Tools & Open Source".to_string(),
            date: "July 20, 2024".to_string(),
            time: "1:00 PM - 5:00 PM".to_string(),
            location: "Online (Summer Session)".to_string(),
            level: "Beginner".to_string(),
            description: "Built command-line tools and made first open source contributions. Several students got their first PRs merged!".to_string(),
            topics: vec![
                "Command-line argument parsing with clap".to_string(),
                "File I/O and text processing".to_string(),
                "Finding good first issues".to_string(),
                "Git workflow for contributions".to_string(),
                "Code review best practices".to_string(),
            ],
        },
    ];

    view! {
        <div class={style::page_container}>
            <div class={style::page_header}>
                <h1>"Workshops"</h1>
                <p class={style::page_subtitle}>
                    "Two comprehensive workshops every semester to help students onboard and master Rust. "
                    "From complete beginner to building portfolio-worthy projects in just one semester!"
                </p>
            </div>

            <section class={style::upcoming_workshops}>
                <h2>"Upcoming Workshops"</h2>
                <div class={style::workshops_grid}>
                    {upcoming_workshops.into_iter().map(|workshop| {
                        view! {
                            <div class={format!("{} featured", style::workshop_card)}>
                                <div class={style::workshop_header}>
                                    <span class={format!("{} level-{}", style::level_badge, workshop.level.to_lowercase())}>
                                        {workshop.level.clone()}
                                    </span>
                                    <span class={style::workshop_date}>{workshop.date}</span>
                                </div>
                                <h3 class={style::workshop_title}>{workshop.title}</h3>
                                <p class={style::workshop_description}>{workshop.description}</p>
                                <div class={style::workshop_details}>
                                    <div class={style::detail_item}>
                                        <Clock size=16 />
                                        <span>{workshop.time}</span>
                                    </div>
                                    <div class={style::detail_item}>
                                        <MapPin size=16 />
                                        <span>{workshop.location}</span>
                                    </div>
                                </div>
                                <div class={style::workshop_topics}>
                                    {if !workshop.topics.is_empty() {
                                        Some(view! {
                                            <>
                                                <h4>"Topics Covered:"</h4>
                                                <ul>
                                                    {workshop.topics.into_iter().map(|topic| {
                                                        view! { <li>{topic}</li> }
                                                    }).collect_view()}
                                                </ul>
                                            </>
                                        })
                                    } else {
                                        None
                                    }}
                                </div>
                                <button class={format!("{} {} {}", style::btn, style::btn_primary, style::btn_block)}>"Register Now"</button>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </section>

            <section class={style::workshop_tracks}>
                <h2>"Workshop Tracks"</h2>
                <div class={style::tracks_grid}>
                    <div class={style::track_card}>
                        <Target />
                        <h3>"Beginner Track"</h3>
                        <p>"Start from zero. Learn Rust fundamentals, syntax, and core concepts."</p>
                        <ul class={style::track_list}>
                            <li>"Rust setup and tooling"</li>
                            <li>"Basic syntax and types"</li>
                            <li>"Ownership system"</li>
                            <li>"Error handling"</li>
                        </ul>
                    </div>
                    <div class={style::track_card}>
                        <Zap />
                        <h3>"Intermediate Track"</h3>
                        <p>"Build real applications. Web services, CLI tools, and more."</p>
                        <ul class={style::track_list}>
                            <li>"Web development"</li>
                            <li>"Database integration"</li>
                            <li>"Testing strategies"</li>
                            <li>"Performance optimization"</li>
                        </ul>
                    </div>
                    <div class={style::track_card}>
                        <Rocket />
                        <h3>"Advanced Track"</h3>
                        <p>"Master complex topics. Systems programming, async, and unsafe Rust."</p>
                        <ul class={style::track_list}>
                            <li>"Async programming"</li>
                            <li>"Systems programming"</li>
                            <li>"Unsafe Rust"</li>
                            <li>"Performance tuning"</li>
                        </ul>
                    </div>
                </div>
            </section>

            <section class={style::past_workshops}>
                <h2>"Past Workshops"</h2>
                <div class={style::past_workshops_list}>
                    {past_workshops.into_iter().map(|workshop| {
                        view! {
                            <div class={style::past_workshop_item}>
                                <div class={style::past_workshop_info}>
                                    <h4>{workshop.title}</h4>
                                    <p>{workshop.description}</p>
                                </div>
                                <div class={style::past_workshop_meta}>
                                    <span class={style::workshop_date}>{workshop.date}</span>
                                    <a href="#" class={style::materials_link}>"View Materials →"</a>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </section>

            <div class={style::workshop_cta}>
                <h2>"Want to Request a Workshop Topic?"</h2>
                <p>"We're always looking for new workshop ideas. Let us know what you'd like to learn!"</p>
                <button class={format!("{} {}", style::btn, style::btn_primary)}>"Submit Request"</button>
            </div>
        </div>
    }
}