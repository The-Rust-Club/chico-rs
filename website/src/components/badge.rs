use leptos::prelude::*;
use crate::api::DifficultyLevel;

stylance::import_style!(style, "badge.module.scss");

#[component]
pub fn Badge(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] variant: Option<BadgeVariant>,
    #[prop(optional)] size: Option<BadgeSize>,
    children: Children,
) -> impl IntoView {
    let badge_class = {
        let mut classes = vec![style::badge];
        
        if let Some(variant) = variant {
            classes.push(match variant {
                BadgeVariant::Primary => style::badge_primary,
                BadgeVariant::Secondary => style::badge_secondary,
                BadgeVariant::Success => style::badge_success,
                BadgeVariant::Warning => style::badge_warning,
                BadgeVariant::Error => style::badge_error,
                BadgeVariant::Info => style::badge_info,
                BadgeVariant::Outline => style::badge_outline,
            });
        }
        
        if let Some(size) = size {
            classes.push(match size {
                BadgeSize::Small => style::badge_sm,
                BadgeSize::Medium => style::badge_md,
                BadgeSize::Large => style::badge_lg,
            });
        }
        
        if let Some(extra_class) = class {
            format!("{} {}", classes.join(" "), extra_class)
        } else {
            classes.join(" ")
        }
    };

    view! {
        <span class={badge_class}>
            {children()}
        </span>
    }
}

#[component]
pub fn DifficultyBadge(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] size: Option<BadgeSize>,
    level: DifficultyLevel,
) -> impl IntoView {
    let (text, variant_class) = match level {
        DifficultyLevel::Easy => ("Beginner", style::difficulty_easy),
        DifficultyLevel::Medium => ("Intermediate", style::difficulty_medium),
        DifficultyLevel::Hard => ("Advanced", style::difficulty_hard),
    };
    
    let size_class = if let Some(size) = size {
        match size {
            BadgeSize::Small => style::badge_sm,
            BadgeSize::Medium => style::badge_md,
            BadgeSize::Large => style::badge_lg,
        }
    } else {
        style::badge_md
    };
    
    let badge_class = if let Some(extra_class) = class {
        format!("{} {} {} {} {}", style::badge, style::difficulty_badge, variant_class, size_class, extra_class)
    } else {
        format!("{} {} {} {}", style::badge, style::difficulty_badge, variant_class, size_class)
    };

    view! {
        <span class={badge_class}>
            {text}
        </span>
    }
}

#[component]
pub fn PostTypeBadge(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] size: Option<BadgeSize>,
    post_type: String,
) -> impl IntoView {
    let normalized_type = post_type.to_lowercase().replace(" ", "-").replace("&", "and");
    
    let type_class = match normalized_type.as_str() {
        "tutorial" => style::type_tutorial,
        "guide" => style::type_guide,
        "show-and-tell" => style::type_show_and_tell,
        "tech-talk" => style::type_tech_talk,
        "news" => style::type_news,
        "review" => style::type_review,
        _ => style::type_tutorial,
    };
    
    let size_class = if let Some(size) = size {
        match size {
            BadgeSize::Small => style::badge_sm,
            BadgeSize::Medium => style::badge_md,
            BadgeSize::Large => style::badge_lg,
        }
    } else {
        style::badge_md
    };
    
    let badge_class = if let Some(extra_class) = class {
        format!("{} {} {} {} {}", style::badge, style::post_type_badge, type_class, size_class, extra_class)
    } else {
        format!("{} {} {} {}", style::badge, style::post_type_badge, type_class, size_class)
    };

    view! {
        <span class={badge_class}>
            {post_type}
        </span>
    }
}

#[component]
pub fn Tag(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] variant: Option<TagVariant>,
    #[prop(optional)] size: Option<BadgeSize>,
    children: Children,
) -> impl IntoView {
    let mut classes = vec![style::tag];
    
    if let Some(variant) = variant {
        classes.push(match variant {
            TagVariant::Default => style::tag_default,
            TagVariant::Outline => style::tag_outline,
            TagVariant::Solid => style::tag_solid,
            TagVariant::Ghost => style::tag_ghost,
        });
    }
    
    if let Some(size) = size {
        classes.push(match size {
            BadgeSize::Small => style::badge_sm,
            BadgeSize::Medium => style::badge_md,
            BadgeSize::Large => style::badge_lg,
        });
    }
    
    let tag_class = if let Some(extra_class) = class {
        format!("{} {}", classes.join(" "), extra_class)
    } else {
        classes.join(" ")
    };

    view! {
        <span class={tag_class}>
            {children()}
        </span>
    }
}

#[derive(Clone, Copy)]
pub enum BadgeVariant {
    Primary,
    Secondary,
    Success,
    Warning,
    Error,
    Info,
    Outline,
}

#[derive(Clone, Copy)]
pub enum BadgeSize {
    Small,
    Medium,
    Large,
}

#[derive(Clone, Copy)]
pub enum TagVariant {
    Default,
    Outline,
    Solid,
    Ghost,
}