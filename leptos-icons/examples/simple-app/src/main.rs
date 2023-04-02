use leptos::*;
use leptos_icons::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        view! { cx,
            <div style="font-size: 8em; color: #8f39d3;">
                <LeptosAiIcon icon=AiIcon::AiPushpinTwotone style="color: red"/>
                //<LeptosBiIcon icon=BiIcon::BiGraphql width="2em" height="2em" />
                //<LeptosBiIcon icon=BiIcon::BiGraphql />
                //<LeptosBsIcon icon=BsIcon::Bs1Circle />
                //<LeptosFaIcon icon=FaIcon::FaBarsSolid />
                //<LeptosImIcon icon=ImIcon::ImPagebreak />
                //<LeptosImIcon icon=ImIcon::ImPageBreak />
                <LeptosIcon icon=Icon::Io(IoIcon::IoColorWand) style="color: green"/>
            </div>
        }
    })
}
