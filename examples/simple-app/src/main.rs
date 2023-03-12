use leptos::*;
use leptos_icons::fa::solid::Bars::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx,
        <div>
            <Bars />
        </div>
    })
}
