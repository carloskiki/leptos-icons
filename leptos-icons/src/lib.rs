
#[cfg(feature = "Ai")]
pub use leptos_icons_ai::*;
#[cfg(feature = "Fa")]
pub use leptos_icons_fa::*;
#[cfg(feature = "Wi")]
pub use leptos_icons_wi::*;
#[cfg(feature = "Fi")]
pub use leptos_icons_fi::*;
#[cfg(feature = "Vs")]
pub use leptos_icons_vs::*;
#[cfg(feature = "Bs")]
pub use leptos_icons_bs::*;
#[cfg(feature = "Bi")]
pub use leptos_icons_bi::*;
#[cfg(feature = "Im")]
pub use leptos_icons_im::*;
#[cfg(feature = "Io")]
pub use leptos_icons_io::*;
#[cfg(feature = "Ri")]
pub use leptos_icons_ri::*;
#[cfg(feature = "Si")]
pub use leptos_icons_si::*;
#[cfg(feature = "Ti")]
pub use leptos_icons_ti::*;
#[cfg(feature = "Hi")]
pub use leptos_icons_hi::*;
#[cfg(feature = "Cg")]
pub use leptos_icons_cg::*;
#[cfg(feature = "Tb")]
pub use leptos_icons_tb::*;
#[cfg(feature = "Oc")]
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
    #[cfg(feature = "Ai")]
    Ai(leptos_icons_ai::AiIcon),
    #[cfg(feature = "Fa")]
    Fa(leptos_icons_fa::FaIcon),
    #[cfg(feature = "Wi")]
    Wi(leptos_icons_wi::WiIcon),
    #[cfg(feature = "Fi")]
    Fi(leptos_icons_fi::FiIcon),
    #[cfg(feature = "Vs")]
    Vs(leptos_icons_vs::VsIcon),
    #[cfg(feature = "Bs")]
    Bs(leptos_icons_bs::BsIcon),
    #[cfg(feature = "Bi")]
    Bi(leptos_icons_bi::BiIcon),
    #[cfg(feature = "Im")]
    Im(leptos_icons_im::ImIcon),
    #[cfg(feature = "Io")]
    Io(leptos_icons_io::IoIcon),
    #[cfg(feature = "Ri")]
    Ri(leptos_icons_ri::RiIcon),
    #[cfg(feature = "Si")]
    Si(leptos_icons_si::SiIcon),
    #[cfg(feature = "Ti")]
    Ti(leptos_icons_ti::TiIcon),
    #[cfg(feature = "Hi")]
    Hi(leptos_icons_hi::HiIcon),
    #[cfg(feature = "Cg")]
    Cg(leptos_icons_cg::CgIcon),
    #[cfg(feature = "Tb")]
    Tb(leptos_icons_tb::TbIcon),
    #[cfg(feature = "Oc")]
    Oc(leptos_icons_oc::OcIcon),
}
#[cfg(feature = "Ai")]
impl From<AiIcon> for Icon {
    fn from(value: AiIcon) -> Self {
        Icon::Ai(value)
    }
}
#[cfg(feature = "Fa")]
impl From<FaIcon> for Icon {
    fn from(value: FaIcon) -> Self {
        Icon::Fa(value)
    }
}
#[cfg(feature = "Wi")]
impl From<WiIcon> for Icon {
    fn from(value: WiIcon) -> Self {
        Icon::Wi(value)
    }
}
#[cfg(feature = "Fi")]
impl From<FiIcon> for Icon {
    fn from(value: FiIcon) -> Self {
        Icon::Fi(value)
    }
}
#[cfg(feature = "Vs")]
impl From<VsIcon> for Icon {
    fn from(value: VsIcon) -> Self {
        Icon::Vs(value)
    }
}
#[cfg(feature = "Bs")]
impl From<BsIcon> for Icon {
    fn from(value: BsIcon) -> Self {
        Icon::Bs(value)
    }
}
#[cfg(feature = "Bi")]
impl From<BiIcon> for Icon {
    fn from(value: BiIcon) -> Self {
        Icon::Bi(value)
    }
}
#[cfg(feature = "Im")]
impl From<ImIcon> for Icon {
    fn from(value: ImIcon) -> Self {
        Icon::Im(value)
    }
}
#[cfg(feature = "Io")]
impl From<IoIcon> for Icon {
    fn from(value: IoIcon) -> Self {
        Icon::Io(value)
    }
}
#[cfg(feature = "Ri")]
impl From<RiIcon> for Icon {
    fn from(value: RiIcon) -> Self {
        Icon::Ri(value)
    }
}
#[cfg(feature = "Si")]
impl From<SiIcon> for Icon {
    fn from(value: SiIcon) -> Self {
        Icon::Si(value)
    }
}
#[cfg(feature = "Ti")]
impl From<TiIcon> for Icon {
    fn from(value: TiIcon) -> Self {
        Icon::Ti(value)
    }
}
#[cfg(feature = "Hi")]
impl From<HiIcon> for Icon {
    fn from(value: HiIcon) -> Self {
        Icon::Hi(value)
    }
}
#[cfg(feature = "Cg")]
impl From<CgIcon> for Icon {
    fn from(value: CgIcon) -> Self {
        Icon::Cg(value)
    }
}
#[cfg(feature = "Tb")]
impl From<TbIcon> for Icon {
    fn from(value: TbIcon) -> Self {
        Icon::Tb(value)
    }
}
#[cfg(feature = "Oc")]
impl From<OcIcon> for Icon {
    fn from(value: OcIcon) -> Self {
        Icon::Oc(value)
    }
}

#[leptos::component]
pub fn LeptosIcon(
    cx: leptos::Scope,
    /// The icon to show.
    #[prop(into)]
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
) -> impl leptos::IntoView {
    leptos::IntoView::into_view(
        match icon {
            #[cfg(feature = "Ai")]
            #[allow(unreachable_code, unreachable_patterns)]
            Icon::Ai(icon) => LeptosAiIcon(cx, icon, width, height, class, style),
            #[cfg(feature = "Fa")]
            #[allow(unreachable_code, unreachable_patterns)]
            Icon::Fa(icon) => LeptosFaIcon(cx, icon, width, height, class, style),
            #[cfg(feature = "Wi")]
            #[allow(unreachable_code, unreachable_patterns)]
            Icon::Wi(icon) => LeptosWiIcon(cx, icon, width, height, class, style),
            #[cfg(feature = "Fi")]
            #[allow(unreachable_code, unreachable_patterns)]
            Icon::Fi(icon) => LeptosFiIcon(cx, icon, width, height, class, style),
            #[cfg(feature = "Vs")]
            #[allow(unreachable_code, unreachable_patterns)]
            Icon::Vs(icon) => LeptosVsIcon(cx, icon, width, height, class, style),
            #[cfg(feature = "Bs")]
            #[allow(unreachable_code, unreachable_patterns)]
            Icon::Bs(icon) => LeptosBsIcon(cx, icon, width, height, class, style),
            #[cfg(feature = "Bi")]
            #[allow(unreachable_code, unreachable_patterns)]
            Icon::Bi(icon) => LeptosBiIcon(cx, icon, width, height, class, style),
            #[cfg(feature = "Im")]
            #[allow(unreachable_code, unreachable_patterns)]
            Icon::Im(icon) => LeptosImIcon(cx, icon, width, height, class, style),
            #[cfg(feature = "Io")]
            #[allow(unreachable_code, unreachable_patterns)]
            Icon::Io(icon) => LeptosIoIcon(cx, icon, width, height, class, style),
            #[cfg(feature = "Ri")]
            #[allow(unreachable_code, unreachable_patterns)]
            Icon::Ri(icon) => LeptosRiIcon(cx, icon, width, height, class, style),
            #[cfg(feature = "Si")]
            #[allow(unreachable_code, unreachable_patterns)]
            Icon::Si(icon) => LeptosSiIcon(cx, icon, width, height, class, style),
            #[cfg(feature = "Ti")]
            #[allow(unreachable_code, unreachable_patterns)]
            Icon::Ti(icon) => LeptosTiIcon(cx, icon, width, height, class, style),
            #[cfg(feature = "Hi")]
            #[allow(unreachable_code, unreachable_patterns)]
            Icon::Hi(icon) => LeptosHiIcon(cx, icon, width, height, class, style),
            #[cfg(feature = "Cg")]
            #[allow(unreachable_code, unreachable_patterns)]
            Icon::Cg(icon) => LeptosCgIcon(cx, icon, width, height, class, style),
            #[cfg(feature = "Tb")]
            #[allow(unreachable_code, unreachable_patterns)]
            Icon::Tb(icon) => LeptosTbIcon(cx, icon, width, height, class, style),
            #[cfg(feature = "Oc")]
            #[allow(unreachable_code, unreachable_patterns)]
            Icon::Oc(icon) => LeptosOcIcon(cx, icon, width, height, class, style),
        },
        cx,
    )
}
