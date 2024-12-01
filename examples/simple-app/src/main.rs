use icondata as i;
use leptos::prelude::*;
use leptos_icons::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        view! {
            <div style="font-size: 8em; color: #8f39d3;">
                <Icon icon=i::AiCarryOutTwotone style="color: green" />
                <Icon icon=i::BiGraphql width="2em" height="2em" style="color: green"/>
                <Icon icon=i::BiGraphql style="color: orange"/>
                <Icon icon=i::Bs1Circle style="color: red"/>
                <Icon icon=i::FaBarsSolid />
                <Icon icon=i::ImPagebreak />
                <Icon icon=i::ImPageBreak />
                <Icon icon=i::OcAlertSm />
                <Icon icon=i::OcAlertLg width="1em" height="2em" />
            </div>
        }
    })
}
