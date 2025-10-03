-- Seed data for The Rust Club API
-- Run with: wrangler d1 execute chico-rs-db --file=./migrations/0002_seed_data.sql

-- Insert sample members
INSERT INTO members (name, github_username, email) VALUES
('Alex Chen', 'alexcodes', 'alex@example.com'),
('Jordan Smith', 'jordandev', 'jordan@example.com'),
('Sam Wilson', 'samw', 'sam@example.com'),
('Taylor Rodriguez', 'taylorr', 'taylor@example.com'),
('Casey Johnson', 'caseyjay', 'casey@example.com'),
('Morgan Park', 'morganp', 'morgan@example.com'),
('Riley Davis', 'rileyd', 'riley@example.com'),
('Avery Thompson', 'averyt', 'avery@example.com'),
('Quinn Lee', 'quinnl', 'quinn@example.com');

-- Insert sample events
INSERT INTO events (uuid, title, description, date, time, location, event_type, recurring) VALUES
('550e8400-e29b-41d4-a716-446655440001',
 'Spring 2025 Workshop #1: Rust Fundamentals',
 'Learn Rust fundamentals including ownership, borrowing, and building your first CLI tool. Perfect for beginners!',
 'February 15', '1:00 PM - 5:00 PM', 'Engineering Building, Room 301', 'Workshop', FALSE),

('550e8400-e29b-41d4-a716-446655440002',
 'Weekly Study Group',
 'Join us for collaborative learning and project work. Bring your Rust questions!',
 'Every Friday', '3:00 PM - 5:00 PM', 'Library, Study Room B', 'StudyGroup', TRUE),

('550e8400-e29b-41d4-a716-446655440003',
 'Industry Guest Speaker: WebAssembly in Production',
 'Learn how major companies are using Rust and WebAssembly in production environments.',
 'March 8', '6:00 PM - 8:00 PM', 'Auditorium A, Student Center', 'Seminar', FALSE);

-- Insert sample issues
INSERT INTO issues (uuid, title, description, repo, github_url, difficulty, tags) VALUES
('550e8400-e29b-41d4-a716-446655440010',
 'Add documentation for async patterns',
 'We need comprehensive documentation covering async/await patterns in Rust. This would help newcomers understand concurrent programming.',
 'rust-lang/reference', 'https://github.com/rust-lang/reference/issues/123', 'Easy',
 '["documentation", "async", "good-first-issue"]'),

('550e8400-e29b-41d4-a716-446655440011',
 'Improve error message for trait bound errors',
 'Current error messages for complex trait bounds can be confusing. We need clearer, more actionable error messages.',
 'rust-lang/rust', 'https://github.com/rust-lang/rust/issues/456', 'Medium',
 '["diagnostics", "error-messages", "good-first-issue"]'),

('550e8400-e29b-41d4-a716-446655440012',
 'Add more examples to std::collections docs',
 'The collections module could benefit from more practical examples showing real-world usage patterns.',
 'rust-lang/rust', 'https://github.com/rust-lang/rust/issues/789', 'Easy',
 '["documentation", "std", "examples"]'),

('550e8400-e29b-41d4-a716-446655440013',
 'Implement Display for more error types',
 'Several error types in the ecosystem are missing Display implementations, making debugging harder.',
 'tokio-rs/tokio', 'https://github.com/tokio-rs/tokio/issues/101', 'Medium',
 '["error-handling", "display", "good-first-issue"]'),

('550e8400-e29b-41d4-a716-446655440014',
 'Add benchmarks for sorting algorithms',
 'We need comprehensive benchmarks comparing different sorting implementations to guide optimization efforts.',
 'rust-lang/rust', 'https://github.com/rust-lang/rust/issues/202', 'Hard',
 '["performance", "benchmarks", "algorithms"]'),

('550e8400-e29b-41d4-a716-446655440015',
 'Fix clippy lint false positive',
 'The `unnecessary_wraps` lint is triggering false positives in certain generic contexts.',
 'rust-lang/rust-clippy', 'https://github.com/rust-lang/rust-clippy/issues/303', 'Medium',
 '["clippy", "false-positive", "good-first-issue"]');

-- Insert sample projects
INSERT INTO projects (uuid, name, description, github_url, leader_id, status, tech_stack, contributors_needed, skills_needed) VALUES
('550e8400-e29b-41d4-a716-446655440020',
 'The Rust Club Website',
 'Official website for The Rust Club built with Leptos and deployed on Cloudflare Pages.',
 'https://github.com/rust-club/website', 1, 'Active',
 '["Leptos", "Trunk", "CSS", "Cloudflare"]', TRUE,
 '["Frontend", "CSS", "Design"]'),

('550e8400-e29b-41d4-a716-446655440021',
 'RustBot Discord Bot',
 'A Discord bot for The Rust Club server with moderation, event management, and learning resources.',
 'https://github.com/rust-club/rustbot', 4, 'InDevelopment',
 '["Rust", "Serenity", "SQLite"]', TRUE,
 '["Backend", "Discord API", "Database"]'),

('550e8400-e29b-41d4-a716-446655440022',
 'Rust Learning CLI',
 'Interactive CLI tool to help students learn Rust concepts through hands-on exercises and quizzes.',
 'https://github.com/rust-club/learn-rust-cli', 6, 'Beta',
 '["Rust", "Clap", "Crossterm"]', FALSE,
 '[]'),

('550e8400-e29b-41d4-a716-446655440023',
 'Campus Event Scraper',
 'Web scraper that aggregates tech events from various campus sources into a unified calendar.',
 'https://github.com/rust-club/event-scraper', 9, 'Planning',
 '["Rust", "Scraper", "Tokio", "SQLite"]', TRUE,
 '["Web Scraping", "Async Rust", "Database"]');

-- Insert project contributors
INSERT INTO project_contributors (project_id, member_id) VALUES
-- Website contributors
(1, 2), (1, 3),
-- Discord bot contributors
(2, 5),
-- CLI tool contributors
(3, 7), (3, 8);