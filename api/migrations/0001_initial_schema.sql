-- Initial schema for The Rust Club API
-- Run with: wrangler d1 execute chico-rs-db --file=./migrations/0001_initial_schema.sql

-- Members table
CREATE TABLE members (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    github_username TEXT UNIQUE,
    email TEXT,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Events table
CREATE TABLE events (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT UNIQUE NOT NULL, -- For public API
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    date TEXT NOT NULL, -- Store as text for flexibility
    time TEXT NOT NULL,
    location TEXT NOT NULL,
    event_type TEXT NOT NULL CHECK (event_type IN ('Workshop', 'StudyGroup', 'Seminar', 'Hackathon', 'Panel', 'Networking')),
    recurring BOOLEAN DEFAULT FALSE,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Issues table
CREATE TABLE issues (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT UNIQUE NOT NULL, -- For public API
    title TEXT NOT NULL,
    description TEXT NOT NULL,
    repo TEXT NOT NULL,
    github_url TEXT NOT NULL UNIQUE,
    difficulty TEXT NOT NULL CHECK (difficulty IN ('Easy', 'Medium', 'Hard')),
    tags TEXT NOT NULL, -- JSON array as text
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Projects table
CREATE TABLE projects (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    uuid TEXT UNIQUE NOT NULL, -- For public API
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    github_url TEXT NOT NULL UNIQUE,
    leader_id INTEGER NOT NULL,
    status TEXT NOT NULL CHECK (status IN ('Planning', 'Active', 'InDevelopment', 'Beta', 'Completed', 'Archived')),
    tech_stack TEXT NOT NULL, -- JSON array as text
    contributors_needed BOOLEAN DEFAULT FALSE,
    skills_needed TEXT NOT NULL, -- JSON array as text
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (leader_id) REFERENCES members (id)
);

-- Project contributors junction table
CREATE TABLE project_contributors (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    project_id INTEGER NOT NULL,
    member_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (project_id) REFERENCES projects (id) ON DELETE CASCADE,
    FOREIGN KEY (member_id) REFERENCES members (id) ON DELETE CASCADE,
    UNIQUE(project_id, member_id)
);

-- Indexes for better query performance
CREATE INDEX idx_events_date ON events(date);
CREATE INDEX idx_events_type ON events(event_type);
CREATE INDEX idx_events_recurring ON events(recurring);

CREATE INDEX idx_issues_difficulty ON issues(difficulty);
CREATE INDEX idx_issues_repo ON issues(repo);

CREATE INDEX idx_projects_status ON projects(status);
CREATE INDEX idx_projects_contributors_needed ON projects(contributors_needed);
CREATE INDEX idx_projects_leader ON projects(leader_id);

CREATE INDEX idx_project_contributors_project ON project_contributors(project_id);
CREATE INDEX idx_project_contributors_member ON project_contributors(member_id);

CREATE INDEX idx_members_github ON members(github_username);