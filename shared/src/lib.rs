use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;

#[cfg(feature = "openapi")]
use utoipa::ToSchema;

// Common enums and types shared between API and frontend

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub enum DifficultyLevel {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub enum EventType {
    Workshop,
    StudyGroup,
    Seminar,
    Hackathon,
    Panel,
    Networking,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub enum ProjectStatus {
    Planning,
    Active,
    InDevelopment,
    Beta,
    Completed,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub enum BlogPostType {
    Tutorial,
    Guide,
    ShowAndTell,
    TechTalk,
    News,
    Review,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub enum BlogCategory {
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

// Common structs

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct Member {
    /// Member's name
    pub name: String,
    /// GitHub username
    pub github_username: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct BlogSeries {
    /// Series title
    pub title: String,
    /// Current part number
    pub part: u32,
    /// Total parts in series
    pub total_parts: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct ExternalLink {
    /// Link title
    pub title: String,
    /// Link URL
    pub url: String,
}

// API Models (used for serialization between API and frontend)

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct Stats {
    /// Number of active club members
    pub active_members: u32,
    /// PRs merged this semester by club members
    pub prs_merged_this_semester: u32,
    /// Number of workshops held
    pub workshops_held: u32,
    /// Projects club members have contributed to
    pub projects_contributed_to: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct Event {
    /// Unique event identifier
    pub id: String,
    /// Event title
    pub title: String,
    /// Event description
    pub description: String,
    /// Event date
    pub date: String,
    /// Event time
    pub time: String,
    /// Event location
    pub location: String,
    /// Event type
    pub event_type: EventType,
    /// Whether this is a recurring event
    pub recurring: bool,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct Issue {
    /// Unique issue identifier
    pub id: String,
    /// Issue title
    pub title: String,
    /// Issue description
    pub description: String,
    /// Repository name
    pub repo: String,
    /// GitHub issue URL
    pub github_url: String,
    /// Issue difficulty level
    pub difficulty: DifficultyLevel,
    /// Associated tags
    pub tags: Vec<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct Project {
    /// Unique project identifier
    pub id: String,
    /// Project name
    pub name: String,
    /// Project description
    pub description: String,
    /// GitHub repository URL
    pub github_url: String,
    /// Project leader
    pub leader: Member,
    /// Project contributors
    pub contributors: Vec<Member>,
    /// Project status
    pub status: ProjectStatus,
    /// Technology stack
    pub tech_stack: Vec<String>,
    /// Whether contributors are needed
    pub contributors_needed: bool,
    /// Skills needed for contribution
    pub skills_needed: Vec<String>,
    /// Creation timestamp
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct BlogPost {
    /// Unique blog post identifier
    pub id: String,
    /// Post title
    pub title: String,
    /// URL-friendly slug
    pub slug: String,
    /// Short excerpt/summary
    pub excerpt: String,
    /// Post type
    pub post_type: BlogPostType,
    /// Post category
    pub category: BlogCategory,
    /// Associated tags
    pub tags: Vec<String>,
    /// Author information
    pub author_name: String,
    /// Author's GitHub username
    pub author_github: Option<String>,
    /// Difficulty level (optional)
    pub difficulty_level: Option<DifficultyLevel>,
    /// Estimated reading time in minutes
    pub estimated_read_time: u32,
    /// Publication date
    pub published_at: String,
    /// Last update date
    pub updated_at: Option<String>,
    /// View count
    pub views: u32,
    /// Like count
    pub likes: u32,
    /// URL to markdown content on storage.chico.rs
    pub markdown_url: String,
    /// Series information (optional)
    pub series: Option<BlogSeries>,
    /// External links
    pub external_links: Vec<ExternalLink>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "openapi", derive(ToSchema))]
pub struct HealthCheck {
    /// Service status
    pub status: String,
    /// Current timestamp
    pub timestamp: DateTime<Utc>,
    /// API version
    pub version: String,
}

// Display implementations for enums
impl fmt::Display for DifficultyLevel {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            DifficultyLevel::Easy => write!(f, "Easy"),
            DifficultyLevel::Medium => write!(f, "Medium"),
            DifficultyLevel::Hard => write!(f, "Hard"),
        }
    }
}

impl fmt::Display for EventType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            EventType::Workshop => write!(f, "Workshop"),
            EventType::StudyGroup => write!(f, "Study Group"),
            EventType::Seminar => write!(f, "Seminar"),
            EventType::Hackathon => write!(f, "Hackathon"),
            EventType::Panel => write!(f, "Panel"),
            EventType::Networking => write!(f, "Networking"),
        }
    }
}

impl fmt::Display for BlogPostType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BlogPostType::Tutorial => write!(f, "Tutorial"),
            BlogPostType::Guide => write!(f, "Guide"),
            BlogPostType::ShowAndTell => write!(f, "Show & Tell"),
            BlogPostType::TechTalk => write!(f, "Tech Talk"),
            BlogPostType::News => write!(f, "News"),
            BlogPostType::Review => write!(f, "Review"),
        }
    }
}

impl fmt::Display for ProjectStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ProjectStatus::Planning => write!(f, "Planning"),
            ProjectStatus::Active => write!(f, "Active"),
            ProjectStatus::InDevelopment => write!(f, "In Development"),
            ProjectStatus::Beta => write!(f, "Beta"),
            ProjectStatus::Completed => write!(f, "Completed"),
            ProjectStatus::Archived => write!(f, "Archived"),
        }
    }
}

impl fmt::Display for BlogCategory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            BlogCategory::Fundamentals => write!(f, "Fundamentals"),
            BlogCategory::WebDevelopment => write!(f, "Web Development"),
            BlogCategory::SystemsProgramming => write!(f, "Systems Programming"),
            BlogCategory::GameDevelopment => write!(f, "Game Development"),
            BlogCategory::CLI => write!(f, "CLI"),
            BlogCategory::DataScience => write!(f, "Data Science"),
            BlogCategory::Blockchain => write!(f, "Blockchain"),
            BlogCategory::Performance => write!(f, "Performance"),
            BlogCategory::Testing => write!(f, "Testing"),
            BlogCategory::Deployment => write!(f, "Deployment"),
            BlogCategory::Career => write!(f, "Career"),
            BlogCategory::Community => write!(f, "Community"),
        }
    }
}