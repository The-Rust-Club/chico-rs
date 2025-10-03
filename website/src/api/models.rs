use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Event {
    pub id: String,
    pub title: String,
    pub description: String,
    pub event_type: EventType,
    pub date: String,
    pub time: String,
    pub location: String,
    pub recurring: bool,
    pub instructor: Option<String>,
    pub topics_covered: Vec<String>,
    pub difficulty_level: Option<DifficultyLevel>,
    pub max_participants: Option<u32>,
    pub registration_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EventType {
    Workshop,
    StudyGroup,
    Seminar,
    Hackathon,
    Panel,
    Networking,
}

impl std::fmt::Display for EventType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub title: String,
    pub description: String,
    pub repo_name: String,
    pub repo_url: String,
    pub github_url: String,
    pub difficulty: DifficultyLevel,
    pub tags: Vec<String>,
    pub field_tags: Vec<FieldTag>,
    pub estimated_hours: Option<u32>,
    pub language: String,
    pub good_first_issue: bool,
    pub mentorship_available: bool,
    pub created_at: String,
    pub updated_at: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
pub enum DifficultyLevel {
    Easy,
    Medium,
    Hard,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FieldTag {
    WebDevelopment,
    CLI,
    SystemsProgramming,
    GameDevelopment,
    MachineLearning,
    Blockchain,
    Networking,
    Database,
    Graphics,
    Audio,
    Documentation,
    Testing,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub id: String,
    pub name: String,
    pub description: String,
    pub project_type: ProjectType,
    pub status: ProjectStatus,
    pub tech_stack: Vec<String>,
    pub github_url: Option<String>,
    pub club_owned: bool,
    pub leader: Contributor,
    pub contributors: Vec<Contributor>,
    pub skills_needed: Vec<String>,
    pub contributors_needed: bool,
    pub created_at: String,
    pub last_updated: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectType {
    Project,
    Research,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProjectStatus {
    Planning,
    InDevelopment,
    Beta,
    Active,
    Completed,
    OnHold,
    Archived,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Contributor {
    pub id: String,
    pub name: String,
    pub github_username: Option<String>,
    pub role: ContributorRole,
    pub bio: Option<String>,
    pub skills: Vec<String>,
    pub avatar_url: Option<String>,
    pub joined_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ContributorRole {
    President,
    VicePresident,
    Secretary,
    Treasurer,
    ProjectLead,
    Mentor,
    Member,
    Alumni,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub total_members: u32,
    pub active_members: u32,
    pub prs_merged_this_semester: u32,
    pub first_time_contributors: u32,
    pub projects_contributed_to: u32,
    pub workshops_held: u32,
    pub study_sessions_this_month: u32,
    pub discord_members: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Course {
    pub course_code: String,
    pub course_name: String,
    pub professor: String,
    pub status: CourseStatus,
    pub supporters_count: u32,
    pub semester: String,
    pub description: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CourseStatus {
    Approved,
    UnderReview,
    Pending,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workshop {
    pub id: String,
    pub title: String,
    pub description: String,
    pub level: DifficultyLevel,
    pub date: String,
    pub time: String,
    pub location: String,
    pub instructor: String,
    pub topics: Vec<String>,
    pub prerequisites: Vec<String>,
    pub materials_url: Option<String>,
    pub registration_url: Option<String>,
    pub max_participants: u32,
    pub current_participants: u32,
    pub featured: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogPost {
    pub id: String,
    pub title: String,
    pub slug: String,
    pub excerpt: String,
    pub content: String,
    pub post_type: BlogPostType,
    pub category: BlogCategory,
    pub tags: Vec<String>,
    pub author: Contributor,
    pub co_authors: Vec<Contributor>,
    pub difficulty_level: Option<DifficultyLevel>,
    pub estimated_read_time: u32, // minutes
    pub published_at: String,
    pub updated_at: Option<String>,
    pub featured: bool,
    pub series: Option<BlogSeries>,
    pub code_examples: Vec<CodeExample>,
    pub external_links: Vec<ExternalLink>,
    pub views: Option<u32>,
    pub likes: Option<u32>,
    pub markdown_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BlogPostType {
    Tutorial,
    Guide,
    ShowAndTell, // Member project showcases
    TechTalk,    // Event summaries
    News,        // Club announcements
    Review,      // Book/resource reviews
}

#[derive(Debug, Clone, Serialize, Deserialize)]
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

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlogSeries {
    pub id: String,
    pub title: String,
    pub description: String,
    pub part_number: u32,
    pub total_parts: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    pub id: String,
    pub title: String,
    pub description: Option<String>,
    pub language: String,
    pub code: String,
    pub github_url: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExternalLink {
    pub title: String,
    pub url: String,
    pub description: Option<String>,
    pub link_type: LinkType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LinkType {
    Documentation,
    Repository,
    Article,
    Video,
    Tool,
    Reference,
}

impl std::fmt::Display for BlogPostType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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

impl std::fmt::Display for BlogCategory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
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