use leptos::prelude::*;
use leptos_router::components::A;
use lucide_leptos::*;

stylance::import_style!(style, "about.module.scss");

#[component]
pub fn About() -> impl IntoView {
    view! {
        <div class={style::page_container}>
            <div class={style::page_header}>
                <h1>"About The Rust Club"</h1>
                <p class={style::page_subtitle}>
                    "Learn, Build, and Contribute to the Rust Ecosystem"
                </p>
            </div>

            <section class={style::about_section}>
                <h2>"Our Mission"</h2>
                <p class={style::mission_text}>
                    "The Rust Club is a student organization dedicated to teaching Rust programming, "
                    "fostering open source contributions, and advocating for modern systems programming in academia. "
                    "We believe Rust is the future of safe, fast, and reliable software development."
                </p>
            </section>

            <section class={style::what_we_do}>
                <h2>"What We Do"</h2>
                <div class={style::activities_grid}>
                    <div class={style::activity_card}>
                        <GitFork class:activity-icon=true />
                        <h3>"Open Source Contributions"</h3>
                        <p>"We curate beginner-friendly issues in real Rust projects, helping members make their first contributions and build their professional portfolios."</p>
                    </div>
                    
                    <div class={style::activity_card}>
                        <GraduationCap class:activity-icon=true />
                        <h3>"Workshops & Learning"</h3>
                        <p>"Two comprehensive workshops per semester take you from Rust basics to building production applications. Perfect for all skill levels."</p>
                    </div>
                    
                    <div class={style::activity_card}>
                        <Users class:activity-icon=true />
                        <h3>"Community & Networking"</h3>
                        <p>"Weekly study groups, code reviews, and networking events connect you with fellow Rust enthusiasts and industry professionals."</p>
                    </div>
                    
                    <div class={style::activity_card}>
                        <Rocket class:activity-icon=true />
                        <h3>"Club Projects"</h3>
                        <p>"Work on real-world projects like Discord bots, CLI tools, and web applications. Gain hands-on experience you can showcase to employers."</p>
                    </div>
                    
                    <div class={style::activity_card}>
                        <Megaphone class:activity-icon=true />
                        <h3>"Academic Advocacy"</h3>
                        <p>"We're working to get Rust accepted in computer science coursework, bringing modern programming practices to the curriculum."</p>
                    </div>
                    
                    <div class={style::activity_card}>
                        <Star class:activity-icon=true />
                        <h3>"Career Development"</h3>
                        <p>"Build a portfolio of Rust projects, contribute to your resume, and connect with companies using Rust in production."</p>
                    </div>
                </div>
            </section>

            <section class={style::stats_section}>
                <h2>"Our Impact"</h2>
                <div class={style::stats_grid}>
                    <div class={style::stat_card}>
                        <div class={style::stat_number}>"47"</div>
                        <div class={style::stat_label}>"PRs Merged This Semester"</div>
                    </div>
                    <div class={style::stat_card}>
                        <div class={style::stat_number}>"23"</div>
                        <div class={style::stat_label}>"Active Members"</div>
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
            </section>

            <section class={style::join_section}>
                <h2>"Ready to Join?"</h2>
                <p>"Whether you're a complete beginner or an experienced developer, there's a place for you in The Rust Club."</p>
                <div class={style::join_buttons}>
                    <a href="https://discord.gg/rust" target="_blank" class={format!("{} {} {}", style::btn, style::btn_primary, style::btn_large)}>
                        "Join Our Discord"
                    </a>
                    <A href="/workshops">
                        <span class={format!("{} {} {}", style::btn, style::btn_secondary, style::btn_large)}>
                            "Attend a Workshop"
                        </span>
                    </A>
                </div>
            </section>
        </div>
    }
}