use leptos::*;
use leptos_icons_ai::*;
use leptos_icons_bi::*;
use leptos_icons_bs::*;
use leptos_icons_fa::*;
use leptos_icons_im::*;
use leptos_icons_io::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        view! { cx,
            <div style="font-size: 8em; color: #8f39d3;">
                <LeptosAiIcon icon=AiIcon::AiPushpinTwotone />
                <LeptosBiIcon icon=BiIcon::BiGraphql width="2em" height="2em" />
                <LeptosBiIcon icon=BiIcon::BiGraphql />
                <LeptosBsIcon icon=BsIcon::Bs1Circle />
                <LeptosFaIcon icon=FaIcon::FaBarsSolid />
                <LeptosImIcon icon=ImIcon::ImPagebreak />
                <LeptosImIcon icon=ImIcon::ImPageBreak />
                <MyIcon icon=IoIcon::IoColorWand />
            </div>
        }
    })
}

#[component]
fn MyIcon(cx: Scope, icon: IoIcon) -> impl IntoView {
    view! { cx,
        <LeptosIoIcon icon=icon style="color: red"/>
    }
}
