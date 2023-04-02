
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
    width: String,
    /// The height of the icon (vertical side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    #[allow(unused)]
    height: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    #[allow(unused)]
    class: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    #[allow(unused)]
    style: String,
    /// ARIA accessibility title.
    #[prop(into, optional_no_strip)]
    #[allow(unused)]
    title: Option<String>,
) -> impl IntoView {
    match icon {
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Ai(icon) => {
            view! {
                cx, < LeptosAiIcon icon width height class style title / >
            }
                .into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Fa(icon) => {
            view! {
                cx, < LeptosFaIcon icon width height class style title / >
            }
                .into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Wi(icon) => {
            view! {
                cx, < LeptosWiIcon icon width height class style title / >
            }
                .into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Fi(icon) => {
            view! {
                cx, < LeptosFiIcon icon width height class style title / >
            }
                .into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Vs(icon) => {
            view! {
                cx, < LeptosVsIcon icon width height class style title / >
            }
                .into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Bs(icon) => {
            view! {
                cx, < LeptosBsIcon icon width height class style title / >
            }
                .into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Bi(icon) => {
            view! {
                cx, < LeptosBiIcon icon width height class style title / >
            }
                .into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Im(icon) => {
            view! {
                cx, < LeptosImIcon icon width height class style title / >
            }
                .into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Io(icon) => {
            view! {
                cx, < LeptosIoIcon icon width height class style title / >
            }
                .into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Ri(icon) => {
            view! {
                cx, < LeptosRiIcon icon width height class style title / >
            }
                .into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Si(icon) => {
            view! {
                cx, < LeptosSiIcon icon width height class style title / >
            }
                .into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Ti(icon) => {
            view! {
                cx, < LeptosTiIcon icon width height class style title / >
            }
                .into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Hi(icon) => {
            view! {
                cx, < LeptosHiIcon icon width height class style title / >
            }
                .into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Cg(icon) => {
            view! {
                cx, < LeptosCgIcon icon width height class style title / >
            }
                .into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Tb(icon) => {
            view! {
                cx, < LeptosTbIcon icon width height class style title / >
            }
                .into_view(cx)
        }
        #[allow(unreachable_code, unreachable_patterns)]
        Icon::Oc(icon) => {
            view! {
                cx, < LeptosOcIcon icon width height class style title / >
            }
                .into_view(cx)
        }
    }
}
