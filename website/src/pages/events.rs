use leptos::prelude::*;
use lucide_leptos::{Clock, MapPin, Repeat};
use crate::api::get_events;

stylance::import_style!(style, "events.module.scss");


#[component]
pub fn Events() -> impl IntoView {
    let events = get_events();

    view! {
        <div class={style::page_container}>
            <div class={style::page_header}>
                <h1>"Events & Seminars"</h1>
                <p class={style::page_subtitle}>
                    "Regular meetups, study groups, and special events to keep you engaged with Rust. "
                    "Learn together, build together, grow together."
                </p>
            </div>

            <div class={style::events_calendar}>
                <div class={style::calendar_header}>
                    <h2>"Upcoming Events"</h2>
                    <div class={style::event_filters}>
                        <button class={format!("{} {}", style::filter_btn, "active")}>"All Events"</button>
                        <button class={style::filter_btn}>"Study Groups"</button>
                        <button class={style::filter_btn}>"Seminars"</button>
                        <button class={style::filter_btn}>"Workshops"</button>
                    </div>
                </div>

                <div class={style::events_list}>
                    {events.into_iter().map(|event| {
                        let event_type_str = event.event_type.to_string();
                        let date = event.date.clone();

                        view! {
                            <div class={style::event_card}>
                                <div class={style::event_content}>
                                    <div class={style::event_header}>
                                        <div class={style::title_group}>
                                            <span class={format!("{} type-{}", style::event_type_badge, event_type_str.to_lowercase().replace(" ", "-"))}>
                                                {event_type_str.clone()}
                                            </span>
                                            <h3 class={style::event_title}>{event.title}</h3>
                                        </div>
                                        <div class={style::badge_group}>
                                            <span class={style::event_date_badge}>{date.clone()}</span>
                                            {if event.recurring {
                                                Some(view! {
                                                    <span class={style::recurring_badge}>
                                                        <Repeat size=12 />
                                                        "Recurring"
                                                    </span>
                                                })
                                            } else {
                                                None
                                            }}
                                        </div>
                                    </div>
                                    <p class={style::event_description}>{event.description}</p>
                                    <div class={style::event_meta}>
                                        <div class={style::meta_item}>
                                            <Clock size=16 />
                                            <span>{event.time}</span>
                                        </div>
                                        <div class={style::meta_item}>
                                            <MapPin size=16 />
                                            <span>{event.location}</span>
                                        </div>
                                    </div>
                                    <button class={format!("{} {}", style::btn, style::btn_secondary)}>"Add to Calendar"</button>
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>

            <div class={style::join_discord}>
                <h2>"Stay Connected"</h2>
                <p>"Join our Discord server to stay updated on all events and connect with other members."</p>
                <a href="https://discord.chico.rs" target="_blank" class={format!("{} {} {}", style::btn, style::btn_primary, style::btn_large)}>
                    "Join Our Discord"
                </a>
            </div>
        </div>
    }
}