use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiStats {
    pub active_members: u32,
    pub prs_merged_this_semester: u32,
    pub workshops_held: u32,
    pub projects_contributed_to: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiEvent {
    pub id: String, // UUID from API comes as string
    pub title: String,
    pub description: String,
    pub date: String,
    pub time: String,
    pub location: String,
    pub event_type: ApiEventType,
    pub recurring: bool,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiEventType {
    Workshop,
    StudyGroup,
    Seminar,
    Hackathon,
    Panel,
    Networking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiIssue {
    pub id: String, // UUID from API comes as string
    pub title: String,
    pub description: String,
    pub repo: String,
    pub github_url: String,
    pub difficulty: ApiDifficultyLevel,
    pub tags: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiDifficultyLevel {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiProject {
    pub id: String, // UUID from API comes as string
    pub name: String,
    pub description: String,
    pub github_url: String,
    pub leader: ApiMember,
    pub contributors: Vec<ApiMember>,
    pub status: ApiProjectStatus,
    pub tech_stack: Vec<String>,
    pub contributors_needed: bool,
    pub skills_needed: Vec<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiProjectStatus {
    Planning,
    Active,
    InDevelopment,
    Beta,
    Completed,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiMember {
    pub name: String,
    pub github_username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiBlogPost {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub excerpt: String,
    pub post_type: ApiBlogPostType,
    pub category: ApiBlogCategory,
    pub tags: Vec<String>,
    pub author_name: String,
    pub author_github: Option<String>,
    pub difficulty_level: Option<ApiDifficultyLevel>,
    pub estimated_read_time: u32,
    pub published_at: String,
    pub updated_at: Option<String>,
    pub views: u32,
    pub likes: u32,
    pub markdown_url: String,
    pub series: Option<ApiBlogSeries>,
    pub external_links: Vec<ApiExternalLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiBlogPostType {
    Tutorial,
    Guide,
    ShowAndTell,
    TechTalk,
    News,
    Review,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApiBlogCategory {
    Fundamentals,
    WebDevelopment,
    SystemsProgramming,
    GameDevelopment,
    CLI,
    DataScience,
    Blockchain,
    Performance,
    Testing,
    Deployment,
    Career,
    Community,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiBlogSeries {
    pub title: String,
    pub part: u32,
    pub total_parts: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiExternalLink {
    pub title: String,
    pub url: String,
}