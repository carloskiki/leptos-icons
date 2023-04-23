use leptos::*;
use leptos_meta::*;
use leptos_icons::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        provide_meta_context(cx);
        view! { cx,
            <div style="font-size: 8em; color: #8f39d3;">
                 <Icon icon=BiIcon::BiGraphql width="2em" height="2em" style="color: green"/>
                 <Icon icon=BiIcon::BiGraphql style="color: orange"/>
                 <Icon icon=BsIcon::Bs1Circle style="color: red"/>
                 <Icon icon=FaIcon::FaBarsSolid />
                 <Icon icon=ImIcon::ImPagebreak />
                 <Icon icon=ImIcon::ImPageBreak />
                 <Icon icon=OcIcon::OcAlertSm />
                 <Icon icon=OcIcon::OcAlertLg width="1em" height="2em" />
            </div>
        }
    })
}
