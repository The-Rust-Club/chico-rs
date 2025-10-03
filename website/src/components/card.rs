use leptos::prelude::*;

stylance::import_style!(style, "card.module.scss");

#[component]
pub fn Card(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] variant: Option<CardVariant>,
    #[prop(optional)] size: Option<CardSize>,
    #[prop(optional)] elevated: bool,
    children: Children,
) -> impl IntoView {
    let card_class = {
        let mut classes = vec![style::card];
        
        if let Some(variant) = variant {
            classes.push(match variant {
                CardVariant::Default => style::card_default,
                CardVariant::Featured => style::card_featured,
                CardVariant::Compact => style::card_compact,
            });
        }
        
        if let Some(size) = size {
            classes.push(match size {
                CardSize::Small => style::card_sm,
                CardSize::Medium => style::card_md,
                CardSize::Large => style::card_lg,
            });
        }
        
        if elevated {
            classes.push(style::card_elevated);
        }
        
        if let Some(extra_class) = class {
            format!("{} {}", classes.join(" "), extra_class)
        } else {
            classes.join(" ")
        }
    };

    view! {
        <div class={card_class}>
            {children()}
        </div>
    }
}

#[component]
pub fn CardHeader(
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let header_class = if let Some(extra_class) = class {
        format!("{} {}", style::card_header, extra_class)
    } else {
        style::card_header.to_string()
    };

    view! {
        <div class={header_class}>
            {children()}
        </div>
    }
}

#[component]
pub fn CardBody(
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let body_class = if let Some(extra_class) = class {
        format!("{} {}", style::card_body, extra_class)
    } else {
        style::card_body.to_string()
    };

    view! {
        <div class={body_class}>
            {children()}
        </div>
    }
}

#[component]
pub fn CardFooter(
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let footer_class = if let Some(extra_class) = class {
        format!("{} {}", style::card_footer, extra_class)
    } else {
        style::card_footer.to_string()
    };

    view! {
        <div class={footer_class}>
            {children()}
        </div>
    }
}

#[component]
pub fn CardTitle(
    #[prop(optional)] class: Option<String>,
    #[prop(optional)] level: Option<u8>,
    children: Children,
) -> impl IntoView {
    let title_class = if let Some(extra_class) = class {
        format!("{} {}", style::card_title, extra_class)
    } else {
        style::card_title.to_string()
    };

    let level = level.unwrap_or(3);

    match level {
        1 => view! { <h1 class={title_class}>{children()}</h1> }.into_any(),
        2 => view! { <h2 class={title_class}>{children()}</h2> }.into_any(),
        3 => view! { <h3 class={title_class}>{children()}</h3> }.into_any(),
        4 => view! { <h4 class={title_class}>{children()}</h4> }.into_any(),
        5 => view! { <h5 class={title_class}>{children()}</h5> }.into_any(),
        _ => view! { <h6 class={title_class}>{children()}</h6> }.into_any(),
    }
}

#[component]
pub fn CardText(
    #[prop(optional)] class: Option<String>,
    children: Children,
) -> impl IntoView {
    let text_class = if let Some(extra_class) = class {
        format!("{} {}", style::card_text, extra_class)
    } else {
        style::card_text.to_string()
    };

    view! {
        <p class={text_class}>
            {children()}
        </p>
    }
}

#[derive(Clone, Copy)]
pub enum CardVariant {
    Default,
    Featured,
    Compact,
}

#[derive(Clone, Copy)]
pub enum CardSize {
    Small,
    Medium,
    Large,
}