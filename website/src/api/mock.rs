use shared::*;

// Synchronous wrapper functions for components that need immediate data
// These return empty data, components should use async fetch functions from client.rs

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

pub fn get_issues() -> Vec<Issue> {
    // Return empty vec, components should use async fetch
    vec![]
}

pub fn get_events() -> Vec<Event> {
    // Return empty vec, components should use async fetch
    vec![]
}

pub fn get_blog_posts() -> Vec<BlogPost> {
    // Return empty vec, components should use async fetch
    vec![]
}

pub fn get_featured_blog_posts() -> Vec<BlogPost> {
    // Return empty vec, components should use async fetch
    vec![]
}