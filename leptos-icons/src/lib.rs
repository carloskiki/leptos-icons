
pub use leptos_icons_ai::*;
pub use leptos_icons_fa::*;
pub use leptos_icons_wi::*;
pub use leptos_icons_fi::*;
pub use leptos_icons_vs::*;
pub use leptos_icons_bs::*;
pub use leptos_icons_bi::*;
pub use leptos_icons_im::*;
pub use leptos_icons_io::*;
pub use leptos_icons_ri::*;
pub use leptos_icons_si::*;
pub use leptos_icons_ti::*;
pub use leptos_icons_hi::*;
pub use leptos_icons_cg::*;
pub use leptos_icons_tb::*;
pub use leptos_icons_oc::*;

#[cfg_attr(
    feature = "serde",
    derive(
        Debug,
        PartialEq,
        Eq,
        PartialOrd,
        Ord,
        Clone,
        Copy,
        serde::Serialize,
        serde::Deserialize
    )
)]
#[cfg_attr(
    not(feature = "serde"),
    derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)
)]
pub enum Icon {
    Ai(leptos_icons_ai::AiIcon),
    Fa(leptos_icons_fa::FaIcon),
    Wi(leptos_icons_wi::WiIcon),
    Fi(leptos_icons_fi::FiIcon),
    Vs(leptos_icons_vs::VsIcon),
    Bs(leptos_icons_bs::BsIcon),
    Bi(leptos_icons_bi::BiIcon),
    Im(leptos_icons_im::ImIcon),
    Io(leptos_icons_io::IoIcon),
    Ri(leptos_icons_ri::RiIcon),
    Si(leptos_icons_si::SiIcon),
    Ti(leptos_icons_ti::TiIcon),
    Hi(leptos_icons_hi::HiIcon),
    Cg(leptos_icons_cg::CgIcon),
    Tb(leptos_icons_tb::TbIcon),
    Oc(leptos_icons_oc::OcIcon),
}

use leptos::*;
#[component]
pub fn LeptosIcon(
    cx: Scope,
    icon: Icon,
    /// The width of the icon (horizontal side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    #[allow(unused)]
    width: Option<String>,
    /// The height of the icon (vertical side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    #[allow(unused)]
    height: Option<String>,
    /// HTML class attribute.
    #[prop(into, optional)]
    #[allow(unused)]
    class: Option<String>,
    /// HTML style attribute.
    #[prop(into, optional)]
    #[allow(unused)]
    style: Option<String>,
    /// ARIA accessibility title.
    #[prop(into, optional)]
    #[allow(unused)]
    title: Option<String>,
) -> impl IntoView {
    match icon {
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Ai(icon) => {
            LeptosAiIcon(cx, icon, width, height, class, style, title).into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Fa(icon) => {
            LeptosFaIcon(cx, icon, width, height, class, style, title).into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Wi(icon) => {
            LeptosWiIcon(cx, icon, width, height, class, style, title).into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Fi(icon) => {
            LeptosFiIcon(cx, icon, width, height, class, style, title).into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Vs(icon) => {
            LeptosVsIcon(cx, icon, width, height, class, style, title).into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Bs(icon) => {
            LeptosBsIcon(cx, icon, width, height, class, style, title).into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Bi(icon) => {
            LeptosBiIcon(cx, icon, width, height, class, style, title).into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Im(icon) => {
            LeptosImIcon(cx, icon, width, height, class, style, title).into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Io(icon) => {
            LeptosIoIcon(cx, icon, width, height, class, style, title).into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Ri(icon) => {
            LeptosRiIcon(cx, icon, width, height, class, style, title).into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Si(icon) => {
            LeptosSiIcon(cx, icon, width, height, class, style, title).into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Ti(icon) => {
            LeptosTiIcon(cx, icon, width, height, class, style, title).into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Hi(icon) => {
            LeptosHiIcon(cx, icon, width, height, class, style, title).into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Cg(icon) => {
            LeptosCgIcon(cx, icon, width, height, class, style, title).into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Tb(icon) => {
            LeptosTbIcon(cx, icon, width, height, class, style, title).into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Oc(icon) => {
            LeptosOcIcon(cx, icon, width, height, class, style, title).into_view(cx)
        }
    }
}
