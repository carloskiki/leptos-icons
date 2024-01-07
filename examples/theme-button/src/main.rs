use leptos::*;
use leptos_icons::*;
use leptos_meta::*;
use icondata::BsIcon;

const DIV_STYLE: &str = r#"
    display: flex;
    align-items: center;
    justify-content: center;
    width: 100vw;
    height: 100vh;
    margin: 0;
    cursor: pointer;
"#;

const ICON_STYLE: &str = r#"
    width: 20rem;
    height: 20rem;
    padding: 0.5rem;
    border: 4px solid;
    border-radius: 1rem;
"#;

pub fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    provide_meta_context();

    let (dark, set_dark) = create_signal(false);
    let icon: MaybeSignal<icondata::BsIcon> = MaybeSignal::from(Signal::derive(move || {
        if dark.get() {
            icondata::BsMoonStars
        } else {
            icondata::BsSun
        }
    }));
    let toggle_theme = move |_| set_dark.update(|dark| *dark = !*dark);

    mount_to_body(move || {
        view! {
            <Meta name="color-scheme" content=move || if dark.get() { "light".to_string() } else { "dark".to_string() } />
            <div on:click=toggle_theme style=DIV_STYLE>
                <Icon<BsIcon> icon=icon style=ICON_STYLE />
            </div>
        }
    })
}
