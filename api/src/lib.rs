use worker::*;
use serde_json::json;
use utoipa::OpenApi;

mod models;
mod routes;
mod database;

use models::*;

#[derive(OpenApi)]
#[openapi(
    paths(
        routes::get_stats,
        routes::get_events,
        routes::get_issues,
        routes::get_projects,
        routes::get_blog_posts,
        routes::get_featured_blog_posts,
        routes::get_blog_post_by_slug,
        routes::health_check
    ),
    components(
        schemas(Stats, Event, Issue, Project, Member, EventType, DifficultyLevel, ProjectStatus, HealthCheck, BlogPost, BlogPostType, BlogCategory, BlogSeries, ExternalLink)
    ),
    tags(
        (name = "public", description = "Public API endpoints"),
        (name = "blog", description = "Blog and tutorial endpoints"),
        (name = "health", description = "Health and status endpoints")
    ),
    info(
        title = "The Rust Club API",
        version = "1.0.0",
        description = "Official API for The Rust Club at Chico State",
        contact(
            name = "The Rust Club",
            url = "https://chico.rs"
        )
    ),
    servers(
        (url = "https://api.chico.rs", description = "Production server"),
        (url = "https://api-dev.chico.rs", description = "Development server")
    )
)]
struct ApiDoc;

fn cors_headers() -> Headers {
    let mut headers = Headers::new();
    // Allow both chico.rs domains - in production you might want to check the Origin header
    // and dynamically set this based on the request
    headers.set("Access-Control-Allow-Origin", "*").unwrap(); // Allow all origins for now
    headers.set("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS").unwrap();
    headers.set("Access-Control-Allow-Headers", "Content-Type, Authorization").unwrap();
    headers.set("Access-Control-Max-Age", "86400").unwrap();
    headers
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let router = Router::new();

    router
        // Health and info endpoints
        .get("/health", routes::health_check)
        .get("/", routes::api_info)

        // API v1 endpoints
        .get_async("/v1/stats", routes::get_stats)
        .get_async("/v1/events", routes::get_events)
        .get_async("/v1/issues", routes::get_issues)
        .get_async("/v1/projects", routes::get_projects)
        .get_async("/v1/blog", routes::get_blog_posts)
        .get_async("/v1/blog/featured", routes::get_featured_blog_posts)
        .get_async("/v1/blog/:slug", routes::get_blog_post_by_slug)

        // Documentation endpoints
        .get("/docs", routes::serve_swagger_ui)
        .get("/openapi.json", routes::serve_openapi_spec)

        // CORS preflight
        .options("/*path", |_req, _ctx| {
            Ok(Response::empty()?.with_headers(cors_headers()))
        })

        // Catch-all 404
        .or_else_any_method("/*path", |_req, _ctx| {
            let response = json!({
                "error": "Not Found",
                "message": "The requested endpoint does not exist",
                "docs": "https://api.chico.rs/docs"
            });
            Ok(Response::from_json(&response)?
                .with_status(404)
                .with_headers(cors_headers()))
        })

        .run(req, env)
        .await
        .map(|response| {
            // Add CORS headers to all responses
            response.with_headers(cors_headers())
        })
}