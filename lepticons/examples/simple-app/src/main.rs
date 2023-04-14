use leptos::*;
use leptos_meta::*;
use lepticons::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        provide_meta_context(cx);
        view! { cx,
            <div style="font-size: 8em; color: #8f39d3;">
                <LeptosIcon icon=BiIcon::BiGraphql width="2em" height="2em" style="color: green"/>
                <LeptosIcon icon=BiIcon::BiGraphql style="color: orange"/>
                <LeptosIcon icon=BsIcon::Bs1Circle style="color: red"/>
                <LeptosIcon icon=FaIcon::FaBarsSolid />
                <LeptosIcon icon=ImIcon::ImPagebreak />
                <LeptosIcon icon=ImIcon::ImPageBreak />
                <LeptosIcon icon=IoIcon::IoColorWand />
            </div>
        }
    })
}
