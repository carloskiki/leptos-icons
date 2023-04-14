
#[cfg(feature = "Ai")]
pub use icondata_ai::*;
#[cfg(feature = "Fa")]
pub use icondata_fa::*;
#[cfg(feature = "Wi")]
pub use icondata_wi::*;
#[cfg(feature = "Fi")]
pub use icondata_fi::*;
#[cfg(feature = "Vs")]
pub use icondata_vs::*;
#[cfg(feature = "Bs")]
pub use icondata_bs::*;
#[cfg(feature = "Bi")]
pub use icondata_bi::*;
#[cfg(feature = "Im")]
pub use icondata_im::*;
#[cfg(feature = "Io")]
pub use icondata_io::*;
#[cfg(feature = "Ri")]
pub use icondata_ri::*;
#[cfg(feature = "Si")]
pub use icondata_si::*;
#[cfg(feature = "Ti")]
pub use icondata_ti::*;
#[cfg(feature = "Hi")]
pub use icondata_hi::*;
#[cfg(feature = "Cg")]
pub use icondata_cg::*;
#[cfg(feature = "Tb")]
pub use icondata_tb::*;
#[cfg(feature = "Oc")]
pub use icondata_oc::*;

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
    Ai(icondata_ai::AiIcon),
    #[cfg(feature = "Fa")]
    Fa(icondata_fa::FaIcon),
    #[cfg(feature = "Wi")]
    Wi(icondata_wi::WiIcon),
    #[cfg(feature = "Fi")]
    Fi(icondata_fi::FiIcon),
    #[cfg(feature = "Vs")]
    Vs(icondata_vs::VsIcon),
    #[cfg(feature = "Bs")]
    Bs(icondata_bs::BsIcon),
    #[cfg(feature = "Bi")]
    Bi(icondata_bi::BiIcon),
    #[cfg(feature = "Im")]
    Im(icondata_im::ImIcon),
    #[cfg(feature = "Io")]
    Io(icondata_io::IoIcon),
    #[cfg(feature = "Ri")]
    Ri(icondata_ri::RiIcon),
    #[cfg(feature = "Si")]
    Si(icondata_si::SiIcon),
    #[cfg(feature = "Ti")]
    Ti(icondata_ti::TiIcon),
    #[cfg(feature = "Hi")]
    Hi(icondata_hi::HiIcon),
    #[cfg(feature = "Cg")]
    Cg(icondata_cg::CgIcon),
    #[cfg(feature = "Tb")]
    Tb(icondata_tb::TbIcon),
    #[cfg(feature = "Oc")]
    Oc(icondata_oc::OcIcon),
}
impl<'a> icondata_core::IconData<'a> for crate::Icon {
    fn data(self) -> &'a icondata_core::Data {
        match self {
            #[cfg(feature = "Ai")]
            Self::Ai(icon) => icondata_core::IconData::data(icon),
            #[cfg(feature = "Fa")]
            Self::Fa(icon) => icondata_core::IconData::data(icon),
            #[cfg(feature = "Wi")]
            Self::Wi(icon) => icondata_core::IconData::data(icon),
            #[cfg(feature = "Fi")]
            Self::Fi(icon) => icondata_core::IconData::data(icon),
            #[cfg(feature = "Vs")]
            Self::Vs(icon) => icondata_core::IconData::data(icon),
            #[cfg(feature = "Bs")]
            Self::Bs(icon) => icondata_core::IconData::data(icon),
            #[cfg(feature = "Bi")]
            Self::Bi(icon) => icondata_core::IconData::data(icon),
            #[cfg(feature = "Im")]
            Self::Im(icon) => icondata_core::IconData::data(icon),
            #[cfg(feature = "Io")]
            Self::Io(icon) => icondata_core::IconData::data(icon),
            #[cfg(feature = "Ri")]
            Self::Ri(icon) => icondata_core::IconData::data(icon),
            #[cfg(feature = "Si")]
            Self::Si(icon) => icondata_core::IconData::data(icon),
            #[cfg(feature = "Ti")]
            Self::Ti(icon) => icondata_core::IconData::data(icon),
            #[cfg(feature = "Hi")]
            Self::Hi(icon) => icondata_core::IconData::data(icon),
            #[cfg(feature = "Cg")]
            Self::Cg(icon) => icondata_core::IconData::data(icon),
            #[cfg(feature = "Tb")]
            Self::Tb(icon) => icondata_core::IconData::data(icon),
            #[cfg(feature = "Oc")]
            Self::Oc(icon) => icondata_core::IconData::data(icon),
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
