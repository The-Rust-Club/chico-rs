-- Simple blog post insert without foreign key dependencies
INSERT INTO blog_posts (
    id,
    title,
    slug,
    excerpt,
    post_type,
    category,
    tags,
    author_name,
    author_github,
    difficulty_level,
    estimated_read_time,
    published_at,
    views,
    likes,
    markdown_url
) VALUES (
    'blog-getting-started-rust-2024',
    'Getting Started with Rust: Your Journey Begins Here',
    'getting-started-with-rust',
    'A comprehensive guide to setting up your Rust development environment and writing your first program. Perfect for beginners looking to start their Rust journey.',
    'tutorial',
    'fundamentals',
    '["beginner", "setup", "environment", "first-program", "rustup", "cargo"]',
    'The Rust Club',
    'werdxz',
    'easy',
    12,
    '2024-12-17T10:00:00Z',
    0,
    0,
    'https://storage.chico.rs/blog/getting-started-with-rust.md'
);