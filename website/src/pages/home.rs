use leptos::prelude::*;
use lucide_leptos::*;
use crate::api::{mock, DifficultyLevel};

stylance::import_style!(style, "home.module.scss");

#[component]
pub fn Home() -> impl IntoView {
    // For now, use mock data until we fix the async integration
    let upcoming_events = mock::get_upcoming_events();
    let recent_issues = mock::get_issues().into_iter().take(6).collect::<Vec<_>>();
    let featured_projects = mock::get_club_projects().into_iter().take(4).collect::<Vec<_>>();

    view! {
        <div class="page-container">
            // Header section
            <section class={style::dashboard_header}>
                <div class={style::welcome_section}>
                    <h1 class={style::welcome_title}>
                        <Code />
                        "Welcome to The Rust Club"
                    </h1>
                    <p class={style::welcome_subtitle}>
                        "Stay up to date with the latest events, issues, and student projects"
                    </p>
                </div>
            </section>

            <div class={style::dashboard_grid}>
                // Upcoming Events Section
                <section class={style::events_section}>
                    <div class={style::section_header}>
                        <h2 class={style::section_title}>
                            <Calendar />
                            "Upcoming Events"
                        </h2>
                        <a href="/events" class={style::section_link}>"View All →"</a>
                    </div>
                    <div class={style::events_list}>
                        {upcoming_events.into_iter().take(3).map(|event| {
                            view! {
                                <div class={style::event_card}>
                                    <div class={style::event_content}>
                                        <div class={style::event_header}>
                                            <h3 class={style::event_title}>{event.title.clone()}</h3>
                                            <span class={style::event_date_badge}>{event.date}</span>
                                        </div>
                                        <p class={style::event_description}>{event.description}</p>
                                        <div class={style::event_meta}>
                                            <span class={style::event_time}>
                                                <Clock />
                                                {event.time}
                                            </span>
                                            <span class={style::event_location}>
                                                <MapPin />
                                                {event.location}
                                            </span>
                                        </div>
                                    </div>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </section>

                // Recent Issues Section
                <section class={style::issues_section}>
                    <div class={style::section_header}>
                        <h2 class={style::section_title}>
                            <Bug />
                            "Good First Issues"
                        </h2>
                        <a href="/opensource" class={style::section_link}>"Browse All →"</a>
                    </div>
                    <div class={style::issues_list}>
                        {recent_issues.into_iter().map(|issue| {
                            let difficulty_class = match issue.difficulty {
                                DifficultyLevel::Easy => style::difficulty_easy,
                                DifficultyLevel::Medium => style::difficulty_medium,
                                DifficultyLevel::Hard => style::difficulty_hard,
                            };
                            
                            view! {
                                <div class={style::issue_card}>
                                    <div class={style::issue_header}>
                                        <h3 class={style::issue_title}>{issue.title.clone()}</h3>
                                        <span class={format!("{} {}", style::difficulty_badge, difficulty_class)}>
                                            {match issue.difficulty {
                                                DifficultyLevel::Easy => "Easy",
                                                DifficultyLevel::Medium => "Medium",
                                                DifficultyLevel::Hard => "Hard",
                                            }}
                                        </span>
                                    </div>
                                    <p class={style::issue_description}>{issue.description}</p>
                                    <div class={style::issue_footer}>
                                        <div class={style::issue_tags}>
                                            {issue.tags.into_iter().take(2).map(|tag| {
                                                view! { <span class={style::issue_tag}>{tag}</span> }
                                            }).collect_view()}
                                        </div>
                                        <a href={issue.github_url} target="_blank" class={style::issue_link}>
                                            <ExternalLink />
                                            "View Issue"
                                        </a>
                                    </div>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </section>

                // Student Projects Section
                <section class={style::projects_section}>
                    <div class={style::section_header}>
                        <h2 class={style::section_title}>
                            <Rocket />
                            "Student Projects"
                        </h2>
                        <a href="/projects" class={style::section_link}>"View All →"</a>
                    </div>
                    <div class={style::projects_grid}>
                        {featured_projects.into_iter().map(|project| {
                            view! {
                                <div class={style::project_card}>
                                    <div class={style::project_header}>
                                        <h3 class={style::project_title}>{project.name.clone()}</h3>
                                        <div class={style::project_stats}>
                                            <span class={style::project_stat}>
                                                <Users />
                                                {project.contributors.len()}
                                            </span>
                                        </div>
                                    </div>
                                    <p class={style::project_description}>{project.description}</p>
                                    <div class={style::project_footer}>
                                        <span class={style::project_author}>
                                            "by "{project.leader.name}
                                        </span>
                                        <a href={project.github_url} target="_blank" class={style::project_link}>
                                            <Github />
                                            "View Code"
                                        </a>
                                    </div>
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </section>
            </div>

            // Quick Actions Section
            <section class={style::quick_actions}>
                <h2 class={style::actions_title}>"Quick Actions"</h2>
                <div class={style::actions_grid}>
                    <a href="/blog" class={style::action_card}>
                        <BookOpen />
                        <span>"Read Latest Tutorials"</span>
                    </a>
                    <a href="/opensource" class={style::action_card}>
                        <GitPullRequest />
                        <span>"Find Issues to Solve"</span>
                    </a>
                    <a href="/workshops" class={style::action_card}>
                        <GraduationCap />
                        <span>"Join Next Workshop"</span>
                    </a>
                    <a href="https://discord.chico.rs" target="_blank" class={style::action_card}>
                        <MessageSquare />
                        <span>"Join Discord Chat"</span>
                    </a>
                </div>
            </section>
        </div>
    }
}
