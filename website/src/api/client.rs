use shared::*;

const API_BASE_URL: &str = "https://api.chico.rs";

// Stats API
pub async fn fetch_stats() -> Result<Stats, String> {
    let url = format!("{}/v1/stats", API_BASE_URL);

    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Stats>().await {
                    Ok(stats) => Ok(stats),
                    Err(e) => Err(format!("Failed to parse stats: {}", e)),
                }
            } else {
                Err(format!("HTTP error: {}", response.status()))
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}

// Events API
pub async fn fetch_events() -> Result<Vec<Event>, String> {
    let url = format!("{}/v1/events", API_BASE_URL);

    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Vec<Event>>().await {
                    Ok(events) => Ok(events),
                    Err(e) => Err(format!("Failed to parse events: {}", e)),
                }
            } else {
                Err(format!("HTTP error: {}", response.status()))
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}

// Issues API
pub async fn fetch_issues() -> Result<Vec<Issue>, String> {
    let url = format!("{}/v1/issues", API_BASE_URL);

    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Vec<Issue>>().await {
                    Ok(issues) => Ok(issues),
                    Err(e) => Err(format!("Failed to parse issues: {}", e)),
                }
            } else {
                Err(format!("HTTP error: {}", response.status()))
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}

// Projects API
pub async fn fetch_projects() -> Result<Vec<Project>, String> {
    let url = format!("{}/v1/projects", API_BASE_URL);

    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Vec<Project>>().await {
                    Ok(projects) => Ok(projects),
                    Err(e) => Err(format!("Failed to parse projects: {}", e)),
                }
            } else {
                Err(format!("HTTP error: {}", response.status()))
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}

// Blog API
pub async fn fetch_blog_posts() -> Result<Vec<BlogPost>, String> {
    let url = format!("{}/v1/blog", API_BASE_URL);

    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Vec<BlogPost>>().await {
                    Ok(posts) => Ok(posts),
                    Err(e) => Err(format!("Failed to parse blog posts: {}", e)),
                }
            } else {
                Err(format!("HTTP error: {}", response.status()))
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}

pub async fn fetch_featured_blog_posts() -> Result<Vec<BlogPost>, String> {
    let url = format!("{}/v1/blog/featured", API_BASE_URL);

    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<Vec<BlogPost>>().await {
                    Ok(posts) => Ok(posts),
                    Err(e) => Err(format!("Failed to parse featured blog posts: {}", e)),
                }
            } else {
                Err(format!("HTTP error: {}", response.status()))
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}

pub async fn fetch_blog_post_by_slug(slug: &str) -> Result<Option<BlogPost>, String> {
    let url = format!("{}/v1/blog/{}", API_BASE_URL, slug);

    match reqwest::get(&url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.json::<BlogPost>().await {
                    Ok(post) => Ok(Some(post)),
                    Err(e) => Err(format!("Failed to parse blog post: {}", e)),
                }
            } else if response.status().as_u16() == 404 {
                Ok(None)
            } else {
                Err(format!("HTTP error: {}", response.status()))
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}

pub async fn fetch_markdown_content(url: &str) -> Result<String, String> {
    match reqwest::get(url).await {
        Ok(response) => {
            if response.status().is_success() {
                match response.text().await {
                    Ok(content) => Ok(content),
                    Err(e) => Err(format!("Failed to read markdown content: {}", e)),
                }
            } else {
                Err(format!("HTTP error: {}", response.status()))
            }
        }
        Err(e) => Err(format!("Network error: {}", e)),
    }
}

// Synchronous wrapper functions for components that need immediate data
// These will use the async functions in Effect hooks

pub fn get_upcoming_events() -> Vec<Event> {
    // Return empty vec, components should use async fetch
    vec![]
}

pub fn get_issues_by_difficulty(_difficulty: DifficultyLevel) -> Vec<Issue> {
    // Return empty vec, components should use async fetch
    vec![]
}

pub fn get_club_projects() -> Vec<Project> {
    // Return empty vec, components should use async fetch
    vec![]
}