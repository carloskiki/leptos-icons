use leptos::*;
use leptos_icons::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        view! { cx,
            <div style="font-size: 8em; color: #8f39d3;">
                <LeptosIcon icon=Icon::BiGraphql width="2em" height="2em" />
                <LeptosIcon icon=Icon::AiPushpinTwotone />
                <LeptosIcon icon=Icon::BiGraphql />
                <LeptosIcon icon=Icon::Bs1Circle />
                <LeptosIcon icon=Icon::FaBarsSolid />
                <LeptosIcon icon=Icon::ImPagebreak />
                <LeptosIcon icon=Icon::ImPageBreak />
                <MyIcon icon=Icon::IoColorWand />
            </div>
        }
    })
}

#[component]
fn MyIcon(cx: Scope, icon: Icon) -> impl IntoView {
    view! { cx,
        <LeptosIcon icon=icon style="color: red"/>
    }
}
