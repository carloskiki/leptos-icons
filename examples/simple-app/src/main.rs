use leptos::*;
use leptos_icons::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        view! { cx,
            <div style="font-size: 8em; color: #8f39d3;">
                <Icon icon=Icon::from(AiIcon::AiCarryOutTwotone) style="color: green" />
                <Icon icon=Icon::from(BiIcon::BiGraphql) width="2em" height="2em" style="color: green"/>
                <Icon icon=Icon::from(BiIcon::BiGraphql) style="color: orange"/>
                <Icon icon=Icon::from(BsIcon::Bs1Circle) style="color: red"/>
                <Icon icon=Icon::from(FaIcon::FaBarsSolid) />
                <Icon icon=Icon::from(ImIcon::ImPagebreak) />
                <Icon icon=Icon::from(ImIcon::ImPageBreak) />
                <Icon icon=Icon::from(OcIcon::OcAlertSm) />
                <Icon icon=Icon::from(OcIcon::OcAlertLg) width="1em" height="2em" />
            </div>
        }
    })
}
