
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
    use leptos_icons_core::IconData;
    leptos::IntoView::into_view(
        match icon {
            #[cfg(feature = "Ai")]
            Icon::Ai(icon) => {
                leptos_icons_core::LeptosIconCore(
                    cx,
                    icon.data(),
                    width,
                    height,
                    class,
                    style,
                )
            }
            #[cfg(feature = "Fa")]
            Icon::Fa(icon) => {
                leptos_icons_core::LeptosIconCore(
                    cx,
                    icon.data(),
                    width,
                    height,
                    class,
                    style,
                )
            }
            #[cfg(feature = "Wi")]
            Icon::Wi(icon) => {
                leptos_icons_core::LeptosIconCore(
                    cx,
                    icon.data(),
                    width,
                    height,
                    class,
                    style,
                )
            }
            #[cfg(feature = "Fi")]
            Icon::Fi(icon) => {
                leptos_icons_core::LeptosIconCore(
                    cx,
                    icon.data(),
                    width,
                    height,
                    class,
                    style,
                )
            }
            #[cfg(feature = "Vs")]
            Icon::Vs(icon) => {
                leptos_icons_core::LeptosIconCore(
                    cx,
                    icon.data(),
                    width,
                    height,
                    class,
                    style,
                )
            }
            #[cfg(feature = "Bs")]
            Icon::Bs(icon) => {
                leptos_icons_core::LeptosIconCore(
                    cx,
                    icon.data(),
                    width,
                    height,
                    class,
                    style,
                )
            }
            #[cfg(feature = "Bi")]
            Icon::Bi(icon) => {
                leptos_icons_core::LeptosIconCore(
                    cx,
                    icon.data(),
                    width,
                    height,
                    class,
                    style,
                )
            }
            #[cfg(feature = "Im")]
            Icon::Im(icon) => {
                leptos_icons_core::LeptosIconCore(
                    cx,
                    icon.data(),
                    width,
                    height,
                    class,
                    style,
                )
            }
            #[cfg(feature = "Io")]
            Icon::Io(icon) => {
                leptos_icons_core::LeptosIconCore(
                    cx,
                    icon.data(),
                    width,
                    height,
                    class,
                    style,
                )
            }
            #[cfg(feature = "Ri")]
            Icon::Ri(icon) => {
                leptos_icons_core::LeptosIconCore(
                    cx,
                    icon.data(),
                    width,
                    height,
                    class,
                    style,
                )
            }
            #[cfg(feature = "Si")]
            Icon::Si(icon) => {
                leptos_icons_core::LeptosIconCore(
                    cx,
                    icon.data(),
                    width,
                    height,
                    class,
                    style,
                )
            }
            #[cfg(feature = "Ti")]
            Icon::Ti(icon) => {
                leptos_icons_core::LeptosIconCore(
                    cx,
                    icon.data(),
                    width,
                    height,
                    class,
                    style,
                )
            }
            #[cfg(feature = "Hi")]
            Icon::Hi(icon) => {
                leptos_icons_core::LeptosIconCore(
                    cx,
                    icon.data(),
                    width,
                    height,
                    class,
                    style,
                )
            }
            #[cfg(feature = "Cg")]
            Icon::Cg(icon) => {
                leptos_icons_core::LeptosIconCore(
                    cx,
                    icon.data(),
                    width,
                    height,
                    class,
                    style,
                )
            }
            #[cfg(feature = "Tb")]
            Icon::Tb(icon) => {
                leptos_icons_core::LeptosIconCore(
                    cx,
                    icon.data(),
                    width,
                    height,
                    class,
                    style,
                )
            }
            #[cfg(feature = "Oc")]
            Icon::Oc(icon) => {
                leptos_icons_core::LeptosIconCore(
                    cx,
                    icon.data(),
                    width,
                    height,
                    class,
                    style,
                )
            }
        },
        cx,
    )
}
