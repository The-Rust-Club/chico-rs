use leptos::prelude::*;
use lucide_leptos::*;

stylance::import_style!(style, "advocacy.module.scss");

#[derive(Clone)]
struct Course {
    code: String,
    name: String,
    department: String,
    status: String,
    supporters: i32,
}

#[component]
pub fn Advocacy() -> impl IntoView {
    let courses = vec![
        Course {
            code: "CS 311".to_string(),
            name: "Computer Architecture".to_string(),
            department: "Computer Science".to_string(),
            status: "Under Review".to_string(),
            supporters: 127,
        },
        Course {
            code: "CS 211".to_string(),
            name: "Programming & Algorithms II".to_string(),
            department: "Computer Science".to_string(),
            status: "Pending".to_string(),
            supporters: 89,
        },
        Course {
            code: "CS 370".to_string(),
            name: "Parallel Programming".to_string(),
            department: "Computer Science".to_string(),
            status: "Approved".to_string(),
            supporters: 156,
        },
        Course {
            code: "CS 340".to_string(),
            name: "Operating Systems".to_string(),
            department: "Computer Science".to_string(),
            status: "Under Review".to_string(),
            supporters: 143,
        },
        Course {
            code: "EECE 344".to_string(),
            name: "Embedded Systems".to_string(),
            department: "Electrical Engineering".to_string(),
            status: "Pending".to_string(),
            supporters: 67,
        },
    ];

    let total_supporters = RwSignal::new(523);

    view! {
        <div class={style::page_container}>
            <div class={style::page_header}>
                <h1>"Rust in Academia"</h1>
                <p class={style::page_subtitle}>
                    "We're advocating for Rust to be accepted as a submission language in output-focused courses. "
                    "Join our petition to bring modern systems programming to the curriculum."
                </p>
            </div>

            <div class={style::petition_hero}>
                <div class={style::petition_content}>
                    <h2>"The Petition"</h2>
                    <div class={style::petition_text}>
                        <p>
                            "We, the students of The Rust Club, petition for the acceptance of Rust as an approved "
                            "programming language for coursework submissions in output-focused computer science and "
                            "engineering courses."
                        </p>
                        <h3>"Why Rust?"</h3>
                        <ul class={style::benefits_list}>
                            <li>"Memory safety without garbage collection"</li>
                            <li>"Industry adoption by major tech companies"</li>
                            <li>"Modern tooling and excellent documentation"</li>
                            <li>"Preparation for systems programming careers"</li>
                            <li>"Strong type system prevents common bugs"</li>
                        </ul>
                    </div>
                    <div class={style::petition_stats}>
                        <div class={format!("{} primary", style::stat_card)}>
                            <div class={style::stat_number}>{move || total_supporters.get()}</div>
                            <div class={style::stat_label}>"Total Supporters"</div>
                        </div>
                        <div class={style::stat_card}>
                            <div class={style::stat_number}>"5"</div>
                            <div class={style::stat_label}>"Courses Targeted"</div>
                        </div>
                        <div class={format!("{} success", style::stat_card)}>
                            <div class={style::stat_number}>"1"</div>
                            <div class={style::stat_label}>"Approved"</div>
                        </div>
                    </div>
                    <button class={format!("{} {} {}", style::btn, style::btn_primary, style::btn_large)} on:click=move |_| {
                        total_supporters.update(|n| *n += 1);
                    }>
                        "Sign the Petition"
                    </button>
                </div>
            </div>

            <section class="courses-section">
                <h2>"Target Courses"</h2>
                <div class="courses-table">
                    <div class="table-header">
                        <span>"Course"</span>
                        <span>"Department"</span>
                        <span>"Status"</span>
                        <span>"Supporters"</span>
                    </div>
                    {courses.into_iter().map(|course| {
                        view! {
                            <div class="table-row">
                                <div class="course-info">
                                    <span class="course-code">{course.code}</span>
                                    <span class="course-name">{course.name}</span>
                                </div>
                                <span class="department">{course.department}</span>
                                <span class={format!("status status-{}", course.status.to_lowercase().replace(" ", "-"))}>
                                    {course.status.clone()}
                                </span>
                                <div class="supporters">
                                    <span class="supporter-count">{course.supporters}</span>
                                    <button class="support-btn">"+"</button>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </section>

            <section class="arguments-section">
                <h2>"Our Arguments"</h2>
                <div class="arguments-grid">
                    <div class="argument-card">
                        <h3>"Industry Relevance"</h3>
                        <p>
                            "Rust is used in production by Microsoft, Google, Amazon, Discord, Dropbox, and many others. "
                            "Learning Rust prepares students for modern industry practices."
                        </p>
                    </div>
                    <div class="argument-card">
                        <h3>"Educational Value"</h3>
                        <p>
                            "Rust's ownership system teaches fundamental concepts about memory management "
                            "that are valuable regardless of the language used in professional practice."
                        </p>
                    </div>
                    <div class="argument-card">
                        <h3>"Safety First"</h3>
                        <p>
                            "Rust prevents entire classes of bugs at compile time, allowing students to focus "
                            "on algorithms and logic rather than debugging memory issues."
                        </p>
                    </div>
                    <div class="argument-card">
                        <h3>"Modern Tooling"</h3>
                        <p>
                            "Cargo, rustfmt, and clippy provide a superior development experience. "
                            "Students can focus on learning concepts rather than fighting with build systems."
                        </p>
                    </div>
                </div>
            </section>

            <section class="success-stories">
                <h2>"Success Stories"</h2>
                <div class="story-card success">
                    <div class="story-header">
                        <span class="success-badge"><Check size=16 />"SUCCESS"</span>
                        <h3>"CS 370: Parallel Programming"</h3>
                    </div>
                    <p>
                        "After presenting the benefits of Rust's fearless concurrency and demonstrating how it prevents "
                        "data races at compile time, the CS department approved Rust for CS 370 submissions. Students using "
                        "Rust reported fewer debugging hours and better understanding of parallel programming concepts."
                    </p>
                    <div class="story-stats">
                        <span>"156 supporters"</span>
                        <span>"3 months advocacy"</span>
                        <span>"Approved Fall 2024"</span>
                    </div>
                </div>
            </section>

            <section class="how-to-help">
                <h2>"How You Can Help"</h2>
                <div class="help-cards">
                    <div class="help-card">
                        <PenTool class:help-icon=true />
                        <h3>"Sign the Petition"</h3>
                        <p>"Add your name to show faculty that students want modern language options."</p>
                    </div>
                    <div class="help-card">
                        <MessageSquare class:help-icon=true />
                        <h3>"Talk to Professors"</h3>
                        <p>"Discuss with your professors about the benefits of allowing Rust submissions."</p>
                    </div>
                    <div class="help-card">
                        <Lightbulb class:help-icon=true />
                        <h3>"Create Examples"</h3>
                        <p>"Build example solutions in Rust to demonstrate its effectiveness for coursework."</p>
                    </div>
                    <div class="help-card">
                        <TrendingUp class:help-icon=true />
                        <h3>"Share Success Stories"</h3>
                        <p>"Document your positive experiences using Rust in approved courses."</p>
                    </div>
                </div>
            </section>

            <div class="advocacy-cta">
                <h2>"Ready to Make a Difference?"</h2>
                <p>"Join us in bringing Rust to the academic curriculum. Your voice matters!"</p>
                <div class="cta-actions">
                    <button class={format!("{} {} {}", style::btn, style::btn_primary, style::btn_large)}>"Sign the Petition"</button>
                    <button class="btn btn-secondary btn-large">"View Full Proposal"</button>
                </div>
            </div>
        </div>
    }
}