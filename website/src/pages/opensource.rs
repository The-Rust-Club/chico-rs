use leptos::prelude::*;
use lucide_leptos::*;

stylance::import_style!(style, "opensource.module.scss");

#[derive(Clone)]
struct Issue {
    title: String,
    repo: String,
    url: String,
    difficulty: String,
    tags: Vec<String>,
    description: String,
}

#[component]
pub fn OpenSource() -> impl IntoView {
    let issues = vec![
        Issue {
            title: "Fix broken link in README.md".to_string(),
            repo: "rust-lang/mdBook".to_string(),
            url: "https://github.com/rust-lang/mdBook/issues/2450".to_string(),
            difficulty: "Easy".to_string(),
            tags: vec!["documentation".to_string(), "good-first-issue".to_string()],
            description: "A simple documentation fix - update broken links in the main README file. Great first contribution!".to_string(),
        },
        Issue {
            title: "Add example for custom derive macro".to_string(),
            repo: "tokio-rs/tokio".to_string(),
            url: "https://github.com/tokio-rs/tokio/issues/6234".to_string(),
            difficulty: "Easy".to_string(),
            tags: vec!["documentation".to_string(), "examples".to_string()],
            description: "Add a code example showing how to use derive macros with async traits. Help other developers learn!".to_string(),
        },
        Issue {
            title: "Improve error message for type mismatch".to_string(),
            repo: "serde-rs/serde".to_string(),
            url: "https://github.com/serde-rs/serde/issues/2789".to_string(),
            difficulty: "Easy".to_string(),
            tags: vec!["error-handling".to_string(), "user-experience".to_string()],
            description: "Make error messages more helpful by providing context about expected vs actual types.".to_string(),
        },
        Issue {
            title: "Add unit tests for new feature flag".to_string(),
            repo: "actix/actix-web".to_string(),
            url: "https://github.com/actix/actix-web/issues/3156".to_string(),
            difficulty: "Medium".to_string(),
            tags: vec!["testing".to_string(), "feature-flags".to_string()],
            description: "Write comprehensive tests for the new compression feature. Good practice with Rust testing patterns.".to_string(),
        },
        Issue {
            title: "Implement trait for custom deserializer".to_string(),
            repo: "clap-rs/clap".to_string(),
            url: "https://github.com/clap-rs/clap/issues/4923".to_string(),
            difficulty: "Medium".to_string(),
            tags: vec!["traits".to_string(), "cli".to_string()],
            description: "Implement a trait to support custom value parsing. Learn about trait design and CLI argument handling.".to_string(),
        },
        Issue {
            title: "Add beginner-friendly CLI examples".to_string(),
            repo: "rust-lang/cargo".to_string(),
            url: "https://github.com/rust-lang/cargo/issues/12543".to_string(),
            difficulty: "Easy".to_string(),
            tags: vec!["documentation".to_string(), "cli".to_string(), "good-first-issue".to_string()],
            description: "Create simple, well-commented examples for common cargo commands to help new Rust developers.".to_string(),
        },
        Issue {
            title: "Fix typo in function documentation".to_string(),
            repo: "rust-random/rand".to_string(),
            url: "https://github.com/rust-random/rand/issues/1432".to_string(),
            difficulty: "Easy".to_string(),
            tags: vec!["documentation".to_string(), "typo".to_string()],
            description: "Simple typo fix in documentation comments. Perfect for your first open source contribution!".to_string(),
        },
        Issue {
            title: "Add validation for config file format".to_string(),
            repo: "rust-analyzer/rust-analyzer".to_string(),
            url: "https://github.com/rust-analyzer/rust-analyzer/issues/15678".to_string(),
            difficulty: "Medium".to_string(),
            tags: vec!["validation".to_string(), "config".to_string()],
            description: "Implement input validation for configuration files. Learn about error handling and user input validation.".to_string(),
        },
        Issue {
            title: "Update deprecated API usage in examples".to_string(),
            repo: "hyperium/hyper".to_string(),
            url: "https://github.com/hyperium/hyper/issues/3241".to_string(),
            difficulty: "Easy".to_string(),
            tags: vec!["examples".to_string(), "deprecation".to_string()],
            description: "Update code examples to use the new API instead of deprecated functions. Learn about HTTP libraries in Rust.".to_string(),
        },
        Issue {
            title: "Improve performance of string operations".to_string(),
            repo: "BurntSushi/ripgrep".to_string(),
            url: "https://github.com/BurntSushi/ripgrep/issues/2456".to_string(),
            difficulty: "Hard".to_string(),
            tags: vec!["performance".to_string(), "optimization".to_string()],
            description: "Optimize string processing for better performance. Advanced contribution involving profiling and optimization.".to_string(),
        },
    ];

    view! {
        <div class={style::page_container}>
            <div class={style::page_header}>
                <h1>"Open Source Opportunities"</h1>
                <p class={style::page_subtitle}>
                    "Start your open source journey with these curated, beginner-friendly Rust issues. "
                    "Each issue has been selected to help you learn while contributing to real projects."
                </p>
            </div>

            <div class={style::filters_section}>
                <div class={style::filter_chips}>
                    <button class={format!("{} {}", style::chip, style::chip_active)}>"All"</button>
                    <button class={style::chip}>"Easy"</button>
                    <button class={style::chip}>"Medium"</button>
                    <button class={style::chip}>"Hard"</button>
                    <button class={style::chip}>"Documentation"</button>
                    <button class={style::chip}>"Testing"</button>
                </div>
            </div>

            <div class={style::issues_grid}>
                {issues.into_iter().map(|issue| {
                    view! {
                        <div class={style::issue_card}>
                            <div class={style::issue_header}>
                                <span class={format!("{} difficulty-{}", style::difficulty_badge, issue.difficulty.to_lowercase())}>
                                    {issue.difficulty.clone()}
                                </span>
                                <span class={style::repo_name}>{issue.repo}</span>
                            </div>
                            <h3 class={style::issue_title}>{issue.title}</h3>
                            <p class={style::issue_description}>{issue.description}</p>
                            <div class={style::issue_tags}>
                                {issue.tags.into_iter().map(|tag| {
                                    view! { <span class={style::tag}>{"#"}{tag}</span> }
                                }).collect_view()}
                            </div>
                            <div class={style::issue_actions}>
                                <a href={issue.url} target="_blank" class={style::issue_link}>
                                    <ExternalLink size=16 />
                                    "View on GitHub"
                                </a>
                                <button class={style::claim_btn}>"Claim Issue"</button>
                            </div>
                        </div>
                    }
                }).collect_view()}
            </div>

            <div class={style::tips_section}>
                <h2>"Your Open Source Journey"</h2>
                <div class={style::tips_grid}>
                    <div class={style::tip_card}>
                        <h3>"1. Set Up Your Environment"</h3>
                        <p>"Install Rust, configure your IDE, and set up Git. Our workshops cover development environment setup - check the schedule!"</p>
                    </div>
                    <div class={style::tip_card}>
                        <h3>"2. Join Our Discord"</h3>
                        <p>"Get real-time help from club members who are also contributing to open source. Share your progress and celebrate wins together!"</p>
                    </div>
                    <div class={style::tip_card}>
                        <h3>"3. Claim Your Issue"</h3>
                        <p>"Use our 'Claim Issue' feature to let other club members know you're working on it. This prevents duplicate work and builds accountability."</p>
                    </div>
                    <div class={style::tip_card}>
                        <h3>"4. Read Contributing Guidelines"</h3>
                        <p>"Every project has its own guidelines. Take time to understand the project's code style, testing requirements, and PR process."</p>
                    </div>
                    <div class={style::tip_card}>
                        <h3>"5. Start Small, Think Big"</h3>
                        <p>"Begin with documentation fixes or simple bug fixes. These help you understand the codebase before tackling major features."</p>
                    </div>
                    <div class={style::tip_card}>
                        <h3>"6. Add It to Your Resume"</h3>
                        <p>"Once your PR is merged, you can proudly say 'Contributed to open source projects in Rust' on your resume and LinkedIn!"</p>
                    </div>
                </div>
            </div>

            <section class={style::contribution_tracker}>
                <h2>"Club Contribution Stats"</h2>
                <div class={style::stats_grid}>
                    <div class={style::stat_card}>
                        <div class={style::stat_number}>"47"</div>
                        <div class={style::stat_label}>"PRs Merged This Semester"</div>
                    </div>
                    <div class={style::stat_card}>
                        <div class={style::stat_number}>"23"</div>
                        <div class={style::stat_label}>"Active Contributors"</div>
                    </div>
                    <div class={style::stat_card}>
                        <div class={style::stat_number}>"12"</div>
                        <div class={style::stat_label}>"First-Time Contributors"</div>
                    </div>
                    <div class={style::stat_card}>
                        <div class={style::stat_number}>"8"</div>
                        <div class={style::stat_label}>"Projects Contributed To"</div>
                    </div>
                </div>
                <p class={style::stats_note}>"These numbers represent our club members' collective contribution to the Rust ecosystem. Your contributions count!"</p>
            </section>
        </div>
    }
}