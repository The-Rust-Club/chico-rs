use worker::{query, Result, D1Database};
use crate::models::*;
use serde::Deserialize;

pub struct DatabaseService;

// Helper structs for database rows
#[derive(Deserialize)]
struct EventRow {
    uuid: String,
    title: String,
    description: String,
    date: String,
    time: String,
    location: String,
    event_type: String,
    recurring: i32,
    created_at: String,
}

#[derive(Deserialize)]
struct IssueRow {
    uuid: String,
    title: String,
    description: String,
    repo: String,
    github_url: String,
    difficulty: String,
    tags: String,
    created_at: String,
}

#[derive(Deserialize)]
struct ProjectRow {
    uuid: String,
    name: String,
    description: String,
    github_url: String,
    leader_name: String,
    leader_github: Option<String>,
    status: String,
    tech_stack: String,
    contributors_needed: i32,
    skills_needed: String,
    created_at: String,
}

#[derive(Deserialize)]
struct BlogPostRow {
    id: String,
    title: String,
    slug: String,
    excerpt: String,
    post_type: String,
    category: String,
    tags: String,
    author_name: String,
    author_github: Option<String>,
    difficulty_level: Option<String>,
    estimated_read_time: i32,
    published_at: String,
    updated_at: Option<String>,
    views: i32,
    likes: i32,
    markdown_url: String,
    series_title: Option<String>,
    series_part: Option<i32>,
    series_total_parts: Option<i32>,
    external_links: Option<String>,
}

impl DatabaseService {
    pub async fn get_events(db: &D1Database) -> Result<Vec<Event>> {
        let stmt = query!(db, "SELECT uuid, title, description, date, time, location, event_type, recurring, created_at FROM events ORDER BY created_at DESC");
        let results = stmt.all().await?;

        let mut events = Vec::new();

        // Parse the results
        if let Ok(rows) = results.results::<EventRow>() {
            for row in rows {
                let event = Event {
                    id: row.uuid,
                    title: row.title,
                    description: row.description,
                    date: row.date,
                    time: row.time,
                    location: row.location,
                    event_type: match row.event_type.as_str() {
                        "Workshop" => EventType::Workshop,
                        "StudyGroup" => EventType::StudyGroup,
                        "Seminar" => EventType::Seminar,
                        "Hackathon" => EventType::Hackathon,
                        "Panel" => EventType::Panel,
                        "Networking" => EventType::Networking,
                        _ => EventType::Workshop,
                    },
                    recurring: row.recurring != 0,
                    created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                        .unwrap_or_default()
                        .with_timezone(&chrono::Utc),
                };
                events.push(event);
            }
            return Ok(events);
        }

        // Fallback to sample data if database is empty
        let events = vec![
            Event {
                id: "550e8400-e29b-41d4-a716-446655440001".to_string(),
                title: "Spring 2025 Workshop #1: Rust Fundamentals".to_string(),
                description: "Learn Rust fundamentals including ownership, borrowing, and building your first CLI tool. Perfect for beginners!".to_string(),
                date: "February 15".to_string(),
                time: "1:00 PM - 5:00 PM".to_string(),
                location: "Engineering Building, Room 301".to_string(),
                event_type: EventType::Workshop,
                recurring: false,
                created_at: chrono::Utc::now(),
            },
            Event {
                id: "550e8400-e29b-41d4-a716-446655440002".to_string(),
                title: "Weekly Study Group".to_string(),
                description: "Join us for collaborative learning and project work. Bring your Rust questions!".to_string(),
                date: "Every Friday".to_string(),
                time: "3:00 PM - 5:00 PM".to_string(),
                location: "Library, Study Room B".to_string(),
                event_type: EventType::StudyGroup,
                recurring: true,
                created_at: chrono::Utc::now(),
            },
            Event {
                id: "550e8400-e29b-41d4-a716-446655440003".to_string(),
                title: "Industry Guest Speaker: WebAssembly in Production".to_string(),
                description: "Learn how major companies are using Rust and WebAssembly in production environments.".to_string(),
                date: "March 8".to_string(),
                time: "6:00 PM - 8:00 PM".to_string(),
                location: "Auditorium A, Student Center".to_string(),
                event_type: EventType::Seminar,
                recurring: false,
                created_at: chrono::Utc::now(),
            },
        ];
        Ok(events)
    }

    pub async fn get_issues(db: &D1Database) -> Result<Vec<Issue>> {
        let stmt = query!(db, "SELECT uuid, title, description, repo, github_url, difficulty, tags, created_at FROM issues ORDER BY created_at DESC");
        let results = stmt.all().await?;

        let mut issues = Vec::new();

        // Parse the results
        if let Ok(rows) = results.results::<IssueRow>() {
            for row in rows {
                let issue = Issue {
                    id: row.uuid,
                    title: row.title,
                    description: row.description,
                    repo: row.repo,
                    github_url: row.github_url,
                    difficulty: match row.difficulty.as_str() {
                        "Easy" => DifficultyLevel::Easy,
                        "Medium" => DifficultyLevel::Medium,
                        "Hard" => DifficultyLevel::Hard,
                        _ => DifficultyLevel::Easy,
                    },
                    tags: serde_json::from_str(&row.tags).unwrap_or_default(),
                    created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                        .unwrap_or_default()
                        .with_timezone(&chrono::Utc),
                };
                issues.push(issue);
            }
            return Ok(issues);
        }

        // Fallback to sample data if database is empty
        let issues = vec![
            Issue {
                id: "550e8400-e29b-41d4-a716-446655440010".to_string(),
                title: "Add documentation for async patterns".to_string(),
                description: "We need comprehensive documentation covering async/await patterns in Rust. This would help newcomers understand concurrent programming.".to_string(),
                repo: "rust-lang/reference".to_string(),
                github_url: "https://github.com/rust-lang/reference/issues/123".to_string(),
                difficulty: DifficultyLevel::Easy,
                tags: vec!["documentation".to_string(), "async".to_string(), "good-first-issue".to_string()],
                created_at: chrono::Utc::now(),
            },
            Issue {
                id: "550e8400-e29b-41d4-a716-446655440011".to_string(),
                title: "Improve error message for trait bound errors".to_string(),
                description: "Current error messages for complex trait bounds can be confusing. We need clearer, more actionable error messages.".to_string(),
                repo: "rust-lang/rust".to_string(),
                github_url: "https://github.com/rust-lang/rust/issues/456".to_string(),
                difficulty: DifficultyLevel::Medium,
                tags: vec!["diagnostics".to_string(), "error-messages".to_string(), "good-first-issue".to_string()],
                created_at: chrono::Utc::now(),
            },
        ];
        Ok(issues)
    }

    pub async fn get_projects(db: &D1Database) -> Result<Vec<Project>> {
        let stmt = query!(
            db,
            "SELECT p.uuid, p.name, p.description, p.github_url,
                    m.name as leader_name, m.github_username as leader_github,
                    p.status, p.tech_stack, p.contributors_needed, p.skills_needed, p.created_at
             FROM projects p
             JOIN members m ON p.leader_id = m.id
             ORDER BY p.created_at DESC"
        );
        let results = stmt.all().await?;

        let mut projects = Vec::new();

        // Parse the results
        if let Ok(rows) = results.results::<ProjectRow>() {
            for row in rows {
                let project = Project {
                    id: row.uuid,
                    name: row.name,
                    description: row.description,
                    github_url: row.github_url,
                    leader: Member {
                        name: row.leader_name,
                        github_username: row.leader_github,
                    },
                    contributors: vec![], // TODO: Load contributors with separate query
                    status: match row.status.as_str() {
                        "Planning" => ProjectStatus::Planning,
                        "Active" => ProjectStatus::Active,
                        "InDevelopment" => ProjectStatus::InDevelopment,
                        "Beta" => ProjectStatus::Beta,
                        "Completed" => ProjectStatus::Completed,
                        "Archived" => ProjectStatus::Archived,
                        _ => ProjectStatus::Planning,
                    },
                    tech_stack: serde_json::from_str(&row.tech_stack).unwrap_or_default(),
                    contributors_needed: row.contributors_needed != 0,
                    skills_needed: serde_json::from_str(&row.skills_needed).unwrap_or_default(),
                    created_at: chrono::DateTime::parse_from_rfc3339(&row.created_at)
                        .unwrap_or_default()
                        .with_timezone(&chrono::Utc),
                };
                projects.push(project);
            }
            return Ok(projects);
        }

        // Fallback to sample data if database is empty
        let projects = vec![
            Project {
                id: "550e8400-e29b-41d4-a716-446655440020".to_string(),
                name: "The Rust Club Website".to_string(),
                description: "Official website for The Rust Club built with Leptos and deployed on Cloudflare Pages.".to_string(),
                github_url: "https://github.com/rust-club/website".to_string(),
                leader: Member {
                    name: "Alex Chen".to_string(),
                    github_username: Some("alexcodes".to_string()),
                },
                contributors: vec![
                    Member {
                        name: "Jordan Smith".to_string(),
                        github_username: Some("jordandev".to_string()),
                    },
                    Member {
                        name: "Sam Wilson".to_string(),
                        github_username: Some("samw".to_string()),
                    },
                ],
                status: ProjectStatus::Active,
                tech_stack: vec!["Leptos".to_string(), "Trunk".to_string(), "CSS".to_string(), "Cloudflare".to_string()],
                contributors_needed: true,
                skills_needed: vec!["Frontend".to_string(), "CSS".to_string(), "Design".to_string()],
                created_at: chrono::Utc::now(),
            },
            Project {
                id: "550e8400-e29b-41d4-a716-446655440021".to_string(),
                name: "RustBot Discord Bot".to_string(),
                description: "A Discord bot for The Rust Club server with moderation, event management, and learning resources.".to_string(),
                github_url: "https://github.com/rust-club/rustbot".to_string(),
                leader: Member {
                    name: "Taylor Rodriguez".to_string(),
                    github_username: Some("taylorr".to_string()),
                },
                contributors: vec![
                    Member {
                        name: "Casey Johnson".to_string(),
                        github_username: Some("caseyjay".to_string()),
                    },
                ],
                status: ProjectStatus::InDevelopment,
                tech_stack: vec!["Rust".to_string(), "Serenity".to_string(), "SQLite".to_string()],
                contributors_needed: true,
                skills_needed: vec!["Backend".to_string(), "Discord API".to_string(), "Database".to_string()],
                created_at: chrono::Utc::now(),
            },
        ];
        Ok(projects)
    }

    pub async fn get_member_count(db: &D1Database) -> Result<u32> {
        let stmt = query!(db, "SELECT COUNT(*) as count FROM members");
        let result = stmt.first::<serde_json::Value>(None).await?;

        if let Some(row) = result {
            if let Some(count) = row.get("count").and_then(|v| v.as_u64()) {
                return Ok(count as u32);
            }
        }

        // Fallback count if database is empty
        Ok(9)
    }

    pub async fn get_blog_posts(db: &D1Database) -> Result<Vec<BlogPost>> {
        let stmt = query!(
            db,
            "SELECT id, title, slug, excerpt, post_type, category, tags, author_name, author_github,
                    difficulty_level, estimated_read_time, published_at, updated_at, views, likes,
                    markdown_url, series_title, series_part, series_total_parts, external_links
             FROM blog_posts
             ORDER BY published_at DESC"
        );
        let results = stmt.all().await?;

        let mut posts = Vec::new();

        if let Ok(rows) = results.results::<BlogPostRow>() {
            for row in rows {
                let post = Self::blog_post_from_row(row);
                posts.push(post);
            }
        }

        Ok(posts)
    }

    pub async fn get_blog_posts_by_ids(db: &D1Database, ids: &[String]) -> Result<Vec<BlogPost>> {
        if ids.is_empty() {
            return Ok(Vec::new());
        }

        let placeholders = ids.iter().map(|_| "?").collect::<Vec<_>>().join(",");
        let query_str = format!(
            "SELECT id, title, slug, excerpt, post_type, category, tags, author_name, author_github,
                    difficulty_level, estimated_read_time, published_at, updated_at, views, likes,
                    markdown_url, series_title, series_part, series_total_parts, external_links
             FROM blog_posts
             WHERE id IN ({})
             ORDER BY published_at DESC",
            placeholders
        );

        // Convert IDs to JsValue for binding
        let bind_values: Vec<wasm_bindgen::JsValue> = ids.iter()
            .map(|id| wasm_bindgen::JsValue::from_str(id))
            .collect();

        let stmt = db.prepare(&query_str).bind(&bind_values)?;
        let results = stmt.all().await?;
        let mut posts = Vec::new();

        if let Ok(rows) = results.results::<BlogPostRow>() {
            for row in rows {
                let post = Self::blog_post_from_row(row);
                posts.push(post);
            }
        }

        Ok(posts)
    }

    pub async fn get_blog_post_by_slug(db: &D1Database, slug: &str) -> Result<Option<BlogPost>> {
        let stmt = query!(
            db,
            "SELECT id, title, slug, excerpt, post_type, category, tags, author_name, author_github,
                    difficulty_level, estimated_read_time, published_at, updated_at, views, likes,
                    markdown_url, series_title, series_part, series_total_parts, external_links
             FROM blog_posts
             WHERE slug = ?",
            slug
        );
        let result = stmt?.first::<BlogPostRow>(None).await?;

        if let Some(row) = result {
            let post = Self::blog_post_from_row(row);
            Ok(Some(post))
        } else {
            Ok(None)
        }
    }

    pub async fn increment_blog_post_views(db: &D1Database, post_id: &str) -> Result<()> {
        let stmt = query!(
            db,
            "UPDATE blog_posts SET views = views + 1 WHERE id = ?",
            post_id
        );
        let _ = stmt?.run().await?;
        Ok(())
    }

    fn blog_post_from_row(row: BlogPostRow) -> BlogPost {
        BlogPost {
            id: row.id,
            title: row.title,
            slug: row.slug,
            excerpt: row.excerpt,
            post_type: match row.post_type.as_str() {
                "tutorial" => BlogPostType::Tutorial,
                "guide" => BlogPostType::Guide,
                "show_and_tell" => BlogPostType::ShowAndTell,
                "tech_talk" => BlogPostType::TechTalk,
                "news" => BlogPostType::News,
                "review" => BlogPostType::Review,
                _ => BlogPostType::Tutorial,
            },
            category: match row.category.as_str() {
                "fundamentals" => BlogCategory::Fundamentals,
                "web_development" => BlogCategory::WebDevelopment,
                "systems_programming" => BlogCategory::SystemsProgramming,
                "game_development" => BlogCategory::GameDevelopment,
                "cli" => BlogCategory::CLI,
                "data_science" => BlogCategory::DataScience,
                "blockchain" => BlogCategory::Blockchain,
                "performance" => BlogCategory::Performance,
                "testing" => BlogCategory::Testing,
                "deployment" => BlogCategory::Deployment,
                "career" => BlogCategory::Career,
                "community" => BlogCategory::Community,
                _ => BlogCategory::Fundamentals,
            },
            tags: serde_json::from_str(&row.tags).unwrap_or_default(),
            author_name: row.author_name,
            author_github: row.author_github,
            difficulty_level: row.difficulty_level.map(|d| match d.as_str() {
                "easy" => DifficultyLevel::Easy,
                "medium" => DifficultyLevel::Medium,
                "hard" => DifficultyLevel::Hard,
                _ => DifficultyLevel::Easy,
            }),
            estimated_read_time: row.estimated_read_time as u32,
            published_at: row.published_at,
            updated_at: row.updated_at,
            views: row.views as u32,
            likes: row.likes as u32,
            markdown_url: row.markdown_url,
            series: if let Some(title) = row.series_title {
                Some(BlogSeries {
                    title,
                    part: row.series_part.unwrap_or(1) as u32,
                    total_parts: row.series_total_parts.map(|t| t as u32),
                })
            } else {
                None
            },
            external_links: row.external_links
                .and_then(|links| serde_json::from_str(&links).ok())
                .unwrap_or_default(),
        }
    }
}