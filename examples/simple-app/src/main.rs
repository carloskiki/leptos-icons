use leptos::*;
use leptos_icons::oc::lg::CheckCircleFill::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| view! { cx,
        <div>
            <CheckCircleFill class="test" size="600" title="a11y title" color="red" />
        </div>
    })
}
