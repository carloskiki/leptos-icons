
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
