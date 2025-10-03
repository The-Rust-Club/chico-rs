use leptos::prelude::*;
use lucide_leptos::*;

stylance::import_style!(style, "nav.module.scss");

#[component]
pub fn Nav() -> impl IntoView {
    let (is_mobile_menu_open, set_mobile_menu_open) = signal(false);
    
    let toggle_mobile_menu = move |_| {
        set_mobile_menu_open.update(|open| *open = !*open);
    };
    
    let close_mobile_menu = move |_| {
        set_mobile_menu_open.set(false);
    };

    view! {
        <nav class={style::navbar}>
            <div class={style::nav_container}>
                <a href="/" class={style::nav_logo}>
                    <Code />
                    <span class={style::nav_title}>"The Rust Club"</span>
                </a>
                
                // Desktop menu
                <ul class={style::nav_menu}>
                    <li class={style::nav_item}>
                        <a href="/" class={style::nav_link}>"Home"</a>
                    </li>
                    <li class={style::nav_item}>
                        <a href="/blog" class={style::nav_link}>"Blog"</a>
                    </li>
                    <li class={style::nav_item}>
                        <a href="/opensource" class={style::nav_link}>"Open Source"</a>
                    </li>
                    <li class={style::nav_item}>
                        <a href="/workshops" class={style::nav_link}>"Workshops"</a>
                    </li>
                    <li class={style::nav_item}>
                        <a href="/events" class={style::nav_link}>"Events"</a>
                    </li>
                    <li class={style::nav_item}>
                        <a href="/projects" class={style::nav_link}>"Projects"</a>
                    </li>
                    <li class={style::nav_item}>
                        <a href="/advocacy" class={style::nav_link}>"Advocacy"</a>
                    </li>
                    <li class={style::nav_item}>
                        <a href="/about" class={style::nav_link}>"About"</a>
                    </li>
                </ul>
                
                // Mobile menu button
                <button 
                    class={style::mobile_menu_button}
                    on:click=toggle_mobile_menu
                    aria-label="Toggle navigation menu"
                >
                    {move || if is_mobile_menu_open.get() {
                        view! { <X /> }.into_any()
                    } else {
                        view! { <Menu /> }.into_any()
                    }}
                </button>
            </div>
            
            // Mobile menu overlay
            <div 
                class={move || if is_mobile_menu_open.get() { 
                    format!("{} {}", style::mobile_menu_overlay, style::mobile_menu_open)
                } else { 
                    style::mobile_menu_overlay.to_string() 
                }}
                on:click=close_mobile_menu
            >
                <ul class={style::mobile_menu}>
                    <li class={style::mobile_nav_item}>
                        <a href="/" class={style::mobile_nav_link} on:click=close_mobile_menu>"Home"</a>
                    </li>
                    <li class={style::mobile_nav_item}>
                        <a href="/blog" class={style::mobile_nav_link} on:click=close_mobile_menu>"Blog"</a>
                    </li>
                    <li class={style::mobile_nav_item}>
                        <a href="/opensource" class={style::mobile_nav_link} on:click=close_mobile_menu>"Open Source"</a>
                    </li>
                    <li class={style::mobile_nav_item}>
                        <a href="/workshops" class={style::mobile_nav_link} on:click=close_mobile_menu>"Workshops"</a>
                    </li>
                    <li class={style::mobile_nav_item}>
                        <a href="/events" class={style::mobile_nav_link} on:click=close_mobile_menu>"Events"</a>
                    </li>
                    <li class={style::mobile_nav_item}>
                        <a href="/projects" class={style::mobile_nav_link} on:click=close_mobile_menu>"Projects"</a>
                    </li>
                    <li class={style::mobile_nav_item}>
                        <a href="/advocacy" class={style::mobile_nav_link} on:click=close_mobile_menu>"Advocacy"</a>
                    </li>
                    <li class={style::mobile_nav_item}>
                        <a href="/about" class={style::mobile_nav_link} on:click=close_mobile_menu>"About"</a>
                    </li>
                </ul>
            </div>
        </nav>
    }
}
