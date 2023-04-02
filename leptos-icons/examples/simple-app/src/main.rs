use leptos::*;
use leptos_icons::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();
    mount_to_body(|cx| {
        view! { cx,
            <div style="font-size: 8em; color: #8f39d3;">
                <LeptosAiIcon icon=AiIcon::AiPushpinTwotone style=Some(String::from("color: red"))/>
                <LeptosIcon icon=Icon::Bi(BiIcon::BiGraphql) width="2em" height="2em" style="color: green"/>
                <LeptosIcon icon=Icon::Bi(BiIcon::BiGraphql) style="color: orange" title="Custom title"/>
                <LeptosIcon icon=Icon::Bs(BsIcon::Bs1Circle) style="color: green"/>
                <LeptosIcon icon=Icon::Fa(FaIcon::FaBarsSolid) />
                <LeptosIcon icon=Icon::Im(ImIcon::ImPagebreak) />
                <LeptosIcon icon=Icon::Im(ImIcon::ImPageBreak) />
                <LeptosIcon icon=Icon::Io(IoIcon::IoColorWand) />
            </div>
        }
    })
}
