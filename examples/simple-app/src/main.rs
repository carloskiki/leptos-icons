use leptos::*;
use leptos_icons::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|| {
        view! {
            <div style="font-size: 8em; color: #8f39d3;">
                <Icon icon=icondata::AiCarryOutTwotone style="color: green" />
                <Icon icon=icondata::BiGraphql width="2em" height="2em" style="color: green"/>
                <Icon icon=icondata::BiGraphql style="color: orange"/>
                <Icon icon=icondata::Bs1Circle style="color: red"/>
                <Icon icon=icondata::FaBarsSolid />
                <Icon icon=icondata::ImPagebreak />
                <Icon icon=icondata::ImPageBreak />
                <Icon icon=icondata::OcAlertSm />
                <Icon icon=icondata::OcAlertLg width="1em" height="2em" />
            </div>
        }
    })
}
