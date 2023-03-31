use leptos::*;
use leptos_icons::ai::*;
use leptos_icons::bi::*;
use leptos_icons::bs::*;
use leptos_icons::fa::*;
use leptos_icons::im::*;
use leptos_icons::io::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        view! { cx,
            <div style="font-size: 8em; color: #8f39d3;">
                <AiPushpinTwotone />
                <BiGraphql />
                <Bs1Circle />
                <FaBarsSolid />
                <ImPagebreak />
                <ImPageBreak />
                <IoColorWand />
            </div>
        }
    })
}
