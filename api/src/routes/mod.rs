use worker::*;
use serde_json::json;
use chrono::Utc;
use utoipa::OpenApi;

use crate::models::*;
use crate::database::DatabaseService;
use crate::ApiDoc;

/// Get club statistics
#[utoipa::path(
    get,
    path = "/v1/stats",
    responses(
        (status = 200, description = "Club statistics", body = Stats)
    ),
    tag = "public"
)]
pub async fn get_stats(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let kv = ctx.env.kv("CACHE")?;
    let db = ctx.env.d1("DB")?;

    // Try to get stats from KV cache first
    if let Ok(Some(cached_stats)) = kv.get("club_stats").text().await {
        if let Ok(stats) = serde_json::from_str::<Stats>(&cached_stats) {
            return Response::from_json(&stats);
        }
    }

    // If not in cache, calculate from D1 and cache it
    let member_count = DatabaseService::get_member_count(&db).await.unwrap_or(0);

    let stats = Stats {
        active_members: member_count,
        prs_merged_this_semester: 47, // TODO: Get from GitHub API or manual input
        workshops_held: 12, // TODO: Count from events table
        projects_contributed_to: 8, // TODO: Count from projects table
    };

    // Cache for 1 hour
    if let Ok(stats_json) = serde_json::to_string(&stats) {
        let _ = kv.put("club_stats", stats_json)?.expiration_ttl(3600).execute().await;
    }

    Response::from_json(&stats)
}

/// Get upcoming events
#[utoipa::path(
    get,
    path = "/v1/events",
    responses(
        (status = 200, description = "List of upcoming events", body = [Event])
    ),
    tag = "public"
)]
pub async fn get_events(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;

    match DatabaseService::get_events(&db).await {
        Ok(events) => Response::from_json(&events),
        Err(e) => {
            // Log the error for debugging
            web_sys::console::error_1(&format!("Failed to fetch events: {:?}", e).into());

            let error = json!({
                "error": "Internal Server Error",
                "message": "Unable to fetch events at this time"
            });
            Ok(Response::from_json(&error)?.with_status(500))
        }
    }
}

/// Get good first issues
#[utoipa::path(
    get,
    path = "/v1/issues",
    responses(
        (status = 200, description = "List of good first issues", body = [Issue])
    ),
    tag = "public"
)]
pub async fn get_issues(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;

    match DatabaseService::get_issues(&db).await {
        Ok(issues) => Response::from_json(&issues),
        Err(e) => {
            web_sys::console::error_1(&format!("Failed to fetch issues: {:?}", e).into());

            let error = json!({
                "error": "Internal Server Error",
                "message": "Unable to fetch issues at this time"
            });
            Ok(Response::from_json(&error)?.with_status(500))
        }
    }
}

/// Get student projects
#[utoipa::path(
    get,
    path = "/v1/projects",
    responses(
        (status = 200, description = "List of student projects", body = [Project])
    ),
    tag = "public"
)]
pub async fn get_projects(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;

    match DatabaseService::get_projects(&db).await {
        Ok(projects) => Response::from_json(&projects),
        Err(e) => {
            web_sys::console::error_1(&format!("Failed to fetch projects: {:?}", e).into());

            let error = json!({
                "error": "Internal Server Error",
                "message": "Unable to fetch projects at this time"
            });
            Ok(Response::from_json(&error)?.with_status(500))
        }
    }
}

/// Health check endpoint
#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Service health status", body = HealthCheck)
    ),
    tag = "health"
)]
pub fn health_check(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let health = HealthCheck {
        status: "healthy".to_string(),
        timestamp: Utc::now(),
        version: "v1".to_string(),
    };

    Response::from_json(&health)
}

/// API information endpoint
pub fn api_info(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let info = json!({
        "name": "The Rust Club API",
        "version": "1.0.0",
        "description": "Official API for The Rust Club at Chico State",
        "docs": "https://api.chico.rs/docs",
        "endpoints": {
            "stats": "/v1/stats",
            "events": "/v1/events",
            "issues": "/v1/issues",
            "projects": "/v1/projects",
            "blog": "/v1/blog",
            "featured_blog": "/v1/blog/featured"
        },
        "repository": "https://github.com/rust-club/chico-rs"
    });

    Response::from_json(&info)
}

/// Serve Swagger UI
pub fn serve_swagger_ui(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let html = include_str!("../../static/swagger-ui.html");
    Response::from_html(html)
}

/// Serve OpenAPI specification
pub fn serve_openapi_spec(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let spec = ApiDoc::openapi();
    Response::from_json(&spec)
}

/// Get all blog posts
#[utoipa::path(
    get,
    path = "/v1/blog",
    responses(
        (status = 200, description = "List of blog posts", body = [BlogPost])
    ),
    tag = "blog"
)]
pub async fn get_blog_posts(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let db = ctx.env.d1("DB")?;

    match DatabaseService::get_blog_posts(&db).await {
        Ok(posts) => Response::from_json(&posts),
        Err(e) => {
            web_sys::console::error_1(&format!("Failed to fetch blog posts: {:?}", e).into());

            let error = json!({
                "error": "Internal Server Error",
                "message": "Unable to fetch blog posts at this time"
            });
            Ok(Response::from_json(&error)?.with_status(500))
        }
    }
}

/// Get featured blog posts
#[utoipa::path(
    get,
    path = "/v1/blog/featured",
    responses(
        (status = 200, description = "List of featured blog posts", body = [BlogPost])
    ),
    tag = "blog"
)]
pub async fn get_featured_blog_posts(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let kv = ctx.env.kv("CACHE")?;
    let db = ctx.env.d1("DB")?;

    // Get featured post IDs from KV
    match kv.get("featured_blog_posts").text().await {
        Ok(Some(featured_ids_json)) => {
            if let Ok(featured_ids) = serde_json::from_str::<Vec<String>>(&featured_ids_json) {
                match DatabaseService::get_blog_posts_by_ids(&db, &featured_ids).await {
                    Ok(posts) => Response::from_json(&posts),
                    Err(e) => {
                        web_sys::console::error_1(&format!("Failed to fetch featured posts: {:?}", e).into());

                        let error = json!({
                            "error": "Internal Server Error",
                            "message": "Unable to fetch featured posts at this time"
                        });
                        Ok(Response::from_json(&error)?.with_status(500))
                    }
                }
            } else {
                // Invalid JSON in KV, return empty array
                Response::from_json(&Vec::<BlogPost>::new())
            }
        }
        _ => {
            // No featured posts configured, return empty array
            Response::from_json(&Vec::<BlogPost>::new())
        }
    }
}

/// Get single blog post by slug
#[utoipa::path(
    get,
    path = "/v1/blog/{slug}",
    params(
        ("slug" = String, Path, description = "Blog post slug")
    ),
    responses(
        (status = 200, description = "Blog post", body = BlogPost),
        (status = 404, description = "Blog post not found")
    ),
    tag = "blog"
)]
pub async fn get_blog_post_by_slug(_req: Request, ctx: RouteContext<()>) -> Result<Response> {
    let slug = ctx.param("slug").map_or("", |v| v);
    let db = ctx.env.d1("DB")?;

    match DatabaseService::get_blog_post_by_slug(&db, slug).await {
        Ok(Some(post)) => {
            // Increment view count
            let _ = DatabaseService::increment_blog_post_views(&db, &post.id).await;
            Response::from_json(&post)
        }
        Ok(None) => {
            let error = json!({
                "error": "Not Found",
                "message": "Blog post not found"
            });
            Ok(Response::from_json(&error)?.with_status(404))
        }
        Err(e) => {
            web_sys::console::error_1(&format!("Failed to fetch blog post: {:?}", e).into());

            let error = json!({
                "error": "Internal Server Error",
                "message": "Unable to fetch blog post at this time"
            });
            Ok(Response::from_json(&error)?.with_status(500))
        }
    }
}