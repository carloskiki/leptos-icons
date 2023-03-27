use leptos::*;
use leptos_icons::bs::*;
use leptos_icons::fa::*;
use leptos_icons::im::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        view! { cx,
            <div>
                <Bs1Circle />
                <FaBarsSolid />
                <ImPagebreak />
                <ImPageBreak />
            </div>
        }
    })
}
