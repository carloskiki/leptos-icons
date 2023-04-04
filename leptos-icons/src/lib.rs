
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
impl<'a> leptos_icons_core::IconData<'a> for crate::Icon {
    fn data(self) -> &'a leptos_icons_core::Data {
        match self {
            #[cfg(feature = "Ai")]
            Self::Ai(icon) => leptos_icons_core::IconData::data(icon),
            #[cfg(feature = "Fa")]
            Self::Fa(icon) => leptos_icons_core::IconData::data(icon),
            #[cfg(feature = "Wi")]
            Self::Wi(icon) => leptos_icons_core::IconData::data(icon),
            #[cfg(feature = "Fi")]
            Self::Fi(icon) => leptos_icons_core::IconData::data(icon),
            #[cfg(feature = "Vs")]
            Self::Vs(icon) => leptos_icons_core::IconData::data(icon),
            #[cfg(feature = "Bs")]
            Self::Bs(icon) => leptos_icons_core::IconData::data(icon),
            #[cfg(feature = "Bi")]
            Self::Bi(icon) => leptos_icons_core::IconData::data(icon),
            #[cfg(feature = "Im")]
            Self::Im(icon) => leptos_icons_core::IconData::data(icon),
            #[cfg(feature = "Io")]
            Self::Io(icon) => leptos_icons_core::IconData::data(icon),
            #[cfg(feature = "Ri")]
            Self::Ri(icon) => leptos_icons_core::IconData::data(icon),
            #[cfg(feature = "Si")]
            Self::Si(icon) => leptos_icons_core::IconData::data(icon),
            #[cfg(feature = "Ti")]
            Self::Ti(icon) => leptos_icons_core::IconData::data(icon),
            #[cfg(feature = "Hi")]
            Self::Hi(icon) => leptos_icons_core::IconData::data(icon),
            #[cfg(feature = "Cg")]
            Self::Cg(icon) => leptos_icons_core::IconData::data(icon),
            #[cfg(feature = "Tb")]
            Self::Tb(icon) => leptos_icons_core::IconData::data(icon),
            #[cfg(feature = "Oc")]
            Self::Oc(icon) => leptos_icons_core::IconData::data(icon),
        }
    }
}
#[cfg(feature = "Ai")]
impl From<AiIcon> for Icon {
    fn from(value: AiIcon) -> Self {
        Self::Ai(value)
    }
}
#[cfg(feature = "Fa")]
impl From<FaIcon> for Icon {
    fn from(value: FaIcon) -> Self {
        Self::Fa(value)
    }
}
#[cfg(feature = "Wi")]
impl From<WiIcon> for Icon {
    fn from(value: WiIcon) -> Self {
        Self::Wi(value)
    }
}
#[cfg(feature = "Fi")]
impl From<FiIcon> for Icon {
    fn from(value: FiIcon) -> Self {
        Self::Fi(value)
    }
}
#[cfg(feature = "Vs")]
impl From<VsIcon> for Icon {
    fn from(value: VsIcon) -> Self {
        Self::Vs(value)
    }
}
#[cfg(feature = "Bs")]
impl From<BsIcon> for Icon {
    fn from(value: BsIcon) -> Self {
        Self::Bs(value)
    }
}
#[cfg(feature = "Bi")]
impl From<BiIcon> for Icon {
    fn from(value: BiIcon) -> Self {
        Self::Bi(value)
    }
}
#[cfg(feature = "Im")]
impl From<ImIcon> for Icon {
    fn from(value: ImIcon) -> Self {
        Self::Im(value)
    }
}
#[cfg(feature = "Io")]
impl From<IoIcon> for Icon {
    fn from(value: IoIcon) -> Self {
        Self::Io(value)
    }
}
#[cfg(feature = "Ri")]
impl From<RiIcon> for Icon {
    fn from(value: RiIcon) -> Self {
        Self::Ri(value)
    }
}
#[cfg(feature = "Si")]
impl From<SiIcon> for Icon {
    fn from(value: SiIcon) -> Self {
        Self::Si(value)
    }
}
#[cfg(feature = "Ti")]
impl From<TiIcon> for Icon {
    fn from(value: TiIcon) -> Self {
        Self::Ti(value)
    }
}
#[cfg(feature = "Hi")]
impl From<HiIcon> for Icon {
    fn from(value: HiIcon) -> Self {
        Self::Hi(value)
    }
}
#[cfg(feature = "Cg")]
impl From<CgIcon> for Icon {
    fn from(value: CgIcon) -> Self {
        Self::Cg(value)
    }
}
#[cfg(feature = "Tb")]
impl From<TbIcon> for Icon {
    fn from(value: TbIcon) -> Self {
        Self::Tb(value)
    }
}
#[cfg(feature = "Oc")]
impl From<OcIcon> for Icon {
    fn from(value: OcIcon) -> Self {
        Self::Oc(value)
    }
}

#[cfg(
    any(
        feature = "Ai",
        feature = "Fa",
        feature = "Wi",
        feature = "Fi",
        feature = "Vs",
        feature = "Bs",
        feature = "Bi",
        feature = "Im",
        feature = "Io",
        feature = "Ri",
        feature = "Si",
        feature = "Ti",
        feature = "Hi",
        feature = "Cg",
        feature = "Tb",
        feature = "Oc",
    )
)]
#[leptos::component]
pub fn LeptosIcon(
    cx: leptos::Scope,
    /// The icon to show.
    #[prop(into)]
    icon: crate::Icon,
    /// The width of the icon (horizontal side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    width: Option<String>,
    /// The height of the icon (vertical side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional)]
    height: Option<String>,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: Option<String>,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: Option<String>,
) -> impl leptos::IntoView {
    leptos::IntoView::into_view(
        leptos_icons_core::LeptosIconCore(
            cx,
            leptos_icons_core::IconData::data(icon),
            width,
            height,
            class,
            style,
        ),
        cx,
    )
}
