use leptos::prelude::*;
use leptos_icons::Icon;
use leptos_meta::{provide_meta_context, Meta};

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    provide_meta_context();

    let (dark, set_dark) = signal(false);
    let icon = Signal::derive(move || {
        if dark.get() {
            icondata::BsMoonStars
        } else {
            icondata::BsSun
        }
    });
    let toggle_theme = move |_| set_dark.update(|dark| *dark = !*dark);

    let div_style = view! {
        <{..} style="display: flex; align-items: center; justify-content: center; width: 100vw; height: 100vh; margin: 0; cursor: pointer;" />
    };

    let icon_style = view! {
        <{..} style="padding: 0.5rem; border: 4px solid; border-radius: 1rem;" />
    };

    mount_to_body(move || {
        view! {
            <Meta name="color-scheme" content=move || if dark.get() { "light".to_string() } else { "dark".to_string() } />
            <div {..div_style} on:click=toggle_theme>
                <Icon icon width="20rem" height="20rem" {..icon_style} />
            </div>
        }
    });
}
