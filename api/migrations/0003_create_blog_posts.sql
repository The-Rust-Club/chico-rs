-- Migration: Create blog posts table
-- Created: 2025-09-17

CREATE TABLE blog_posts (
    id TEXT PRIMARY KEY,
    title TEXT NOT NULL,
    slug TEXT NOT NULL UNIQUE,
    excerpt TEXT NOT NULL,
    post_type TEXT NOT NULL CHECK (post_type IN ('tutorial', 'guide', 'show_and_tell', 'tech_talk', 'news', 'review')),
    category TEXT NOT NULL CHECK (category IN ('fundamentals', 'web_development', 'systems_programming', 'game_development', 'cli', 'data_science', 'blockchain', 'performance', 'testing', 'deployment', 'career', 'community')),
    tags TEXT NOT NULL, -- JSON array as text
    author_name TEXT NOT NULL,
    author_github TEXT,
    difficulty_level TEXT CHECK (difficulty_level IN ('easy', 'medium', 'hard')),
    estimated_read_time INTEGER NOT NULL,
    published_at TEXT NOT NULL,
    updated_at TEXT,
    views INTEGER DEFAULT 0,
    likes INTEGER DEFAULT 0,
    -- R2 storage info
    markdown_url TEXT NOT NULL, -- Full URL to storage.chico.rs or R2 resource name
    -- Series info (optional)
    series_title TEXT,
    series_part INTEGER,
    series_total_parts INTEGER,
    -- External links (JSON)
    external_links TEXT, -- JSON array as text
    created_at TEXT DEFAULT (datetime('now')),
    FOREIGN KEY (author_github) REFERENCES members(github_username)
);

CREATE INDEX idx_blog_posts_slug ON blog_posts(slug);
CREATE INDEX idx_blog_posts_post_type ON blog_posts(post_type);
CREATE INDEX idx_blog_posts_category ON blog_posts(category);
CREATE INDEX idx_blog_posts_published_at ON blog_posts(published_at);
CREATE INDEX idx_blog_posts_author ON blog_posts(author_github);
CREATE INDEX idx_blog_posts_series ON blog_posts(series_title);