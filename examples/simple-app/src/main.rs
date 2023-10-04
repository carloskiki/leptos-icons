use leptos::*;
use leptos_icons::AiIcon::*;
use leptos_icons::BiIcon::*;
use leptos_icons::BsIcon::*;
use leptos_icons::FaIcon::*;
use leptos_icons::ImIcon::*;
use leptos_icons::OcIcon::*;
use leptos_icons::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <div style="font-size: 8em; color: #8f39d3;">
                <Icon icon=Icon::from(AiCarryOutTwotone) style="color: green" />
                <Icon icon=Icon::from(BiGraphql) width="2em" height="2em" style="color: green"/>
                <Icon icon=Icon::from(BiGraphql) style="color: orange"/>
                <Icon icon=Icon::from(Bs1Circle) style="color: red"/>
                <Icon icon=Icon::from(FaBarsSolid) />
                <Icon icon=Icon::from(ImPagebreak) />
                <Icon icon=Icon::from(ImPageBreak) />
                <Icon icon=Icon::from(OcAlertSm) />
                <Icon icon=Icon::from(OcAlertLg) width="1em" height="2em" />
            </div>
        }
    })
}
