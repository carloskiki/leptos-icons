
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
pub enum FiIcon {
    #[cfg(feature = "FiActivity")]
    FiActivity,
    #[cfg(feature = "FiAirplay")]
    FiAirplay,
    #[cfg(feature = "FiAlertCircle")]
    FiAlertCircle,
    #[cfg(feature = "FiAlertOctagon")]
    FiAlertOctagon,
    #[cfg(feature = "FiAlertTriangle")]
    FiAlertTriangle,
    #[cfg(feature = "FiAlignCenter")]
    FiAlignCenter,
    #[cfg(feature = "FiAlignJustify")]
    FiAlignJustify,
    #[cfg(feature = "FiAlignLeft")]
    FiAlignLeft,
    #[cfg(feature = "FiAlignRight")]
    FiAlignRight,
    #[cfg(feature = "FiAnchor")]
    FiAnchor,
    #[cfg(feature = "FiAperture")]
    FiAperture,
    #[cfg(feature = "FiArchive")]
    FiArchive,
    #[cfg(feature = "FiArrowDown")]
    FiArrowDown,
    #[cfg(feature = "FiArrowDownCircle")]
    FiArrowDownCircle,
    #[cfg(feature = "FiArrowDownLeft")]
    FiArrowDownLeft,
    #[cfg(feature = "FiArrowDownRight")]
    FiArrowDownRight,
    #[cfg(feature = "FiArrowLeft")]
    FiArrowLeft,
    #[cfg(feature = "FiArrowLeftCircle")]
    FiArrowLeftCircle,
    #[cfg(feature = "FiArrowRight")]
    FiArrowRight,
    #[cfg(feature = "FiArrowRightCircle")]
    FiArrowRightCircle,
    #[cfg(feature = "FiArrowUp")]
    FiArrowUp,
    #[cfg(feature = "FiArrowUpCircle")]
    FiArrowUpCircle,
    #[cfg(feature = "FiArrowUpLeft")]
    FiArrowUpLeft,
    #[cfg(feature = "FiArrowUpRight")]
    FiArrowUpRight,
    #[cfg(feature = "FiAtSign")]
    FiAtSign,
    #[cfg(feature = "FiAward")]
    FiAward,
    #[cfg(feature = "FiBarChart")]
    FiBarChart,
    #[cfg(feature = "FiBarChart2")]
    FiBarChart2,
    #[cfg(feature = "FiBattery")]
    FiBattery,
    #[cfg(feature = "FiBatteryCharging")]
    FiBatteryCharging,
    #[cfg(feature = "FiBell")]
    FiBell,
    #[cfg(feature = "FiBellOff")]
    FiBellOff,
    #[cfg(feature = "FiBluetooth")]
    FiBluetooth,
    #[cfg(feature = "FiBold")]
    FiBold,
    #[cfg(feature = "FiBook")]
    FiBook,
    #[cfg(feature = "FiBookOpen")]
    FiBookOpen,
    #[cfg(feature = "FiBookmark")]
    FiBookmark,
    #[cfg(feature = "FiBox")]
    FiBox,
    #[cfg(feature = "FiBriefcase")]
    FiBriefcase,
    #[cfg(feature = "FiCalendar")]
    FiCalendar,
    #[cfg(feature = "FiCamera")]
    FiCamera,
    #[cfg(feature = "FiCameraOff")]
    FiCameraOff,
    #[cfg(feature = "FiCast")]
    FiCast,
    #[cfg(feature = "FiCheck")]
    FiCheck,
    #[cfg(feature = "FiCheckCircle")]
    FiCheckCircle,
    #[cfg(feature = "FiCheckSquare")]
    FiCheckSquare,
    #[cfg(feature = "FiChevronDown")]
    FiChevronDown,
    #[cfg(feature = "FiChevronLeft")]
    FiChevronLeft,
    #[cfg(feature = "FiChevronRight")]
    FiChevronRight,
    #[cfg(feature = "FiChevronUp")]
    FiChevronUp,
    #[cfg(feature = "FiChevronsDown")]
    FiChevronsDown,
    #[cfg(feature = "FiChevronsLeft")]
    FiChevronsLeft,
    #[cfg(feature = "FiChevronsRight")]
    FiChevronsRight,
    #[cfg(feature = "FiChevronsUp")]
    FiChevronsUp,
    #[cfg(feature = "FiChrome")]
    FiChrome,
    #[cfg(feature = "FiCircle")]
    FiCircle,
    #[cfg(feature = "FiClipboard")]
    FiClipboard,
    #[cfg(feature = "FiClock")]
    FiClock,
    #[cfg(feature = "FiCloud")]
    FiCloud,
    #[cfg(feature = "FiCloudDrizzle")]
    FiCloudDrizzle,
    #[cfg(feature = "FiCloudLightning")]
    FiCloudLightning,
    #[cfg(feature = "FiCloudOff")]
    FiCloudOff,
    #[cfg(feature = "FiCloudRain")]
    FiCloudRain,
    #[cfg(feature = "FiCloudSnow")]
    FiCloudSnow,
    #[cfg(feature = "FiCode")]
    FiCode,
    #[cfg(feature = "FiCodepen")]
    FiCodepen,
    #[cfg(feature = "FiCodesandbox")]
    FiCodesandbox,
    #[cfg(feature = "FiCoffee")]
    FiCoffee,
    #[cfg(feature = "FiColumns")]
    FiColumns,
    #[cfg(feature = "FiCommand")]
    FiCommand,
    #[cfg(feature = "FiCompass")]
    FiCompass,
    #[cfg(feature = "FiCopy")]
    FiCopy,
    #[cfg(feature = "FiCornerDownLeft")]
    FiCornerDownLeft,
    #[cfg(feature = "FiCornerDownRight")]
    FiCornerDownRight,
    #[cfg(feature = "FiCornerLeftDown")]
    FiCornerLeftDown,
    #[cfg(feature = "FiCornerLeftUp")]
    FiCornerLeftUp,
    #[cfg(feature = "FiCornerRightDown")]
    FiCornerRightDown,
    #[cfg(feature = "FiCornerRightUp")]
    FiCornerRightUp,
    #[cfg(feature = "FiCornerUpLeft")]
    FiCornerUpLeft,
    #[cfg(feature = "FiCornerUpRight")]
    FiCornerUpRight,
    #[cfg(feature = "FiCpu")]
    FiCpu,
    #[cfg(feature = "FiCreditCard")]
    FiCreditCard,
    #[cfg(feature = "FiCrop")]
    FiCrop,
    #[cfg(feature = "FiCrosshair")]
    FiCrosshair,
    #[cfg(feature = "FiDatabase")]
    FiDatabase,
    #[cfg(feature = "FiDelete")]
    FiDelete,
    #[cfg(feature = "FiDisc")]
    FiDisc,
    #[cfg(feature = "FiDivide")]
    FiDivide,
    #[cfg(feature = "FiDivideCircle")]
    FiDivideCircle,
    #[cfg(feature = "FiDivideSquare")]
    FiDivideSquare,
    #[cfg(feature = "FiDollarSign")]
    FiDollarSign,
    #[cfg(feature = "FiDownload")]
    FiDownload,
    #[cfg(feature = "FiDownloadCloud")]
    FiDownloadCloud,
    #[cfg(feature = "FiDribbble")]
    FiDribbble,
    #[cfg(feature = "FiDroplet")]
    FiDroplet,
    #[cfg(feature = "FiEdit")]
    FiEdit,
    #[cfg(feature = "FiEdit2")]
    FiEdit2,
    #[cfg(feature = "FiEdit3")]
    FiEdit3,
    #[cfg(feature = "FiExternalLink")]
    FiExternalLink,
    #[cfg(feature = "FiEye")]
    FiEye,
    #[cfg(feature = "FiEyeOff")]
    FiEyeOff,
    #[cfg(feature = "FiFacebook")]
    FiFacebook,
    #[cfg(feature = "FiFastForward")]
    FiFastForward,
    #[cfg(feature = "FiFeather")]
    FiFeather,
    #[cfg(feature = "FiFigma")]
    FiFigma,
    #[cfg(feature = "FiFile")]
    FiFile,
    #[cfg(feature = "FiFileMinus")]
    FiFileMinus,
    #[cfg(feature = "FiFilePlus")]
    FiFilePlus,
    #[cfg(feature = "FiFileText")]
    FiFileText,
    #[cfg(feature = "FiFilm")]
    FiFilm,
    #[cfg(feature = "FiFilter")]
    FiFilter,
    #[cfg(feature = "FiFlag")]
    FiFlag,
    #[cfg(feature = "FiFolder")]
    FiFolder,
    #[cfg(feature = "FiFolderMinus")]
    FiFolderMinus,
    #[cfg(feature = "FiFolderPlus")]
    FiFolderPlus,
    #[cfg(feature = "FiFramer")]
    FiFramer,
    #[cfg(feature = "FiFrown")]
    FiFrown,
    #[cfg(feature = "FiGift")]
    FiGift,
    #[cfg(feature = "FiGitBranch")]
    FiGitBranch,
    #[cfg(feature = "FiGitCommit")]
    FiGitCommit,
    #[cfg(feature = "FiGitMerge")]
    FiGitMerge,
    #[cfg(feature = "FiGitPullRequest")]
    FiGitPullRequest,
    #[cfg(feature = "FiGithub")]
    FiGithub,
    #[cfg(feature = "FiGitlab")]
    FiGitlab,
    #[cfg(feature = "FiGlobe")]
    FiGlobe,
    #[cfg(feature = "FiGrid")]
    FiGrid,
    #[cfg(feature = "FiHardDrive")]
    FiHardDrive,
    #[cfg(feature = "FiHash")]
    FiHash,
    #[cfg(feature = "FiHeadphones")]
    FiHeadphones,
    #[cfg(feature = "FiHeart")]
    FiHeart,
    #[cfg(feature = "FiHelpCircle")]
    FiHelpCircle,
    #[cfg(feature = "FiHexagon")]
    FiHexagon,
    #[cfg(feature = "FiHome")]
    FiHome,
    #[cfg(feature = "FiImage")]
    FiImage,
    #[cfg(feature = "FiInbox")]
    FiInbox,
    #[cfg(feature = "FiInfo")]
    FiInfo,
    #[cfg(feature = "FiInstagram")]
    FiInstagram,
    #[cfg(feature = "FiItalic")]
    FiItalic,
    #[cfg(feature = "FiKey")]
    FiKey,
    #[cfg(feature = "FiLayers")]
    FiLayers,
    #[cfg(feature = "FiLayout")]
    FiLayout,
    #[cfg(feature = "FiLifeBuoy")]
    FiLifeBuoy,
    #[cfg(feature = "FiLink")]
    FiLink,
    #[cfg(feature = "FiLink2")]
    FiLink2,
    #[cfg(feature = "FiLinkedin")]
    FiLinkedin,
    #[cfg(feature = "FiList")]
    FiList,
    #[cfg(feature = "FiLoader")]
    FiLoader,
    #[cfg(feature = "FiLock")]
    FiLock,
    #[cfg(feature = "FiLogIn")]
    FiLogIn,
    #[cfg(feature = "FiLogOut")]
    FiLogOut,
    #[cfg(feature = "FiMail")]
    FiMail,
    #[cfg(feature = "FiMap")]
    FiMap,
    #[cfg(feature = "FiMapPin")]
    FiMapPin,
    #[cfg(feature = "FiMaximize")]
    FiMaximize,
    #[cfg(feature = "FiMaximize2")]
    FiMaximize2,
    #[cfg(feature = "FiMeh")]
    FiMeh,
    #[cfg(feature = "FiMenu")]
    FiMenu,
    #[cfg(feature = "FiMessageCircle")]
    FiMessageCircle,
    #[cfg(feature = "FiMessageSquare")]
    FiMessageSquare,
    #[cfg(feature = "FiMic")]
    FiMic,
    #[cfg(feature = "FiMicOff")]
    FiMicOff,
    #[cfg(feature = "FiMinimize")]
    FiMinimize,
    #[cfg(feature = "FiMinimize2")]
    FiMinimize2,
    #[cfg(feature = "FiMinus")]
    FiMinus,
    #[cfg(feature = "FiMinusCircle")]
    FiMinusCircle,
    #[cfg(feature = "FiMinusSquare")]
    FiMinusSquare,
    #[cfg(feature = "FiMonitor")]
    FiMonitor,
    #[cfg(feature = "FiMoon")]
    FiMoon,
    #[cfg(feature = "FiMoreHorizontal")]
    FiMoreHorizontal,
    #[cfg(feature = "FiMoreVertical")]
    FiMoreVertical,
    #[cfg(feature = "FiMousePointer")]
    FiMousePointer,
    #[cfg(feature = "FiMove")]
    FiMove,
    #[cfg(feature = "FiMusic")]
    FiMusic,
    #[cfg(feature = "FiNavigation")]
    FiNavigation,
    #[cfg(feature = "FiNavigation2")]
    FiNavigation2,
    #[cfg(feature = "FiOctagon")]
    FiOctagon,
    #[cfg(feature = "FiPackage")]
    FiPackage,
    #[cfg(feature = "FiPaperclip")]
    FiPaperclip,
    #[cfg(feature = "FiPause")]
    FiPause,
    #[cfg(feature = "FiPauseCircle")]
    FiPauseCircle,
    #[cfg(feature = "FiPenTool")]
    FiPenTool,
    #[cfg(feature = "FiPercent")]
    FiPercent,
    #[cfg(feature = "FiPhone")]
    FiPhone,
    #[cfg(feature = "FiPhoneCall")]
    FiPhoneCall,
    #[cfg(feature = "FiPhoneForwarded")]
    FiPhoneForwarded,
    #[cfg(feature = "FiPhoneIncoming")]
    FiPhoneIncoming,
    #[cfg(feature = "FiPhoneMissed")]
    FiPhoneMissed,
    #[cfg(feature = "FiPhoneOff")]
    FiPhoneOff,
    #[cfg(feature = "FiPhoneOutgoing")]
    FiPhoneOutgoing,
    #[cfg(feature = "FiPieChart")]
    FiPieChart,
    #[cfg(feature = "FiPlay")]
    FiPlay,
    #[cfg(feature = "FiPlayCircle")]
    FiPlayCircle,
    #[cfg(feature = "FiPlus")]
    FiPlus,
    #[cfg(feature = "FiPlusCircle")]
    FiPlusCircle,
    #[cfg(feature = "FiPlusSquare")]
    FiPlusSquare,
    #[cfg(feature = "FiPocket")]
    FiPocket,
    #[cfg(feature = "FiPower")]
    FiPower,
    #[cfg(feature = "FiPrinter")]
    FiPrinter,
    #[cfg(feature = "FiRadio")]
    FiRadio,
    #[cfg(feature = "FiRefreshCcw")]
    FiRefreshCcw,
    #[cfg(feature = "FiRefreshCw")]
    FiRefreshCw,
    #[cfg(feature = "FiRepeat")]
    FiRepeat,
    #[cfg(feature = "FiRewind")]
    FiRewind,
    #[cfg(feature = "FiRotateCcw")]
    FiRotateCcw,
    #[cfg(feature = "FiRotateCw")]
    FiRotateCw,
    #[cfg(feature = "FiRss")]
    FiRss,
    #[cfg(feature = "FiSave")]
    FiSave,
    #[cfg(feature = "FiScissors")]
    FiScissors,
    #[cfg(feature = "FiSearch")]
    FiSearch,
    #[cfg(feature = "FiSend")]
    FiSend,
    #[cfg(feature = "FiServer")]
    FiServer,
    #[cfg(feature = "FiSettings")]
    FiSettings,
    #[cfg(feature = "FiShare")]
    FiShare,
    #[cfg(feature = "FiShare2")]
    FiShare2,
    #[cfg(feature = "FiShield")]
    FiShield,
    #[cfg(feature = "FiShieldOff")]
    FiShieldOff,
    #[cfg(feature = "FiShoppingBag")]
    FiShoppingBag,
    #[cfg(feature = "FiShoppingCart")]
    FiShoppingCart,
    #[cfg(feature = "FiShuffle")]
    FiShuffle,
    #[cfg(feature = "FiSidebar")]
    FiSidebar,
    #[cfg(feature = "FiSkipBack")]
    FiSkipBack,
    #[cfg(feature = "FiSkipForward")]
    FiSkipForward,
    #[cfg(feature = "FiSlack")]
    FiSlack,
    #[cfg(feature = "FiSlash")]
    FiSlash,
    #[cfg(feature = "FiSliders")]
    FiSliders,
    #[cfg(feature = "FiSmartphone")]
    FiSmartphone,
    #[cfg(feature = "FiSmile")]
    FiSmile,
    #[cfg(feature = "FiSpeaker")]
    FiSpeaker,
    #[cfg(feature = "FiSquare")]
    FiSquare,
    #[cfg(feature = "FiStar")]
    FiStar,
    #[cfg(feature = "FiStopCircle")]
    FiStopCircle,
    #[cfg(feature = "FiSun")]
    FiSun,
    #[cfg(feature = "FiSunrise")]
    FiSunrise,
    #[cfg(feature = "FiSunset")]
    FiSunset,
    #[cfg(feature = "FiTable")]
    FiTable,
    #[cfg(feature = "FiTablet")]
    FiTablet,
    #[cfg(feature = "FiTag")]
    FiTag,
    #[cfg(feature = "FiTarget")]
    FiTarget,
    #[cfg(feature = "FiTerminal")]
    FiTerminal,
    #[cfg(feature = "FiThermometer")]
    FiThermometer,
    #[cfg(feature = "FiThumbsDown")]
    FiThumbsDown,
    #[cfg(feature = "FiThumbsUp")]
    FiThumbsUp,
    #[cfg(feature = "FiToggleLeft")]
    FiToggleLeft,
    #[cfg(feature = "FiToggleRight")]
    FiToggleRight,
    #[cfg(feature = "FiTool")]
    FiTool,
    #[cfg(feature = "FiTrash")]
    FiTrash,
    #[cfg(feature = "FiTrash2")]
    FiTrash2,
    #[cfg(feature = "FiTrello")]
    FiTrello,
    #[cfg(feature = "FiTrendingDown")]
    FiTrendingDown,
    #[cfg(feature = "FiTrendingUp")]
    FiTrendingUp,
    #[cfg(feature = "FiTriangle")]
    FiTriangle,
    #[cfg(feature = "FiTruck")]
    FiTruck,
    #[cfg(feature = "FiTv")]
    FiTv,
    #[cfg(feature = "FiTwitch")]
    FiTwitch,
    #[cfg(feature = "FiTwitter")]
    FiTwitter,
    #[cfg(feature = "FiType")]
    FiType,
    #[cfg(feature = "FiUmbrella")]
    FiUmbrella,
    #[cfg(feature = "FiUnderline")]
    FiUnderline,
    #[cfg(feature = "FiUnlock")]
    FiUnlock,
    #[cfg(feature = "FiUpload")]
    FiUpload,
    #[cfg(feature = "FiUploadCloud")]
    FiUploadCloud,
    #[cfg(feature = "FiUser")]
    FiUser,
    #[cfg(feature = "FiUserCheck")]
    FiUserCheck,
    #[cfg(feature = "FiUserMinus")]
    FiUserMinus,
    #[cfg(feature = "FiUserPlus")]
    FiUserPlus,
    #[cfg(feature = "FiUserX")]
    FiUserX,
    #[cfg(feature = "FiUsers")]
    FiUsers,
    #[cfg(feature = "FiVideo")]
    FiVideo,
    #[cfg(feature = "FiVideoOff")]
    FiVideoOff,
    #[cfg(feature = "FiVoicemail")]
    FiVoicemail,
    #[cfg(feature = "FiVolume")]
    FiVolume,
    #[cfg(feature = "FiVolume1")]
    FiVolume1,
    #[cfg(feature = "FiVolume2")]
    FiVolume2,
    #[cfg(feature = "FiVolumeX")]
    FiVolumeX,
    #[cfg(feature = "FiWatch")]
    FiWatch,
    #[cfg(feature = "FiWifi")]
    FiWifi,
    #[cfg(feature = "FiWifiOff")]
    FiWifiOff,
    #[cfg(feature = "FiWind")]
    FiWind,
    #[cfg(feature = "FiX")]
    FiX,
    #[cfg(feature = "FiXCircle")]
    FiXCircle,
    #[cfg(feature = "FiXOctagon")]
    FiXOctagon,
    #[cfg(feature = "FiXSquare")]
    FiXSquare,
    #[cfg(feature = "FiYoutube")]
    FiYoutube,
    #[cfg(feature = "FiZap")]
    FiZap,
    #[cfg(feature = "FiZapOff")]
    FiZapOff,
    #[cfg(feature = "FiZoomIn")]
    FiZoomIn,
    #[cfg(feature = "FiZoomOut")]
    FiZoomOut,
}

use leptos::*;

#[component]
pub fn LeptosFiIcon(
    #[allow(unused)]
    cx: Scope,
    /// Variant of the icon to display.
    #[allow(unused)]
    icon: FiIcon,
    /// The width of the icon (horizontal side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional_no_strip)]
    #[allow(unused)]
    width: Option<String>,
    /// The height of the icon (vertical side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional_no_strip)]
    #[allow(unused)]
    height: Option<String>,
    /// HTML class attribute.
    #[prop(into, optional_no_strip)]
    #[allow(unused)]
    class: Option<String>,
    /// HTML style attribute.
    #[prop(into, optional_no_strip)]
    #[allow(unused)]
    style: Option<String>,
    /// ARIA accessibility title.
    #[prop(into, optional_no_strip)]
    #[allow(unused)]
    title: Option<String>,
) -> impl IntoView {
    #[allow(unused)]
    let width = width.unwrap_or_else(|| String::from("1em"));
    #[allow(unused)]
    let height = height.unwrap_or_else(|| String::from("1em"));
    #[allow(unused)]
    let class = class.unwrap_or_else(|| String::from(""));
    #[allow(unused)]
    let style = style.unwrap_or_else(|| String::from(""));
    match icon {
        #[cfg(feature = "FiActivity")]
        FiIcon::FiActivity => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiActivity".to_owned()) } < / title > < polyline points =
                "22 12 18 12 15 21 9 3 6 12 2 12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiAirplay")]
        FiIcon::FiAirplay => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiAirplay".to_owned()) } < / title > < path d =
                "M5 17H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-1"
                /> < polygon points = "12 15 17 21 7 21 12 15" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiAlertCircle")]
        FiIcon::FiAlertCircle => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiAlertCircle".to_owned()) } < / title > < circle cx = "12" cy = "12" r
                = "10" /> < line x1 = "12" y1 = "8" x2 = "12" y2 = "12" /> < line x1 =
                "12" y1 = "16" x2 = "12.01" y2 = "16" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiAlertOctagon")]
        FiIcon::FiAlertOctagon => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiAlertOctagon".to_owned()) } < / title > < polygon points =
                "7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2"
                /> < line x1 = "12" y1 = "8" x2 = "12" y2 = "12" /> < line x1 = "12" y1 =
                "16" x2 = "12.01" y2 = "16" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiAlertTriangle")]
        FiIcon::FiAlertTriangle => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiAlertTriangle".to_owned()) } < / title > < path d =
                "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"
                /> < line x1 = "12" y1 = "9" x2 = "12" y2 = "13" /> < line x1 = "12" y1 =
                "17" x2 = "12.01" y2 = "17" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiAlignCenter")]
        FiIcon::FiAlignCenter => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiAlignCenter".to_owned()) } < / title > < line x1 = "18" y1 = "10" x2 =
                "6" y2 = "10" /> < line x1 = "21" y1 = "6" x2 = "3" y2 = "6" /> < line x1
                = "21" y1 = "14" x2 = "3" y2 = "14" /> < line x1 = "18" y1 = "18" x2 =
                "6" y2 = "18" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiAlignJustify")]
        FiIcon::FiAlignJustify => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiAlignJustify".to_owned()) } < / title > < line x1 = "21" y1 = "10" x2
                = "3" y2 = "10" /> < line x1 = "21" y1 = "6" x2 = "3" y2 = "6" /> < line
                x1 = "21" y1 = "14" x2 = "3" y2 = "14" /> < line x1 = "21" y1 = "18" x2 =
                "3" y2 = "18" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiAlignLeft")]
        FiIcon::FiAlignLeft => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiAlignLeft".to_owned()) } < / title > < line x1 = "17" y1 = "10" x2 =
                "3" y2 = "10" /> < line x1 = "21" y1 = "6" x2 = "3" y2 = "6" /> < line x1
                = "21" y1 = "14" x2 = "3" y2 = "14" /> < line x1 = "17" y1 = "18" x2 =
                "3" y2 = "18" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiAlignRight")]
        FiIcon::FiAlignRight => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiAlignRight".to_owned()) } < / title > < line x1 = "21" y1 = "10" x2 =
                "7" y2 = "10" /> < line x1 = "21" y1 = "6" x2 = "3" y2 = "6" /> < line x1
                = "21" y1 = "14" x2 = "3" y2 = "14" /> < line x1 = "21" y1 = "18" x2 =
                "7" y2 = "18" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiAnchor")]
        FiIcon::FiAnchor => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiAnchor".to_owned()) } < / title > < circle cx = "12" cy = "5" r = "3"
                /> < line x1 = "12" y1 = "22" x2 = "12" y2 = "8" /> < path d =
                "M5 12H2a10 10 0 0 0 20 0h-3" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiAperture")]
        FiIcon::FiAperture => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiAperture".to_owned()) } < / title > < circle cx = "12" cy = "12" r =
                "10" /> < line x1 = "14.31" y1 = "8" x2 = "20.05" y2 = "17.94" /> < line
                x1 = "9.69" y1 = "8" x2 = "21.17" y2 = "8" /> < line x1 = "7.38" y1 =
                "12" x2 = "13.12" y2 = "2.06" /> < line x1 = "9.69" y1 = "16" x2 = "3.95"
                y2 = "6.06" /> < line x1 = "14.31" y1 = "16" x2 = "2.83" y2 = "16" /> <
                line x1 = "16.62" y1 = "12" x2 = "10.88" y2 = "21.94" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiArchive")]
        FiIcon::FiArchive => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiArchive".to_owned()) } < / title > < polyline points =
                "21 8 21 21 3 21 3 8" /> < rect x = "1" y = "3" width = "22" height = "5"
                /> < line x1 = "10" y1 = "12" x2 = "14" y2 = "12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiArrowDown")]
        FiIcon::FiArrowDown => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiArrowDown".to_owned()) } < / title > < line x1 = "12" y1 = "5" x2 =
                "12" y2 = "19" /> < polyline points = "19 12 12 19 5 12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiArrowDownCircle")]
        FiIcon::FiArrowDownCircle => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiArrowDownCircle".to_owned()) } < / title > < circle cx = "12" cy =
                "12" r = "10" /> < polyline points = "8 12 12 16 16 12" /> < line x1 =
                "12" y1 = "8" x2 = "12" y2 = "16" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiArrowDownLeft")]
        FiIcon::FiArrowDownLeft => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiArrowDownLeft".to_owned()) } < / title > < line x1 = "17" y1 = "7" x2
                = "7" y2 = "17" /> < polyline points = "17 17 7 17 7 7" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiArrowDownRight")]
        FiIcon::FiArrowDownRight => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiArrowDownRight".to_owned()) } < / title > < line x1 = "7" y1 = "7" x2
                = "17" y2 = "17" /> < polyline points = "17 7 17 17 7 17" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiArrowLeft")]
        FiIcon::FiArrowLeft => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiArrowLeft".to_owned()) } < / title > < line x1 = "19" y1 = "12" x2 =
                "5" y2 = "12" /> < polyline points = "12 19 5 12 12 5" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiArrowLeftCircle")]
        FiIcon::FiArrowLeftCircle => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiArrowLeftCircle".to_owned()) } < / title > < circle cx = "12" cy =
                "12" r = "10" /> < polyline points = "12 8 8 12 12 16" /> < line x1 =
                "16" y1 = "12" x2 = "8" y2 = "12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiArrowRight")]
        FiIcon::FiArrowRight => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiArrowRight".to_owned()) } < / title > < line x1 = "5" y1 = "12" x2 =
                "19" y2 = "12" /> < polyline points = "12 5 19 12 12 19" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiArrowRightCircle")]
        FiIcon::FiArrowRightCircle => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiArrowRightCircle".to_owned()) } < / title > < circle cx = "12" cy =
                "12" r = "10" /> < polyline points = "12 16 16 12 12 8" /> < line x1 =
                "8" y1 = "12" x2 = "16" y2 = "12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiArrowUp")]
        FiIcon::FiArrowUp => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiArrowUp".to_owned()) } < / title > < line x1 = "12" y1 = "19" x2 =
                "12" y2 = "5" /> < polyline points = "5 12 12 5 19 12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiArrowUpCircle")]
        FiIcon::FiArrowUpCircle => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiArrowUpCircle".to_owned()) } < / title > < circle cx = "12" cy = "12"
                r = "10" /> < polyline points = "16 12 12 8 8 12" /> < line x1 = "12" y1
                = "16" x2 = "12" y2 = "8" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiArrowUpLeft")]
        FiIcon::FiArrowUpLeft => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiArrowUpLeft".to_owned()) } < / title > < line x1 = "17" y1 = "17" x2 =
                "7" y2 = "7" /> < polyline points = "7 17 7 7 17 7" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiArrowUpRight")]
        FiIcon::FiArrowUpRight => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiArrowUpRight".to_owned()) } < / title > < line x1 = "7" y1 = "17" x2 =
                "17" y2 = "7" /> < polyline points = "7 7 17 7 17 17" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiAtSign")]
        FiIcon::FiAtSign => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiAtSign".to_owned()) } < / title > < circle cx = "12" cy = "12" r = "4"
                /> < path d = "M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-3.92 7.94" /> < / svg
                >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiAward")]
        FiIcon::FiAward => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiAward".to_owned()) } < / title > < circle cx = "12" cy = "8" r = "7"
                /> < polyline points = "8.21 13.89 7 23 12 20 17 23 15.79 13.88" /> < /
                svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiBarChart")]
        FiIcon::FiBarChart => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiBarChart".to_owned()) } < / title > < line x1 = "12" y1 = "20" x2 =
                "12" y2 = "10" /> < line x1 = "18" y1 = "20" x2 = "18" y2 = "4" /> < line
                x1 = "6" y1 = "20" x2 = "6" y2 = "16" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiBarChart2")]
        FiIcon::FiBarChart2 => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiBarChart2".to_owned()) } < / title > < line x1 = "18" y1 = "20" x2 =
                "18" y2 = "10" /> < line x1 = "12" y1 = "20" x2 = "12" y2 = "4" /> < line
                x1 = "6" y1 = "20" x2 = "6" y2 = "14" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiBattery")]
        FiIcon::FiBattery => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiBattery".to_owned()) } < / title > < rect x = "1" y = "6" width = "18"
                height = "12" rx = "2" ry = "2" /> < line x1 = "23" y1 = "13" x2 = "23"
                y2 = "11" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiBatteryCharging")]
        FiIcon::FiBatteryCharging => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiBatteryCharging".to_owned()) } < / title > < path d =
                "M5 18H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3.19M15 6h2a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-3.19"
                /> < line x1 = "23" y1 = "13" x2 = "23" y2 = "11" /> < polyline points =
                "11 6 7 12 13 12 9 18" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiBell")]
        FiIcon::FiBell => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiBell".to_owned()) } < / title > < path d =
                "M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9" /> < path d =
                "M13.73 21a2 2 0 0 1-3.46 0" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiBellOff")]
        FiIcon::FiBellOff => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiBellOff".to_owned()) } < / title > < path d =
                "M13.73 21a2 2 0 0 1-3.46 0" /> < path d =
                "M18.63 13A17.89 17.89 0 0 1 18 8" /> < path d =
                "M6.26 6.26A5.86 5.86 0 0 0 6 8c0 7-3 9-3 9h14" /> < path d =
                "M18 8a6 6 0 0 0-9.33-5" /> < line x1 = "1" y1 = "1" x2 = "23" y2 = "23"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiBluetooth")]
        FiIcon::FiBluetooth => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiBluetooth".to_owned()) } < / title > < polyline points =
                "6.5 6.5 17.5 17.5 12 23 12 1 17.5 6.5 6.5 17.5" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiBold")]
        FiIcon::FiBold => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiBold".to_owned()) } < / title > < path d =
                "M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" /> < path d =
                "M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiBook")]
        FiIcon::FiBook => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiBook".to_owned()) } < / title > < path d =
                "M4 19.5A2.5 2.5 0 0 1 6.5 17H20" /> < path d =
                "M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z" /> < /
                svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiBookOpen")]
        FiIcon::FiBookOpen => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiBookOpen".to_owned()) } < / title > < path d =
                "M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z" /> < path d =
                "M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiBookmark")]
        FiIcon::FiBookmark => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiBookmark".to_owned()) } < / title > < path d =
                "M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiBox")]
        FiIcon::FiBox => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiBox".to_owned()) } < / title > < path d =
                "M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"
                /> < polyline points = "3.27 6.96 12 12.01 20.73 6.96" /> < line x1 =
                "12" y1 = "22.08" x2 = "12" y2 = "12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiBriefcase")]
        FiIcon::FiBriefcase => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiBriefcase".to_owned()) } < / title > < rect x = "2" y = "7" width =
                "20" height = "14" rx = "2" ry = "2" /> < path d =
                "M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCalendar")]
        FiIcon::FiCalendar => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCalendar".to_owned()) } < / title > < rect x = "3" y = "4" width =
                "18" height = "18" rx = "2" ry = "2" /> < line x1 = "16" y1 = "2" x2 =
                "16" y2 = "6" /> < line x1 = "8" y1 = "2" x2 = "8" y2 = "6" /> < line x1
                = "3" y1 = "10" x2 = "21" y2 = "10" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCamera")]
        FiIcon::FiCamera => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCamera".to_owned()) } < / title > < path d =
                "M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z"
                /> < circle cx = "12" cy = "13" r = "4" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCameraOff")]
        FiIcon::FiCameraOff => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCameraOff".to_owned()) } < / title > < line x1 = "1" y1 = "1" x2 =
                "23" y2 = "23" /> < path d =
                "M21 21H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3m3-3h6l2 3h4a2 2 0 0 1 2 2v9.34m-7.72-2.06a4 4 0 1 1-5.56-5.56"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCast")]
        FiIcon::FiCast => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCast".to_owned()) } < / title > < path d =
                "M2 16.1A5 5 0 0 1 5.9 20M2 12.05A9 9 0 0 1 9.95 20M2 8V6a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2h-6"
                /> < line x1 = "2" y1 = "20" x2 = "2.01" y2 = "20" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCheck")]
        FiIcon::FiCheck => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCheck".to_owned()) } < / title > < polyline points = "20 6 9 17 4 12"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCheckCircle")]
        FiIcon::FiCheckCircle => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCheckCircle".to_owned()) } < / title > < path d =
                "M22 11.08V12a10 10 0 1 1-5.93-9.14" /> < polyline points =
                "22 4 12 14.01 9 11.01" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCheckSquare")]
        FiIcon::FiCheckSquare => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCheckSquare".to_owned()) } < / title > < polyline points =
                "9 11 12 14 22 4" /> < path d =
                "M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiChevronDown")]
        FiIcon::FiChevronDown => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiChevronDown".to_owned()) } < / title > < polyline points =
                "6 9 12 15 18 9" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiChevronLeft")]
        FiIcon::FiChevronLeft => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiChevronLeft".to_owned()) } < / title > < polyline points =
                "15 18 9 12 15 6" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiChevronRight")]
        FiIcon::FiChevronRight => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiChevronRight".to_owned()) } < / title > < polyline points =
                "9 18 15 12 9 6" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiChevronUp")]
        FiIcon::FiChevronUp => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiChevronUp".to_owned()) } < / title > < polyline points =
                "18 15 12 9 6 15" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiChevronsDown")]
        FiIcon::FiChevronsDown => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiChevronsDown".to_owned()) } < / title > < polyline points =
                "7 13 12 18 17 13" /> < polyline points = "7 6 12 11 17 6" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiChevronsLeft")]
        FiIcon::FiChevronsLeft => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiChevronsLeft".to_owned()) } < / title > < polyline points =
                "11 17 6 12 11 7" /> < polyline points = "18 17 13 12 18 7" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiChevronsRight")]
        FiIcon::FiChevronsRight => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiChevronsRight".to_owned()) } < / title > < polyline points =
                "13 17 18 12 13 7" /> < polyline points = "6 17 11 12 6 7" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiChevronsUp")]
        FiIcon::FiChevronsUp => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiChevronsUp".to_owned()) } < / title > < polyline points =
                "17 11 12 6 7 11" /> < polyline points = "17 18 12 13 7 18" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiChrome")]
        FiIcon::FiChrome => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiChrome".to_owned()) } < / title > < circle cx = "12" cy = "12" r =
                "10" /> < circle cx = "12" cy = "12" r = "4" /> < line x1 = "21.17" y1 =
                "8" x2 = "12" y2 = "8" /> < line x1 = "3.95" y1 = "6.06" x2 = "8.54" y2 =
                "14" /> < line x1 = "10.88" y1 = "21.94" x2 = "15.46" y2 = "14" /> < /
                svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCircle")]
        FiIcon::FiCircle => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCircle".to_owned()) } < / title > < circle cx = "12" cy = "12" r =
                "10" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiClipboard")]
        FiIcon::FiClipboard => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiClipboard".to_owned()) } < / title > < path d =
                "M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2"
                /> < rect x = "8" y = "2" width = "8" height = "4" rx = "1" ry = "1" /> <
                / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiClock")]
        FiIcon::FiClock => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiClock".to_owned()) } < / title > < circle cx = "12" cy = "12" r = "10"
                /> < polyline points = "12 6 12 12 16 14" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCloud")]
        FiIcon::FiCloud => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCloud".to_owned()) } < / title > < path d =
                "M18 10h-1.26A8 8 0 1 0 9 20h9a5 5 0 0 0 0-10z" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCloudDrizzle")]
        FiIcon::FiCloudDrizzle => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCloudDrizzle".to_owned()) } < / title > < line x1 = "8" y1 = "19" x2 =
                "8" y2 = "21" /> < line x1 = "8" y1 = "13" x2 = "8" y2 = "15" /> < line
                x1 = "16" y1 = "19" x2 = "16" y2 = "21" /> < line x1 = "16" y1 = "13" x2
                = "16" y2 = "15" /> < line x1 = "12" y1 = "21" x2 = "12" y2 = "23" /> <
                line x1 = "12" y1 = "15" x2 = "12" y2 = "17" /> < path d =
                "M20 16.58A5 5 0 0 0 18 7h-1.26A8 8 0 1 0 4 15.25" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCloudLightning")]
        FiIcon::FiCloudLightning => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCloudLightning".to_owned()) } < / title > < path d =
                "M19 16.9A5 5 0 0 0 18 7h-1.26a8 8 0 1 0-11.62 9" /> < polyline points =
                "13 11 9 17 15 17 11 23" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCloudOff")]
        FiIcon::FiCloudOff => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCloudOff".to_owned()) } < / title > < path d =
                "M22.61 16.95A5 5 0 0 0 18 10h-1.26a8 8 0 0 0-7.05-6M5 5a8 8 0 0 0 4 15h9a5 5 0 0 0 1.7-.3"
                /> < line x1 = "1" y1 = "1" x2 = "23" y2 = "23" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCloudRain")]
        FiIcon::FiCloudRain => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCloudRain".to_owned()) } < / title > < line x1 = "16" y1 = "13" x2 =
                "16" y2 = "21" /> < line x1 = "8" y1 = "13" x2 = "8" y2 = "21" /> < line
                x1 = "12" y1 = "15" x2 = "12" y2 = "23" /> < path d =
                "M20 16.58A5 5 0 0 0 18 7h-1.26A8 8 0 1 0 4 15.25" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCloudSnow")]
        FiIcon::FiCloudSnow => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCloudSnow".to_owned()) } < / title > < path d =
                "M20 17.58A5 5 0 0 0 18 8h-1.26A8 8 0 1 0 4 16.25" /> < line x1 = "8" y1
                = "16" x2 = "8.01" y2 = "16" /> < line x1 = "8" y1 = "20" x2 = "8.01" y2
                = "20" /> < line x1 = "12" y1 = "18" x2 = "12.01" y2 = "18" /> < line x1
                = "12" y1 = "22" x2 = "12.01" y2 = "22" /> < line x1 = "16" y1 = "16" x2
                = "16.01" y2 = "16" /> < line x1 = "16" y1 = "20" x2 = "16.01" y2 = "20"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCode")]
        FiIcon::FiCode => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCode".to_owned()) } < / title > < polyline points = "16 18 22 12 16 6"
                /> < polyline points = "8 6 2 12 8 18" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCodepen")]
        FiIcon::FiCodepen => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCodepen".to_owned()) } < / title > < polygon points =
                "12 2 22 8.5 22 15.5 12 22 2 15.5 2 8.5 12 2" /> < line x1 = "12" y1 =
                "22" x2 = "12" y2 = "15.5" /> < polyline points = "22 8.5 12 15.5 2 8.5"
                /> < polyline points = "2 15.5 12 8.5 22 15.5" /> < line x1 = "12" y1 =
                "2" x2 = "12" y2 = "8.5" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCodesandbox")]
        FiIcon::FiCodesandbox => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCodesandbox".to_owned()) } < / title > < path d =
                "M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"
                /> < polyline points = "7.5 4.21 12 6.81 16.5 4.21" /> < polyline points
                = "7.5 19.79 7.5 14.6 3 12" /> < polyline points =
                "21 12 16.5 14.6 16.5 19.79" /> < polyline points =
                "3.27 6.96 12 12.01 20.73 6.96" /> < line x1 = "12" y1 = "22.08" x2 =
                "12" y2 = "12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCoffee")]
        FiIcon::FiCoffee => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCoffee".to_owned()) } < / title > < path d =
                "M18 8h1a4 4 0 0 1 0 8h-1" /> < path d =
                "M2 8h16v9a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V8z" /> < line x1 = "6" y1 = "1"
                x2 = "6" y2 = "4" /> < line x1 = "10" y1 = "1" x2 = "10" y2 = "4" /> <
                line x1 = "14" y1 = "1" x2 = "14" y2 = "4" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiColumns")]
        FiIcon::FiColumns => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiColumns".to_owned()) } < / title > < path d =
                "M12 3h7a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-7m0-18H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h7m0-18v18"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCommand")]
        FiIcon::FiCommand => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCommand".to_owned()) } < / title > < path d =
                "M18 3a3 3 0 0 0-3 3v12a3 3 0 0 0 3 3 3 3 0 0 0 3-3 3 3 0 0 0-3-3H6a3 3 0 0 0-3 3 3 3 0 0 0 3 3 3 3 0 0 0 3-3V6a3 3 0 0 0-3-3 3 3 0 0 0-3 3 3 3 0 0 0 3 3h12a3 3 0 0 0 3-3 3 3 0 0 0-3-3z"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCompass")]
        FiIcon::FiCompass => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCompass".to_owned()) } < / title > < circle cx = "12" cy = "12" r =
                "10" /> < polygon points =
                "16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCopy")]
        FiIcon::FiCopy => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCopy".to_owned()) } < / title > < rect x = "9" y = "9" width = "13"
                height = "13" rx = "2" ry = "2" /> < path d =
                "M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCornerDownLeft")]
        FiIcon::FiCornerDownLeft => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCornerDownLeft".to_owned()) } < / title > < polyline points =
                "9 10 4 15 9 20" /> < path d = "M20 4v7a4 4 0 0 1-4 4H4" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCornerDownRight")]
        FiIcon::FiCornerDownRight => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCornerDownRight".to_owned()) } < / title > < polyline points =
                "15 10 20 15 15 20" /> < path d = "M4 4v7a4 4 0 0 0 4 4h12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCornerLeftDown")]
        FiIcon::FiCornerLeftDown => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCornerLeftDown".to_owned()) } < / title > < polyline points =
                "14 15 9 20 4 15" /> < path d = "M20 4h-7a4 4 0 0 0-4 4v12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCornerLeftUp")]
        FiIcon::FiCornerLeftUp => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCornerLeftUp".to_owned()) } < / title > < polyline points =
                "14 9 9 4 4 9" /> < path d = "M20 20h-7a4 4 0 0 1-4-4V4" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCornerRightDown")]
        FiIcon::FiCornerRightDown => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCornerRightDown".to_owned()) } < / title > < polyline points =
                "10 15 15 20 20 15" /> < path d = "M4 4h7a4 4 0 0 1 4 4v12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCornerRightUp")]
        FiIcon::FiCornerRightUp => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCornerRightUp".to_owned()) } < / title > < polyline points =
                "10 9 15 4 20 9" /> < path d = "M4 20h7a4 4 0 0 0 4-4V4" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCornerUpLeft")]
        FiIcon::FiCornerUpLeft => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCornerUpLeft".to_owned()) } < / title > < polyline points =
                "9 14 4 9 9 4" /> < path d = "M20 20v-7a4 4 0 0 0-4-4H4" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCornerUpRight")]
        FiIcon::FiCornerUpRight => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCornerUpRight".to_owned()) } < / title > < polyline points =
                "15 14 20 9 15 4" /> < path d = "M4 20v-7a4 4 0 0 1 4-4h12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCpu")]
        FiIcon::FiCpu => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCpu".to_owned()) } < / title > < rect x = "4" y = "4" width = "16"
                height = "16" rx = "2" ry = "2" /> < rect x = "9" y = "9" width = "6"
                height = "6" /> < line x1 = "9" y1 = "1" x2 = "9" y2 = "4" /> < line x1 =
                "15" y1 = "1" x2 = "15" y2 = "4" /> < line x1 = "9" y1 = "20" x2 = "9" y2
                = "23" /> < line x1 = "15" y1 = "20" x2 = "15" y2 = "23" /> < line x1 =
                "20" y1 = "9" x2 = "23" y2 = "9" /> < line x1 = "20" y1 = "14" x2 = "23"
                y2 = "14" /> < line x1 = "1" y1 = "9" x2 = "4" y2 = "9" /> < line x1 =
                "1" y1 = "14" x2 = "4" y2 = "14" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCreditCard")]
        FiIcon::FiCreditCard => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCreditCard".to_owned()) } < / title > < rect x = "1" y = "4" width =
                "22" height = "16" rx = "2" ry = "2" /> < line x1 = "1" y1 = "10" x2 =
                "23" y2 = "10" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCrop")]
        FiIcon::FiCrop => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCrop".to_owned()) } < / title > < path d =
                "M6.13 1L6 16a2 2 0 0 0 2 2h15" /> < path d =
                "M1 6.13L16 6a2 2 0 0 1 2 2v15" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiCrosshair")]
        FiIcon::FiCrosshair => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiCrosshair".to_owned()) } < / title > < circle cx = "12" cy = "12" r =
                "10" /> < line x1 = "22" y1 = "12" x2 = "18" y2 = "12" /> < line x1 = "6"
                y1 = "12" x2 = "2" y2 = "12" /> < line x1 = "12" y1 = "6" x2 = "12" y2 =
                "2" /> < line x1 = "12" y1 = "22" x2 = "12" y2 = "18" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiDatabase")]
        FiIcon::FiDatabase => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiDatabase".to_owned()) } < / title > < ellipse cx = "12" cy = "5" rx =
                "9" ry = "3" /> < path d = "M21 12c0 1.66-4 3-9 3s-9-1.34-9-3" /> < path
                d = "M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiDelete")]
        FiIcon::FiDelete => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiDelete".to_owned()) } < / title > < path d =
                "M21 4H8l-7 8 7 8h13a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2z" /> < line x1 = "18"
                y1 = "9" x2 = "12" y2 = "15" /> < line x1 = "12" y1 = "9" x2 = "18" y2 =
                "15" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiDisc")]
        FiIcon::FiDisc => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiDisc".to_owned()) } < / title > < circle cx = "12" cy = "12" r = "10"
                /> < circle cx = "12" cy = "12" r = "3" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiDivide")]
        FiIcon::FiDivide => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiDivide".to_owned()) } < / title > < circle cx = "12" cy = "6" r = "2"
                /> < line x1 = "5" y1 = "12" x2 = "19" y2 = "12" /> < circle cx = "12" cy
                = "18" r = "2" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiDivideCircle")]
        FiIcon::FiDivideCircle => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiDivideCircle".to_owned()) } < / title > < line x1 = "8" y1 = "12" x2 =
                "16" y2 = "12" /> < line x1 = "12" y1 = "16" x2 = "12" y2 = "16" /> <
                line x1 = "12" y1 = "8" x2 = "12" y2 = "8" /> < circle cx = "12" cy =
                "12" r = "10" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiDivideSquare")]
        FiIcon::FiDivideSquare => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiDivideSquare".to_owned()) } < / title > < rect x = "3" y = "3" width =
                "18" height = "18" rx = "2" ry = "2" /> < line x1 = "8" y1 = "12" x2 =
                "16" y2 = "12" /> < line x1 = "12" y1 = "16" x2 = "12" y2 = "16" /> <
                line x1 = "12" y1 = "8" x2 = "12" y2 = "8" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiDollarSign")]
        FiIcon::FiDollarSign => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiDollarSign".to_owned()) } < / title > < line x1 = "12" y1 = "1" x2 =
                "12" y2 = "23" /> < path d =
                "M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiDownload")]
        FiIcon::FiDownload => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiDownload".to_owned()) } < / title > < path d =
                "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" /> < polyline points =
                "7 10 12 15 17 10" /> < line x1 = "12" y1 = "15" x2 = "12" y2 = "3" /> <
                / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiDownloadCloud")]
        FiIcon::FiDownloadCloud => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiDownloadCloud".to_owned()) } < / title > < polyline points =
                "8 17 12 21 16 17" /> < line x1 = "12" y1 = "12" x2 = "12" y2 = "21" /> <
                path d = "M20.88 18.09A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.29" /> < / svg
                >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiDribbble")]
        FiIcon::FiDribbble => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiDribbble".to_owned()) } < / title > < circle cx = "12" cy = "12" r =
                "10" /> < path d =
                "M8.56 2.75c4.37 6.03 6.02 9.42 8.03 17.72m2.54-15.38c-3.72 4.35-8.94 5.66-16.88 5.85m19.5 1.9c-3.5-.93-6.63-.82-8.94 0-2.58.92-5.01 2.86-7.44 6.32"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiDroplet")]
        FiIcon::FiDroplet => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiDroplet".to_owned()) } < / title > < path d =
                "M12 2.69l5.66 5.66a8 8 0 1 1-11.31 0z" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiEdit")]
        FiIcon::FiEdit => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiEdit".to_owned()) } < / title > < path d =
                "M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7" /> < path d
                = "M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiEdit2")]
        FiIcon::FiEdit2 => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiEdit2".to_owned()) } < / title > < path d =
                "M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiEdit3")]
        FiIcon::FiEdit3 => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiEdit3".to_owned()) } < / title > < path d = "M12 20h9" /> < path d =
                "M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiExternalLink")]
        FiIcon::FiExternalLink => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiExternalLink".to_owned()) } < / title > < path d =
                "M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6" /> < polyline
                points = "15 3 21 3 21 9" /> < line x1 = "10" y1 = "14" x2 = "21" y2 =
                "3" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiEye")]
        FiIcon::FiEye => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiEye".to_owned()) } < / title > < path d =
                "M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z" /> < circle cx = "12" cy =
                "12" r = "3" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiEyeOff")]
        FiIcon::FiEyeOff => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiEyeOff".to_owned()) } < / title > < path d =
                "M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24"
                /> < line x1 = "1" y1 = "1" x2 = "23" y2 = "23" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiFacebook")]
        FiIcon::FiFacebook => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiFacebook".to_owned()) } < / title > < path d =
                "M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z" /> <
                / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiFastForward")]
        FiIcon::FiFastForward => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiFastForward".to_owned()) } < / title > < polygon points =
                "13 19 22 12 13 5 13 19" /> < polygon points = "2 19 11 12 2 5 2 19" /> <
                / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiFeather")]
        FiIcon::FiFeather => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiFeather".to_owned()) } < / title > < path d =
                "M20.24 12.24a6 6 0 0 0-8.49-8.49L5 10.5V19h8.5z" /> < line x1 = "16" y1
                = "8" x2 = "2" y2 = "22" /> < line x1 = "17.5" y1 = "15" x2 = "9" y2 =
                "15" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiFigma")]
        FiIcon::FiFigma => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiFigma".to_owned()) } < / title > < path d =
                "M5 5.5A3.5 3.5 0 0 1 8.5 2H12v7H8.5A3.5 3.5 0 0 1 5 5.5z" /> < path d =
                "M12 2h3.5a3.5 3.5 0 1 1 0 7H12V2z" /> < path d =
                "M12 12.5a3.5 3.5 0 1 1 7 0 3.5 3.5 0 1 1-7 0z" /> < path d =
                "M5 19.5A3.5 3.5 0 0 1 8.5 16H12v3.5a3.5 3.5 0 1 1-7 0z" /> < path d =
                "M5 12.5A3.5 3.5 0 0 1 8.5 9H12v7H8.5A3.5 3.5 0 0 1 5 12.5z" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiFile")]
        FiIcon::FiFile => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiFile".to_owned()) } < / title > < path d =
                "M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z" /> <
                polyline points = "13 2 13 9 20 9" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiFileMinus")]
        FiIcon::FiFileMinus => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiFileMinus".to_owned()) } < / title > < path d =
                "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" /> <
                polyline points = "14 2 14 8 20 8" /> < line x1 = "9" y1 = "15" x2 = "15"
                y2 = "15" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiFilePlus")]
        FiIcon::FiFilePlus => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiFilePlus".to_owned()) } < / title > < path d =
                "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" /> <
                polyline points = "14 2 14 8 20 8" /> < line x1 = "12" y1 = "18" x2 =
                "12" y2 = "12" /> < line x1 = "9" y1 = "15" x2 = "15" y2 = "15" /> < /
                svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiFileText")]
        FiIcon::FiFileText => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiFileText".to_owned()) } < / title > < path d =
                "M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z" /> <
                polyline points = "14 2 14 8 20 8" /> < line x1 = "16" y1 = "13" x2 = "8"
                y2 = "13" /> < line x1 = "16" y1 = "17" x2 = "8" y2 = "17" /> < polyline
                points = "10 9 9 9 8 9" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiFilm")]
        FiIcon::FiFilm => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiFilm".to_owned()) } < / title > < rect x = "2" y = "2" width = "20"
                height = "20" rx = "2.18" ry = "2.18" /> < line x1 = "7" y1 = "2" x2 =
                "7" y2 = "22" /> < line x1 = "17" y1 = "2" x2 = "17" y2 = "22" /> < line
                x1 = "2" y1 = "12" x2 = "22" y2 = "12" /> < line x1 = "2" y1 = "7" x2 =
                "7" y2 = "7" /> < line x1 = "2" y1 = "17" x2 = "7" y2 = "17" /> < line x1
                = "17" y1 = "17" x2 = "22" y2 = "17" /> < line x1 = "17" y1 = "7" x2 =
                "22" y2 = "7" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiFilter")]
        FiIcon::FiFilter => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiFilter".to_owned()) } < / title > < polygon points =
                "22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiFlag")]
        FiIcon::FiFlag => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiFlag".to_owned()) } < / title > < path d =
                "M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1z" /> < line x1
                = "4" y1 = "22" x2 = "4" y2 = "15" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiFolder")]
        FiIcon::FiFolder => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiFolder".to_owned()) } < / title > < path d =
                "M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiFolderMinus")]
        FiIcon::FiFolderMinus => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiFolderMinus".to_owned()) } < / title > < path d =
                "M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"
                /> < line x1 = "9" y1 = "14" x2 = "15" y2 = "14" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiFolderPlus")]
        FiIcon::FiFolderPlus => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiFolderPlus".to_owned()) } < / title > < path d =
                "M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z"
                /> < line x1 = "12" y1 = "11" x2 = "12" y2 = "17" /> < line x1 = "9" y1 =
                "14" x2 = "15" y2 = "14" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiFramer")]
        FiIcon::FiFramer => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiFramer".to_owned()) } < / title > < path d =
                "M5 16V9h14V2H5l14 14h-7m-7 0l7 7v-7m-7 0h7" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiFrown")]
        FiIcon::FiFrown => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiFrown".to_owned()) } < / title > < circle cx = "12" cy = "12" r = "10"
                /> < path d = "M16 16s-1.5-2-4-2-4 2-4 2" /> < line x1 = "9" y1 = "9" x2
                = "9.01" y2 = "9" /> < line x1 = "15" y1 = "9" x2 = "15.01" y2 = "9" /> <
                / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiGift")]
        FiIcon::FiGift => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiGift".to_owned()) } < / title > < polyline points =
                "20 12 20 22 4 22 4 12" /> < rect x = "2" y = "7" width = "20" height =
                "5" /> < line x1 = "12" y1 = "22" x2 = "12" y2 = "7" /> < path d =
                "M12 7H7.5a2.5 2.5 0 0 1 0-5C11 2 12 7 12 7z" /> < path d =
                "M12 7h4.5a2.5 2.5 0 0 0 0-5C13 2 12 7 12 7z" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiGitBranch")]
        FiIcon::FiGitBranch => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiGitBranch".to_owned()) } < / title > < line x1 = "6" y1 = "3" x2 = "6"
                y2 = "15" /> < circle cx = "18" cy = "6" r = "3" /> < circle cx = "6" cy
                = "18" r = "3" /> < path d = "M18 9a9 9 0 0 1-9 9" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiGitCommit")]
        FiIcon::FiGitCommit => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiGitCommit".to_owned()) } < / title > < circle cx = "12" cy = "12" r =
                "4" /> < line x1 = "1.05" y1 = "12" x2 = "7" y2 = "12" /> < line x1 =
                "17.01" y1 = "12" x2 = "22.96" y2 = "12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiGitMerge")]
        FiIcon::FiGitMerge => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiGitMerge".to_owned()) } < / title > < circle cx = "18" cy = "18" r =
                "3" /> < circle cx = "6" cy = "6" r = "3" /> < path d =
                "M6 21V9a9 9 0 0 0 9 9" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiGitPullRequest")]
        FiIcon::FiGitPullRequest => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiGitPullRequest".to_owned()) } < / title > < circle cx = "18" cy = "18"
                r = "3" /> < circle cx = "6" cy = "6" r = "3" /> < path d =
                "M13 6h3a2 2 0 0 1 2 2v7" /> < line x1 = "6" y1 = "9" x2 = "6" y2 = "21"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiGithub")]
        FiIcon::FiGithub => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiGithub".to_owned()) } < / title > < path d =
                "M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiGitlab")]
        FiIcon::FiGitlab => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiGitlab".to_owned()) } < / title > < path d =
                "M22.65 14.39L12 22.13 1.35 14.39a.84.84 0 0 1-.3-.94l1.22-3.78 2.44-7.51A.42.42 0 0 1 4.82 2a.43.43 0 0 1 .58 0 .42.42 0 0 1 .11.18l2.44 7.49h8.1l2.44-7.51A.42.42 0 0 1 18.6 2a.43.43 0 0 1 .58 0 .42.42 0 0 1 .11.18l2.44 7.51L23 13.45a.84.84 0 0 1-.35.94z"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiGlobe")]
        FiIcon::FiGlobe => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiGlobe".to_owned()) } < / title > < circle cx = "12" cy = "12" r = "10"
                /> < line x1 = "2" y1 = "12" x2 = "22" y2 = "12" /> < path d =
                "M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiGrid")]
        FiIcon::FiGrid => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiGrid".to_owned()) } < / title > < rect x = "3" y = "3" width = "7"
                height = "7" /> < rect x = "14" y = "3" width = "7" height = "7" /> <
                rect x = "14" y = "14" width = "7" height = "7" /> < rect x = "3" y =
                "14" width = "7" height = "7" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiHardDrive")]
        FiIcon::FiHardDrive => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiHardDrive".to_owned()) } < / title > < line x1 = "22" y1 = "12" x2 =
                "2" y2 = "12" /> < path d =
                "M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"
                /> < line x1 = "6" y1 = "16" x2 = "6.01" y2 = "16" /> < line x1 = "10" y1
                = "16" x2 = "10.01" y2 = "16" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiHash")]
        FiIcon::FiHash => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiHash".to_owned()) } < / title > < line x1 = "4" y1 = "9" x2 = "20" y2
                = "9" /> < line x1 = "4" y1 = "15" x2 = "20" y2 = "15" /> < line x1 =
                "10" y1 = "3" x2 = "8" y2 = "21" /> < line x1 = "16" y1 = "3" x2 = "14"
                y2 = "21" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiHeadphones")]
        FiIcon::FiHeadphones => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiHeadphones".to_owned()) } < / title > < path d =
                "M3 18v-6a9 9 0 0 1 18 0v6" /> < path d =
                "M21 19a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3zM3 19a2 2 0 0 0 2 2h1a2 2 0 0 0 2-2v-3a2 2 0 0 0-2-2H3z"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiHeart")]
        FiIcon::FiHeart => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiHeart".to_owned()) } < / title > < path d =
                "M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiHelpCircle")]
        FiIcon::FiHelpCircle => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiHelpCircle".to_owned()) } < / title > < circle cx = "12" cy = "12" r =
                "10" /> < path d = "M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3" /> < line x1 =
                "12" y1 = "17" x2 = "12.01" y2 = "17" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiHexagon")]
        FiIcon::FiHexagon => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiHexagon".to_owned()) } < / title > < path d =
                "M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiHome")]
        FiIcon::FiHome => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiHome".to_owned()) } < / title > < path d =
                "M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z" /> < polyline points =
                "9 22 9 12 15 12 15 22" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiImage")]
        FiIcon::FiImage => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiImage".to_owned()) } < / title > < rect x = "3" y = "3" width = "18"
                height = "18" rx = "2" ry = "2" /> < circle cx = "8.5" cy = "8.5" r =
                "1.5" /> < polyline points = "21 15 16 10 5 21" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiInbox")]
        FiIcon::FiInbox => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiInbox".to_owned()) } < / title > < polyline points =
                "22 12 16 12 14 15 10 15 8 12 2 12" /> < path d =
                "M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiInfo")]
        FiIcon::FiInfo => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiInfo".to_owned()) } < / title > < circle cx = "12" cy = "12" r = "10"
                /> < line x1 = "12" y1 = "16" x2 = "12" y2 = "12" /> < line x1 = "12" y1
                = "8" x2 = "12.01" y2 = "8" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiInstagram")]
        FiIcon::FiInstagram => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiInstagram".to_owned()) } < / title > < rect x = "2" y = "2" width =
                "20" height = "20" rx = "5" ry = "5" /> < path d =
                "M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z" /> < line x1 = "17.5"
                y1 = "6.5" x2 = "17.51" y2 = "6.5" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiItalic")]
        FiIcon::FiItalic => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiItalic".to_owned()) } < / title > < line x1 = "19" y1 = "4" x2 = "10"
                y2 = "4" /> < line x1 = "14" y1 = "20" x2 = "5" y2 = "20" /> < line x1 =
                "15" y1 = "4" x2 = "9" y2 = "20" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiKey")]
        FiIcon::FiKey => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiKey".to_owned()) } < / title > < path d =
                "M21 2l-2 2m-7.61 7.61a5.5 5.5 0 1 1-7.778 7.778 5.5 5.5 0 0 1 7.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiLayers")]
        FiIcon::FiLayers => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiLayers".to_owned()) } < / title > < polygon points =
                "12 2 2 7 12 12 22 7 12 2" /> < polyline points = "2 17 12 22 22 17" /> <
                polyline points = "2 12 12 17 22 12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiLayout")]
        FiIcon::FiLayout => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiLayout".to_owned()) } < / title > < rect x = "3" y = "3" width = "18"
                height = "18" rx = "2" ry = "2" /> < line x1 = "3" y1 = "9" x2 = "21" y2
                = "9" /> < line x1 = "9" y1 = "21" x2 = "9" y2 = "9" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiLifeBuoy")]
        FiIcon::FiLifeBuoy => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiLifeBuoy".to_owned()) } < / title > < circle cx = "12" cy = "12" r =
                "10" /> < circle cx = "12" cy = "12" r = "4" /> < line x1 = "4.93" y1 =
                "4.93" x2 = "9.17" y2 = "9.17" /> < line x1 = "14.83" y1 = "14.83" x2 =
                "19.07" y2 = "19.07" /> < line x1 = "14.83" y1 = "9.17" x2 = "19.07" y2 =
                "4.93" /> < line x1 = "14.83" y1 = "9.17" x2 = "18.36" y2 = "5.64" /> <
                line x1 = "4.93" y1 = "19.07" x2 = "9.17" y2 = "14.83" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiLink")]
        FiIcon::FiLink => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiLink".to_owned()) } < / title > < path d =
                "M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71" /> < path d
                = "M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71" /> < /
                svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiLink2")]
        FiIcon::FiLink2 => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiLink2".to_owned()) } < / title > < path d =
                "M15 7h3a5 5 0 0 1 5 5 5 5 0 0 1-5 5h-3m-6 0H6a5 5 0 0 1-5-5 5 5 0 0 1 5-5h3"
                /> < line x1 = "8" y1 = "12" x2 = "16" y2 = "12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiLinkedin")]
        FiIcon::FiLinkedin => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiLinkedin".to_owned()) } < / title > < path d =
                "M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z"
                /> < rect x = "2" y = "9" width = "4" height = "12" /> < circle cx = "4"
                cy = "4" r = "2" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiList")]
        FiIcon::FiList => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiList".to_owned()) } < / title > < line x1 = "8" y1 = "6" x2 = "21" y2
                = "6" /> < line x1 = "8" y1 = "12" x2 = "21" y2 = "12" /> < line x1 = "8"
                y1 = "18" x2 = "21" y2 = "18" /> < line x1 = "3" y1 = "6" x2 = "3.01" y2
                = "6" /> < line x1 = "3" y1 = "12" x2 = "3.01" y2 = "12" /> < line x1 =
                "3" y1 = "18" x2 = "3.01" y2 = "18" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiLoader")]
        FiIcon::FiLoader => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiLoader".to_owned()) } < / title > < line x1 = "12" y1 = "2" x2 = "12"
                y2 = "6" /> < line x1 = "12" y1 = "18" x2 = "12" y2 = "22" /> < line x1 =
                "4.93" y1 = "4.93" x2 = "7.76" y2 = "7.76" /> < line x1 = "16.24" y1 =
                "16.24" x2 = "19.07" y2 = "19.07" /> < line x1 = "2" y1 = "12" x2 = "6"
                y2 = "12" /> < line x1 = "18" y1 = "12" x2 = "22" y2 = "12" /> < line x1
                = "4.93" y1 = "19.07" x2 = "7.76" y2 = "16.24" /> < line x1 = "16.24" y1
                = "7.76" x2 = "19.07" y2 = "4.93" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiLock")]
        FiIcon::FiLock => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiLock".to_owned()) } < / title > < rect x = "3" y = "11" width = "18"
                height = "11" rx = "2" ry = "2" /> < path d = "M7 11V7a5 5 0 0 1 10 0v4"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiLogIn")]
        FiIcon::FiLogIn => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiLogIn".to_owned()) } < / title > < path d =
                "M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4" /> < polyline points =
                "10 17 15 12 10 7" /> < line x1 = "15" y1 = "12" x2 = "3" y2 = "12" /> <
                / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiLogOut")]
        FiIcon::FiLogOut => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiLogOut".to_owned()) } < / title > < path d =
                "M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" /> < polyline points =
                "16 17 21 12 16 7" /> < line x1 = "21" y1 = "12" x2 = "9" y2 = "12" /> <
                / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMail")]
        FiIcon::FiMail => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMail".to_owned()) } < / title > < path d =
                "M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z"
                /> < polyline points = "22,6 12,13 2,6" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMap")]
        FiIcon::FiMap => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMap".to_owned()) } < / title > < polygon points =
                "1 6 1 22 8 18 16 22 23 18 23 2 16 6 8 2 1 6" /> < line x1 = "8" y1 = "2"
                x2 = "8" y2 = "18" /> < line x1 = "16" y1 = "6" x2 = "16" y2 = "22" /> <
                / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMapPin")]
        FiIcon::FiMapPin => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMapPin".to_owned()) } < / title > < path d =
                "M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z" /> < circle cx = "12" cy
                = "10" r = "3" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMaximize")]
        FiIcon::FiMaximize => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMaximize".to_owned()) } < / title > < path d =
                "M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMaximize2")]
        FiIcon::FiMaximize2 => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMaximize2".to_owned()) } < / title > < polyline points =
                "15 3 21 3 21 9" /> < polyline points = "9 21 3 21 3 15" /> < line x1 =
                "21" y1 = "3" x2 = "14" y2 = "10" /> < line x1 = "3" y1 = "21" x2 = "10"
                y2 = "14" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMeh")]
        FiIcon::FiMeh => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMeh".to_owned()) } < / title > < circle cx = "12" cy = "12" r = "10"
                /> < line x1 = "8" y1 = "15" x2 = "16" y2 = "15" /> < line x1 = "9" y1 =
                "9" x2 = "9.01" y2 = "9" /> < line x1 = "15" y1 = "9" x2 = "15.01" y2 =
                "9" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMenu")]
        FiIcon::FiMenu => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMenu".to_owned()) } < / title > < line x1 = "3" y1 = "12" x2 = "21" y2
                = "12" /> < line x1 = "3" y1 = "6" x2 = "21" y2 = "6" /> < line x1 = "3"
                y1 = "18" x2 = "21" y2 = "18" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMessageCircle")]
        FiIcon::FiMessageCircle => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMessageCircle".to_owned()) } < / title > < path d =
                "M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMessageSquare")]
        FiIcon::FiMessageSquare => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMessageSquare".to_owned()) } < / title > < path d =
                "M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z" /> < /
                svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMic")]
        FiIcon::FiMic => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMic".to_owned()) } < / title > < path d =
                "M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z" /> < path d =
                "M19 10v2a7 7 0 0 1-14 0v-2" /> < line x1 = "12" y1 = "19" x2 = "12" y2 =
                "23" /> < line x1 = "8" y1 = "23" x2 = "16" y2 = "23" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMicOff")]
        FiIcon::FiMicOff => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMicOff".to_owned()) } < / title > < line x1 = "1" y1 = "1" x2 = "23"
                y2 = "23" /> < path d =
                "M9 9v3a3 3 0 0 0 5.12 2.12M15 9.34V4a3 3 0 0 0-5.94-.6" /> < path d =
                "M17 16.95A7 7 0 0 1 5 12v-2m14 0v2a7 7 0 0 1-.11 1.23" /> < line x1 =
                "12" y1 = "19" x2 = "12" y2 = "23" /> < line x1 = "8" y1 = "23" x2 = "16"
                y2 = "23" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMinimize")]
        FiIcon::FiMinimize => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMinimize".to_owned()) } < / title > < path d =
                "M8 3v3a2 2 0 0 1-2 2H3m18 0h-3a2 2 0 0 1-2-2V3m0 18v-3a2 2 0 0 1 2-2h3M3 16h3a2 2 0 0 1 2 2v3"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMinimize2")]
        FiIcon::FiMinimize2 => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMinimize2".to_owned()) } < / title > < polyline points =
                "4 14 10 14 10 20" /> < polyline points = "20 10 14 10 14 4" /> < line x1
                = "14" y1 = "10" x2 = "21" y2 = "3" /> < line x1 = "3" y1 = "21" x2 =
                "10" y2 = "14" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMinus")]
        FiIcon::FiMinus => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMinus".to_owned()) } < / title > < line x1 = "5" y1 = "12" x2 = "19"
                y2 = "12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMinusCircle")]
        FiIcon::FiMinusCircle => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMinusCircle".to_owned()) } < / title > < circle cx = "12" cy = "12" r
                = "10" /> < line x1 = "8" y1 = "12" x2 = "16" y2 = "12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMinusSquare")]
        FiIcon::FiMinusSquare => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMinusSquare".to_owned()) } < / title > < rect x = "3" y = "3" width =
                "18" height = "18" rx = "2" ry = "2" /> < line x1 = "8" y1 = "12" x2 =
                "16" y2 = "12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMonitor")]
        FiIcon::FiMonitor => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMonitor".to_owned()) } < / title > < rect x = "2" y = "3" width = "20"
                height = "14" rx = "2" ry = "2" /> < line x1 = "8" y1 = "21" x2 = "16" y2
                = "21" /> < line x1 = "12" y1 = "17" x2 = "12" y2 = "21" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMoon")]
        FiIcon::FiMoon => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMoon".to_owned()) } < / title > < path d =
                "M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMoreHorizontal")]
        FiIcon::FiMoreHorizontal => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMoreHorizontal".to_owned()) } < / title > < circle cx = "12" cy = "12"
                r = "1" /> < circle cx = "19" cy = "12" r = "1" /> < circle cx = "5" cy =
                "12" r = "1" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMoreVertical")]
        FiIcon::FiMoreVertical => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMoreVertical".to_owned()) } < / title > < circle cx = "12" cy = "12" r
                = "1" /> < circle cx = "12" cy = "5" r = "1" /> < circle cx = "12" cy =
                "19" r = "1" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMousePointer")]
        FiIcon::FiMousePointer => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMousePointer".to_owned()) } < / title > < path d =
                "M3 3l7.07 16.97 2.51-7.39 7.39-2.51L3 3z" /> < path d = "M13 13l6 6" />
                < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMove")]
        FiIcon::FiMove => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMove".to_owned()) } < / title > < polyline points = "5 9 2 12 5 15" />
                < polyline points = "9 5 12 2 15 5" /> < polyline points =
                "15 19 12 22 9 19" /> < polyline points = "19 9 22 12 19 15" /> < line x1
                = "2" y1 = "12" x2 = "22" y2 = "12" /> < line x1 = "12" y1 = "2" x2 =
                "12" y2 = "22" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiMusic")]
        FiIcon::FiMusic => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiMusic".to_owned()) } < / title > < path d = "M9 18V5l12-2v13" /> <
                circle cx = "6" cy = "18" r = "3" /> < circle cx = "18" cy = "16" r = "3"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiNavigation")]
        FiIcon::FiNavigation => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiNavigation".to_owned()) } < / title > < polygon points =
                "3 11 22 2 13 21 11 13 3 11" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiNavigation2")]
        FiIcon::FiNavigation2 => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiNavigation2".to_owned()) } < / title > < polygon points =
                "12 2 19 21 12 17 5 21 12 2" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiOctagon")]
        FiIcon::FiOctagon => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiOctagon".to_owned()) } < / title > < polygon points =
                "7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPackage")]
        FiIcon::FiPackage => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPackage".to_owned()) } < / title > < line x1 = "16.5" y1 = "9.4" x2 =
                "7.5" y2 = "4.21" /> < path d =
                "M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z"
                /> < polyline points = "3.27 6.96 12 12.01 20.73 6.96" /> < line x1 =
                "12" y1 = "22.08" x2 = "12" y2 = "12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPaperclip")]
        FiIcon::FiPaperclip => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPaperclip".to_owned()) } < / title > < path d =
                "M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPause")]
        FiIcon::FiPause => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPause".to_owned()) } < / title > < rect x = "6" y = "4" width = "4"
                height = "16" /> < rect x = "14" y = "4" width = "4" height = "16" /> < /
                svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPauseCircle")]
        FiIcon::FiPauseCircle => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPauseCircle".to_owned()) } < / title > < circle cx = "12" cy = "12" r
                = "10" /> < line x1 = "10" y1 = "15" x2 = "10" y2 = "9" /> < line x1 =
                "14" y1 = "15" x2 = "14" y2 = "9" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPenTool")]
        FiIcon::FiPenTool => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPenTool".to_owned()) } < / title > < path d =
                "M12 19l7-7 3 3-7 7-3-3z" /> < path d =
                "M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z" /> < path d =
                "M2 2l7.586 7.586" /> < circle cx = "11" cy = "11" r = "2" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPercent")]
        FiIcon::FiPercent => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPercent".to_owned()) } < / title > < line x1 = "19" y1 = "5" x2 = "5"
                y2 = "19" /> < circle cx = "6.5" cy = "6.5" r = "2.5" /> < circle cx =
                "17.5" cy = "17.5" r = "2.5" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPhone")]
        FiIcon::FiPhone => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPhone".to_owned()) } < / title > < path d =
                "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPhoneCall")]
        FiIcon::FiPhoneCall => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPhoneCall".to_owned()) } < / title > < path d =
                "M15.05 5A5 5 0 0 1 19 8.95M15.05 1A9 9 0 0 1 23 8.94m-1 7.98v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPhoneForwarded")]
        FiIcon::FiPhoneForwarded => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPhoneForwarded".to_owned()) } < / title > < polyline points =
                "19 1 23 5 19 9" /> < line x1 = "15" y1 = "5" x2 = "23" y2 = "5" /> <
                path d =
                "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPhoneIncoming")]
        FiIcon::FiPhoneIncoming => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPhoneIncoming".to_owned()) } < / title > < polyline points =
                "16 2 16 8 22 8" /> < line x1 = "23" y1 = "1" x2 = "16" y2 = "8" /> <
                path d =
                "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPhoneMissed")]
        FiIcon::FiPhoneMissed => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPhoneMissed".to_owned()) } < / title > < line x1 = "23" y1 = "1" x2 =
                "17" y2 = "7" /> < line x1 = "17" y1 = "1" x2 = "23" y2 = "7" /> < path d
                =
                "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPhoneOff")]
        FiIcon::FiPhoneOff => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPhoneOff".to_owned()) } < / title > < path d =
                "M10.68 13.31a16 16 0 0 0 3.41 2.6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7 2 2 0 0 1 1.72 2v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.42 19.42 0 0 1-3.33-2.67m-2.67-3.34a19.79 19.79 0 0 1-3.07-8.63A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91"
                /> < line x1 = "23" y1 = "1" x2 = "1" y2 = "23" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPhoneOutgoing")]
        FiIcon::FiPhoneOutgoing => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPhoneOutgoing".to_owned()) } < / title > < polyline points =
                "23 7 23 1 17 1" /> < line x1 = "16" y1 = "8" x2 = "23" y2 = "1" /> <
                path d =
                "M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPieChart")]
        FiIcon::FiPieChart => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPieChart".to_owned()) } < / title > < path d =
                "M21.21 15.89A10 10 0 1 1 8 2.83" /> < path d =
                "M22 12A10 10 0 0 0 12 2v10z" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPlay")]
        FiIcon::FiPlay => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPlay".to_owned()) } < / title > < polygon points =
                "5 3 19 12 5 21 5 3" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPlayCircle")]
        FiIcon::FiPlayCircle => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPlayCircle".to_owned()) } < / title > < circle cx = "12" cy = "12" r =
                "10" /> < polygon points = "10 8 16 12 10 16 10 8" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPlus")]
        FiIcon::FiPlus => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPlus".to_owned()) } < / title > < line x1 = "12" y1 = "5" x2 = "12" y2
                = "19" /> < line x1 = "5" y1 = "12" x2 = "19" y2 = "12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPlusCircle")]
        FiIcon::FiPlusCircle => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPlusCircle".to_owned()) } < / title > < circle cx = "12" cy = "12" r =
                "10" /> < line x1 = "12" y1 = "8" x2 = "12" y2 = "16" /> < line x1 = "8"
                y1 = "12" x2 = "16" y2 = "12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPlusSquare")]
        FiIcon::FiPlusSquare => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPlusSquare".to_owned()) } < / title > < rect x = "3" y = "3" width =
                "18" height = "18" rx = "2" ry = "2" /> < line x1 = "12" y1 = "8" x2 =
                "12" y2 = "16" /> < line x1 = "8" y1 = "12" x2 = "16" y2 = "12" /> < /
                svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPocket")]
        FiIcon::FiPocket => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPocket".to_owned()) } < / title > < path d =
                "M4 3h16a2 2 0 0 1 2 2v6a10 10 0 0 1-10 10A10 10 0 0 1 2 11V5a2 2 0 0 1 2-2z"
                /> < polyline points = "8 10 12 14 16 10" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPower")]
        FiIcon::FiPower => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPower".to_owned()) } < / title > < path d =
                "M18.36 6.64a9 9 0 1 1-12.73 0" /> < line x1 = "12" y1 = "2" x2 = "12" y2
                = "12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiPrinter")]
        FiIcon::FiPrinter => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiPrinter".to_owned()) } < / title > < polyline points =
                "6 9 6 2 18 2 18 9" /> < path d =
                "M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2"
                /> < rect x = "6" y = "14" width = "12" height = "8" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiRadio")]
        FiIcon::FiRadio => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiRadio".to_owned()) } < / title > < circle cx = "12" cy = "12" r = "2"
                /> < path d =
                "M16.24 7.76a6 6 0 0 1 0 8.49m-8.48-.01a6 6 0 0 1 0-8.49m11.31-2.82a10 10 0 0 1 0 14.14m-14.14 0a10 10 0 0 1 0-14.14"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiRefreshCcw")]
        FiIcon::FiRefreshCcw => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiRefreshCcw".to_owned()) } < / title > < polyline points =
                "1 4 1 10 7 10" /> < polyline points = "23 20 23 14 17 14" /> < path d =
                "M20.49 9A9 9 0 0 0 5.64 5.64L1 10m22 4l-4.64 4.36A9 9 0 0 1 3.51 15" />
                < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiRefreshCw")]
        FiIcon::FiRefreshCw => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiRefreshCw".to_owned()) } < / title > < polyline points =
                "23 4 23 10 17 10" /> < polyline points = "1 20 1 14 7 14" /> < path d =
                "M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15" />
                < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiRepeat")]
        FiIcon::FiRepeat => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiRepeat".to_owned()) } < / title > < polyline points = "17 1 21 5 17 9"
                /> < path d = "M3 11V9a4 4 0 0 1 4-4h14" /> < polyline points =
                "7 23 3 19 7 15" /> < path d = "M21 13v2a4 4 0 0 1-4 4H3" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiRewind")]
        FiIcon::FiRewind => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiRewind".to_owned()) } < / title > < polygon points =
                "11 19 2 12 11 5 11 19" /> < polygon points = "22 19 13 12 22 5 22 19" />
                < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiRotateCcw")]
        FiIcon::FiRotateCcw => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiRotateCcw".to_owned()) } < / title > < polyline points =
                "1 4 1 10 7 10" /> < path d = "M3.51 15a9 9 0 1 0 2.13-9.36L1 10" /> < /
                svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiRotateCw")]
        FiIcon::FiRotateCw => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiRotateCw".to_owned()) } < / title > < polyline points =
                "23 4 23 10 17 10" /> < path d = "M20.49 15a9 9 0 1 1-2.12-9.36L23 10" />
                < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiRss")]
        FiIcon::FiRss => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiRss".to_owned()) } < / title > < path d = "M4 11a9 9 0 0 1 9 9" /> <
                path d = "M4 4a16 16 0 0 1 16 16" /> < circle cx = "5" cy = "19" r = "1"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiSave")]
        FiIcon::FiSave => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiSave".to_owned()) } < / title > < path d =
                "M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z" /> <
                polyline points = "17 21 17 13 7 13 7 21" /> < polyline points =
                "7 3 7 8 15 8" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiScissors")]
        FiIcon::FiScissors => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiScissors".to_owned()) } < / title > < circle cx = "6" cy = "6" r = "3"
                /> < circle cx = "6" cy = "18" r = "3" /> < line x1 = "20" y1 = "4" x2 =
                "8.12" y2 = "15.88" /> < line x1 = "14.47" y1 = "14.48" x2 = "20" y2 =
                "20" /> < line x1 = "8.12" y1 = "8.12" x2 = "12" y2 = "12" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiSearch")]
        FiIcon::FiSearch => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiSearch".to_owned()) } < / title > < circle cx = "11" cy = "11" r = "8"
                /> < line x1 = "21" y1 = "21" x2 = "16.65" y2 = "16.65" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiSend")]
        FiIcon::FiSend => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiSend".to_owned()) } < / title > < line x1 = "22" y1 = "2" x2 = "11" y2
                = "13" /> < polygon points = "22 2 15 22 11 13 2 9 22 2" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiServer")]
        FiIcon::FiServer => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiServer".to_owned()) } < / title > < rect x = "2" y = "2" width = "20"
                height = "8" rx = "2" ry = "2" /> < rect x = "2" y = "14" width = "20"
                height = "8" rx = "2" ry = "2" /> < line x1 = "6" y1 = "6" x2 = "6.01" y2
                = "6" /> < line x1 = "6" y1 = "18" x2 = "6.01" y2 = "18" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiSettings")]
        FiIcon::FiSettings => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiSettings".to_owned()) } < / title > < circle cx = "12" cy = "12" r =
                "3" /> < path d =
                "M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiShare")]
        FiIcon::FiShare => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiShare".to_owned()) } < / title > < path d =
                "M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8" /> < polyline points =
                "16 6 12 2 8 6" /> < line x1 = "12" y1 = "2" x2 = "12" y2 = "15" /> < /
                svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiShare2")]
        FiIcon::FiShare2 => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiShare2".to_owned()) } < / title > < circle cx = "18" cy = "5" r = "3"
                /> < circle cx = "6" cy = "12" r = "3" /> < circle cx = "18" cy = "19" r
                = "3" /> < line x1 = "8.59" y1 = "13.51" x2 = "15.42" y2 = "17.49" /> <
                line x1 = "15.41" y1 = "6.51" x2 = "8.59" y2 = "10.49" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiShield")]
        FiIcon::FiShield => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiShield".to_owned()) } < / title > < path d =
                "M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiShieldOff")]
        FiIcon::FiShieldOff => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiShieldOff".to_owned()) } < / title > < path d =
                "M19.69 14a6.9 6.9 0 0 0 .31-2V5l-8-3-3.16 1.18" /> < path d =
                "M4.73 4.73L4 5v7c0 6 8 10 8 10a20.29 20.29 0 0 0 5.62-4.38" /> < line x1
                = "1" y1 = "1" x2 = "23" y2 = "23" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiShoppingBag")]
        FiIcon::FiShoppingBag => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiShoppingBag".to_owned()) } < / title > < path d =
                "M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4z" /> < line x1 = "3"
                y1 = "6" x2 = "21" y2 = "6" /> < path d = "M16 10a4 4 0 0 1-8 0" /> < /
                svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiShoppingCart")]
        FiIcon::FiShoppingCart => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiShoppingCart".to_owned()) } < / title > < circle cx = "9" cy = "21" r
                = "1" /> < circle cx = "20" cy = "21" r = "1" /> < path d =
                "M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6" /> < /
                svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiShuffle")]
        FiIcon::FiShuffle => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiShuffle".to_owned()) } < / title > < polyline points =
                "16 3 21 3 21 8" /> < line x1 = "4" y1 = "20" x2 = "21" y2 = "3" /> <
                polyline points = "21 16 21 21 16 21" /> < line x1 = "15" y1 = "15" x2 =
                "21" y2 = "21" /> < line x1 = "4" y1 = "4" x2 = "9" y2 = "9" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiSidebar")]
        FiIcon::FiSidebar => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiSidebar".to_owned()) } < / title > < rect x = "3" y = "3" width = "18"
                height = "18" rx = "2" ry = "2" /> < line x1 = "9" y1 = "3" x2 = "9" y2 =
                "21" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiSkipBack")]
        FiIcon::FiSkipBack => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiSkipBack".to_owned()) } < / title > < polygon points =
                "19 20 9 12 19 4 19 20" /> < line x1 = "5" y1 = "19" x2 = "5" y2 = "5" />
                < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiSkipForward")]
        FiIcon::FiSkipForward => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiSkipForward".to_owned()) } < / title > < polygon points =
                "5 4 15 12 5 20 5 4" /> < line x1 = "19" y1 = "5" x2 = "19" y2 = "19" />
                < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiSlack")]
        FiIcon::FiSlack => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiSlack".to_owned()) } < / title > < path d =
                "M14.5 10c-.83 0-1.5-.67-1.5-1.5v-5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5v5c0 .83-.67 1.5-1.5 1.5z"
                /> < path d =
                "M20.5 10H19V8.5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z" />
                < path d =
                "M9.5 14c.83 0 1.5.67 1.5 1.5v5c0 .83-.67 1.5-1.5 1.5S8 21.33 8 20.5v-5c0-.83.67-1.5 1.5-1.5z"
                /> < path d =
                "M3.5 14H5v1.5c0 .83-.67 1.5-1.5 1.5S2 16.33 2 15.5 2.67 14 3.5 14z" /> <
                path d =
                "M14 14.5c0-.83.67-1.5 1.5-1.5h5c.83 0 1.5.67 1.5 1.5s-.67 1.5-1.5 1.5h-5c-.83 0-1.5-.67-1.5-1.5z"
                /> < path d =
                "M15.5 19H14v1.5c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5-.67-1.5-1.5-1.5z"
                /> < path d =
                "M10 9.5C10 8.67 9.33 8 8.5 8h-5C2.67 8 2 8.67 2 9.5S2.67 11 3.5 11h5c.83 0 1.5-.67 1.5-1.5z"
                /> < path d =
                "M8.5 5H10V3.5C10 2.67 9.33 2 8.5 2S7 2.67 7 3.5 7.67 5 8.5 5z" /> < /
                svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiSlash")]
        FiIcon::FiSlash => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiSlash".to_owned()) } < / title > < circle cx = "12" cy = "12" r = "10"
                /> < line x1 = "4.93" y1 = "4.93" x2 = "19.07" y2 = "19.07" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiSliders")]
        FiIcon::FiSliders => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiSliders".to_owned()) } < / title > < line x1 = "4" y1 = "21" x2 = "4"
                y2 = "14" /> < line x1 = "4" y1 = "10" x2 = "4" y2 = "3" /> < line x1 =
                "12" y1 = "21" x2 = "12" y2 = "12" /> < line x1 = "12" y1 = "8" x2 = "12"
                y2 = "3" /> < line x1 = "20" y1 = "21" x2 = "20" y2 = "16" /> < line x1 =
                "20" y1 = "12" x2 = "20" y2 = "3" /> < line x1 = "1" y1 = "14" x2 = "7"
                y2 = "14" /> < line x1 = "9" y1 = "8" x2 = "15" y2 = "8" /> < line x1 =
                "17" y1 = "16" x2 = "23" y2 = "16" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiSmartphone")]
        FiIcon::FiSmartphone => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiSmartphone".to_owned()) } < / title > < rect x = "5" y = "2" width =
                "14" height = "20" rx = "2" ry = "2" /> < line x1 = "12" y1 = "18" x2 =
                "12.01" y2 = "18" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiSmile")]
        FiIcon::FiSmile => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiSmile".to_owned()) } < / title > < circle cx = "12" cy = "12" r = "10"
                /> < path d = "M8 14s1.5 2 4 2 4-2 4-2" /> < line x1 = "9" y1 = "9" x2 =
                "9.01" y2 = "9" /> < line x1 = "15" y1 = "9" x2 = "15.01" y2 = "9" /> < /
                svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiSpeaker")]
        FiIcon::FiSpeaker => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiSpeaker".to_owned()) } < / title > < rect x = "4" y = "2" width = "16"
                height = "20" rx = "2" ry = "2" /> < circle cx = "12" cy = "14" r = "4"
                /> < line x1 = "12" y1 = "6" x2 = "12.01" y2 = "6" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiSquare")]
        FiIcon::FiSquare => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiSquare".to_owned()) } < / title > < rect x = "3" y = "3" width = "18"
                height = "18" rx = "2" ry = "2" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiStar")]
        FiIcon::FiStar => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiStar".to_owned()) } < / title > < polygon points =
                "12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiStopCircle")]
        FiIcon::FiStopCircle => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiStopCircle".to_owned()) } < / title > < circle cx = "12" cy = "12" r =
                "10" /> < rect x = "9" y = "9" width = "6" height = "6" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiSun")]
        FiIcon::FiSun => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiSun".to_owned()) } < / title > < circle cx = "12" cy = "12" r = "5" />
                < line x1 = "12" y1 = "1" x2 = "12" y2 = "3" /> < line x1 = "12" y1 =
                "21" x2 = "12" y2 = "23" /> < line x1 = "4.22" y1 = "4.22" x2 = "5.64" y2
                = "5.64" /> < line x1 = "18.36" y1 = "18.36" x2 = "19.78" y2 = "19.78" />
                < line x1 = "1" y1 = "12" x2 = "3" y2 = "12" /> < line x1 = "21" y1 =
                "12" x2 = "23" y2 = "12" /> < line x1 = "4.22" y1 = "19.78" x2 = "5.64"
                y2 = "18.36" /> < line x1 = "18.36" y1 = "5.64" x2 = "19.78" y2 = "4.22"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiSunrise")]
        FiIcon::FiSunrise => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiSunrise".to_owned()) } < / title > < path d = "M17 18a5 5 0 0 0-10 0"
                /> < line x1 = "12" y1 = "2" x2 = "12" y2 = "9" /> < line x1 = "4.22" y1
                = "10.22" x2 = "5.64" y2 = "11.64" /> < line x1 = "1" y1 = "18" x2 = "3"
                y2 = "18" /> < line x1 = "21" y1 = "18" x2 = "23" y2 = "18" /> < line x1
                = "18.36" y1 = "11.64" x2 = "19.78" y2 = "10.22" /> < line x1 = "23" y1 =
                "22" x2 = "1" y2 = "22" /> < polyline points = "8 6 12 2 16 6" /> < / svg
                >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiSunset")]
        FiIcon::FiSunset => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiSunset".to_owned()) } < / title > < path d = "M17 18a5 5 0 0 0-10 0"
                /> < line x1 = "12" y1 = "9" x2 = "12" y2 = "2" /> < line x1 = "4.22" y1
                = "10.22" x2 = "5.64" y2 = "11.64" /> < line x1 = "1" y1 = "18" x2 = "3"
                y2 = "18" /> < line x1 = "21" y1 = "18" x2 = "23" y2 = "18" /> < line x1
                = "18.36" y1 = "11.64" x2 = "19.78" y2 = "10.22" /> < line x1 = "23" y1 =
                "22" x2 = "1" y2 = "22" /> < polyline points = "16 5 12 9 8 5" /> < / svg
                >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiTable")]
        FiIcon::FiTable => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiTable".to_owned()) } < / title > < path d =
                "M9 3H5a2 2 0 0 0-2 2v4m6-6h10a2 2 0 0 1 2 2v4M9 3v18m0 0h10a2 2 0 0 0 2-2V9M9 21H5a2 2 0 0 1-2-2V9m0 0h18"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiTablet")]
        FiIcon::FiTablet => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiTablet".to_owned()) } < / title > < rect x = "4" y = "2" width = "16"
                height = "20" rx = "2" ry = "2" /> < line x1 = "12" y1 = "18" x2 =
                "12.01" y2 = "18" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiTag")]
        FiIcon::FiTag => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiTag".to_owned()) } < / title > < path d =
                "M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z"
                /> < line x1 = "7" y1 = "7" x2 = "7.01" y2 = "7" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiTarget")]
        FiIcon::FiTarget => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiTarget".to_owned()) } < / title > < circle cx = "12" cy = "12" r =
                "10" /> < circle cx = "12" cy = "12" r = "6" /> < circle cx = "12" cy =
                "12" r = "2" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiTerminal")]
        FiIcon::FiTerminal => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiTerminal".to_owned()) } < / title > < polyline points =
                "4 17 10 11 4 5" /> < line x1 = "12" y1 = "19" x2 = "20" y2 = "19" /> < /
                svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiThermometer")]
        FiIcon::FiThermometer => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiThermometer".to_owned()) } < / title > < path d =
                "M14 14.76V3.5a2.5 2.5 0 0 0-5 0v11.26a4.5 4.5 0 1 0 5 0z" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiThumbsDown")]
        FiIcon::FiThumbsDown => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiThumbsDown".to_owned()) } < / title > < path d =
                "M10 15v4a3 3 0 0 0 3 3l4-9V2H5.72a2 2 0 0 0-2 1.7l-1.38 9a2 2 0 0 0 2 2.3zm7-13h2.67A2.31 2.31 0 0 1 22 4v7a2.31 2.31 0 0 1-2.33 2H17"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiThumbsUp")]
        FiIcon::FiThumbsUp => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiThumbsUp".to_owned()) } < / title > < path d =
                "M14 9V5a3 3 0 0 0-3-3l-4 9v11h11.28a2 2 0 0 0 2-1.7l1.38-9a2 2 0 0 0-2-2.3zM7 22H4a2 2 0 0 1-2-2v-7a2 2 0 0 1 2-2h3"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiToggleLeft")]
        FiIcon::FiToggleLeft => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiToggleLeft".to_owned()) } < / title > < rect x = "1" y = "5" width =
                "22" height = "14" rx = "7" ry = "7" /> < circle cx = "8" cy = "12" r =
                "3" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiToggleRight")]
        FiIcon::FiToggleRight => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiToggleRight".to_owned()) } < / title > < rect x = "1" y = "5" width =
                "22" height = "14" rx = "7" ry = "7" /> < circle cx = "16" cy = "12" r =
                "3" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiTool")]
        FiIcon::FiTool => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiTool".to_owned()) } < / title > < path d =
                "M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiTrash")]
        FiIcon::FiTrash => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiTrash".to_owned()) } < / title > < polyline points = "3 6 5 6 21 6" />
                < path d =
                "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiTrash2")]
        FiIcon::FiTrash2 => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiTrash2".to_owned()) } < / title > < polyline points = "3 6 5 6 21 6"
                /> < path d =
                "M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2"
                /> < line x1 = "10" y1 = "11" x2 = "10" y2 = "17" /> < line x1 = "14" y1
                = "11" x2 = "14" y2 = "17" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiTrello")]
        FiIcon::FiTrello => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiTrello".to_owned()) } < / title > < rect x = "3" y = "3" width = "18"
                height = "18" rx = "2" ry = "2" /> < rect x = "7" y = "7" width = "3"
                height = "9" /> < rect x = "14" y = "7" width = "3" height = "5" /> < /
                svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiTrendingDown")]
        FiIcon::FiTrendingDown => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiTrendingDown".to_owned()) } < / title > < polyline points =
                "23 18 13.5 8.5 8.5 13.5 1 6" /> < polyline points = "17 18 23 18 23 12"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiTrendingUp")]
        FiIcon::FiTrendingUp => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiTrendingUp".to_owned()) } < / title > < polyline points =
                "23 6 13.5 15.5 8.5 10.5 1 18" /> < polyline points = "17 6 23 6 23 12"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiTriangle")]
        FiIcon::FiTriangle => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiTriangle".to_owned()) } < / title > < path d =
                "M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiTruck")]
        FiIcon::FiTruck => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiTruck".to_owned()) } < / title > < rect x = "1" y = "3" width = "15"
                height = "13" /> < polygon points = "16 8 20 8 23 11 23 16 16 16 16 8" />
                < circle cx = "5.5" cy = "18.5" r = "2.5" /> < circle cx = "18.5" cy =
                "18.5" r = "2.5" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiTv")]
        FiIcon::FiTv => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiTv".to_owned()) } < / title > < rect x = "2" y = "7" width = "20"
                height = "15" rx = "2" ry = "2" /> < polyline points = "17 2 12 7 7 2" />
                < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiTwitch")]
        FiIcon::FiTwitch => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiTwitch".to_owned()) } < / title > < path d =
                "M21 2H3v16h5v4l4-4h5l4-4V2zM11 11V7M16 11V7" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiTwitter")]
        FiIcon::FiTwitter => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiTwitter".to_owned()) } < / title > < path d =
                "M23 3a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-7.86 3v1A10.66 10.66 0 0 1 3 4s-4 9 5 13a11.64 11.64 0 0 1-7 2c9 5 20 0 20-11.5a4.5 4.5 0 0 0-.08-.83A7.72 7.72 0 0 0 23 3z"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiType")]
        FiIcon::FiType => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiType".to_owned()) } < / title > < polyline points =
                "4 7 4 4 20 4 20 7" /> < line x1 = "9" y1 = "20" x2 = "15" y2 = "20" /> <
                line x1 = "12" y1 = "4" x2 = "12" y2 = "20" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiUmbrella")]
        FiIcon::FiUmbrella => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiUmbrella".to_owned()) } < / title > < path d =
                "M23 12a11.05 11.05 0 0 0-22 0zm-5 7a3 3 0 0 1-6 0v-7" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiUnderline")]
        FiIcon::FiUnderline => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiUnderline".to_owned()) } < / title > < path d =
                "M6 3v7a6 6 0 0 0 6 6 6 6 0 0 0 6-6V3" /> < line x1 = "4" y1 = "21" x2 =
                "20" y2 = "21" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiUnlock")]
        FiIcon::FiUnlock => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiUnlock".to_owned()) } < / title > < rect x = "3" y = "11" width = "18"
                height = "11" rx = "2" ry = "2" /> < path d = "M7 11V7a5 5 0 0 1 9.9-1"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiUpload")]
        FiIcon::FiUpload => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiUpload".to_owned()) } < / title > < path d =
                "M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" /> < polyline points =
                "17 8 12 3 7 8" /> < line x1 = "12" y1 = "3" x2 = "12" y2 = "15" /> < /
                svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiUploadCloud")]
        FiIcon::FiUploadCloud => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiUploadCloud".to_owned()) } < / title > < polyline points =
                "16 16 12 12 8 16" /> < line x1 = "12" y1 = "12" x2 = "12" y2 = "21" /> <
                path d = "M20.39 18.39A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.3" /> <
                polyline points = "16 16 12 12 8 16" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiUser")]
        FiIcon::FiUser => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiUser".to_owned()) } < / title > < path d =
                "M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2" /> < circle cx = "12" cy =
                "7" r = "4" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiUserCheck")]
        FiIcon::FiUserCheck => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiUserCheck".to_owned()) } < / title > < path d =
                "M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" /> < circle cx = "8.5" cy =
                "7" r = "4" /> < polyline points = "17 11 19 13 23 9" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiUserMinus")]
        FiIcon::FiUserMinus => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiUserMinus".to_owned()) } < / title > < path d =
                "M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" /> < circle cx = "8.5" cy =
                "7" r = "4" /> < line x1 = "23" y1 = "11" x2 = "17" y2 = "11" /> < / svg
                >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiUserPlus")]
        FiIcon::FiUserPlus => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiUserPlus".to_owned()) } < / title > < path d =
                "M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" /> < circle cx = "8.5" cy =
                "7" r = "4" /> < line x1 = "20" y1 = "8" x2 = "20" y2 = "14" /> < line x1
                = "23" y1 = "11" x2 = "17" y2 = "11" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiUserX")]
        FiIcon::FiUserX => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiUserX".to_owned()) } < / title > < path d =
                "M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" /> < circle cx = "8.5" cy =
                "7" r = "4" /> < line x1 = "18" y1 = "8" x2 = "23" y2 = "13" /> < line x1
                = "23" y1 = "8" x2 = "18" y2 = "13" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiUsers")]
        FiIcon::FiUsers => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiUsers".to_owned()) } < / title > < path d =
                "M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2" /> < circle cx = "9" cy = "7"
                r = "4" /> < path d = "M23 21v-2a4 4 0 0 0-3-3.87" /> < path d =
                "M16 3.13a4 4 0 0 1 0 7.75" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiVideo")]
        FiIcon::FiVideo => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiVideo".to_owned()) } < / title > < polygon points =
                "23 7 16 12 23 17 23 7" /> < rect x = "1" y = "5" width = "15" height =
                "14" rx = "2" ry = "2" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiVideoOff")]
        FiIcon::FiVideoOff => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiVideoOff".to_owned()) } < / title > < path d =
                "M16 16v1a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h2m5.66 0H14a2 2 0 0 1 2 2v3.34l1 1L23 7v10"
                /> < line x1 = "1" y1 = "1" x2 = "23" y2 = "23" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiVoicemail")]
        FiIcon::FiVoicemail => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiVoicemail".to_owned()) } < / title > < circle cx = "5.5" cy = "11.5" r
                = "4.5" /> < circle cx = "18.5" cy = "11.5" r = "4.5" /> < line x1 =
                "5.5" y1 = "16" x2 = "18.5" y2 = "16" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiVolume")]
        FiIcon::FiVolume => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiVolume".to_owned()) } < / title > < polygon points =
                "11 5 6 9 2 9 2 15 6 15 11 19 11 5" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiVolume1")]
        FiIcon::FiVolume1 => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiVolume1".to_owned()) } < / title > < polygon points =
                "11 5 6 9 2 9 2 15 6 15 11 19 11 5" /> < path d =
                "M15.54 8.46a5 5 0 0 1 0 7.07" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiVolume2")]
        FiIcon::FiVolume2 => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiVolume2".to_owned()) } < / title > < polygon points =
                "11 5 6 9 2 9 2 15 6 15 11 19 11 5" /> < path d =
                "M19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07" /> < / svg
                >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiVolumeX")]
        FiIcon::FiVolumeX => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiVolumeX".to_owned()) } < / title > < polygon points =
                "11 5 6 9 2 9 2 15 6 15 11 19 11 5" /> < line x1 = "23" y1 = "9" x2 =
                "17" y2 = "15" /> < line x1 = "17" y1 = "9" x2 = "23" y2 = "15" /> < /
                svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiWatch")]
        FiIcon::FiWatch => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiWatch".to_owned()) } < / title > < circle cx = "12" cy = "12" r = "7"
                /> < polyline points = "12 9 12 12 13.5 13.5" /> < path d =
                "M16.51 17.35l-.35 3.83a2 2 0 0 1-2 1.82H9.83a2 2 0 0 1-2-1.82l-.35-3.83m.01-10.7l.35-3.83A2 2 0 0 1 9.83 1h4.35a2 2 0 0 1 2 1.82l.35 3.83"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiWifi")]
        FiIcon::FiWifi => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiWifi".to_owned()) } < / title > < path d =
                "M5 12.55a11 11 0 0 1 14.08 0" /> < path d =
                "M1.42 9a16 16 0 0 1 21.16 0" /> < path d =
                "M8.53 16.11a6 6 0 0 1 6.95 0" /> < line x1 = "12" y1 = "20" x2 = "12.01"
                y2 = "20" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiWifiOff")]
        FiIcon::FiWifiOff => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiWifiOff".to_owned()) } < / title > < line x1 = "1" y1 = "1" x2 = "23"
                y2 = "23" /> < path d = "M16.72 11.06A10.94 10.94 0 0 1 19 12.55" /> <
                path d = "M5 12.55a10.94 10.94 0 0 1 5.17-2.39" /> < path d =
                "M10.71 5.05A16 16 0 0 1 22.58 9" /> < path d =
                "M1.42 9a15.91 15.91 0 0 1 4.7-2.88" /> < path d =
                "M8.53 16.11a6 6 0 0 1 6.95 0" /> < line x1 = "12" y1 = "20" x2 = "12.01"
                y2 = "20" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiWind")]
        FiIcon::FiWind => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiWind".to_owned()) } < / title > < path d =
                "M9.59 4.59A2 2 0 1 1 11 8H2m10.59 11.41A2 2 0 1 0 14 16H2m15.73-8.27A2.5 2.5 0 1 1 19.5 12H2"
                /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiX")]
        FiIcon::FiX => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiX".to_owned()) } < / title > < line x1 = "18" y1 = "6" x2 = "6" y2 =
                "18" /> < line x1 = "6" y1 = "6" x2 = "18" y2 = "18" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiXCircle")]
        FiIcon::FiXCircle => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiXCircle".to_owned()) } < / title > < circle cx = "12" cy = "12" r =
                "10" /> < line x1 = "15" y1 = "9" x2 = "9" y2 = "15" /> < line x1 = "9"
                y1 = "9" x2 = "15" y2 = "15" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiXOctagon")]
        FiIcon::FiXOctagon => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiXOctagon".to_owned()) } < / title > < polygon points =
                "7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2"
                /> < line x1 = "15" y1 = "9" x2 = "9" y2 = "15" /> < line x1 = "9" y1 =
                "9" x2 = "15" y2 = "15" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiXSquare")]
        FiIcon::FiXSquare => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiXSquare".to_owned()) } < / title > < rect x = "3" y = "3" width = "18"
                height = "18" rx = "2" ry = "2" /> < line x1 = "9" y1 = "9" x2 = "15" y2
                = "15" /> < line x1 = "15" y1 = "9" x2 = "9" y2 = "15" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiYoutube")]
        FiIcon::FiYoutube => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiYoutube".to_owned()) } < / title > < path d =
                "M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z"
                /> < polygon points = "9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02" /> < /
                svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiZap")]
        FiIcon::FiZap => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiZap".to_owned()) } < / title > < polygon points =
                "13 2 3 14 12 14 11 22 21 10 12 10 13 2" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiZapOff")]
        FiIcon::FiZapOff => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiZapOff".to_owned()) } < / title > < polyline points =
                "12.41 6.75 13 2 10.57 4.92" /> < polyline points =
                "18.57 12.91 21 10 15.66 10" /> < polyline points =
                "8 8 3 14 12 14 11 22 16 16" /> < line x1 = "1" y1 = "1" x2 = "23" y2 =
                "23" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiZoomIn")]
        FiIcon::FiZoomIn => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiZoomIn".to_owned()) } < / title > < circle cx = "11" cy = "11" r = "8"
                /> < line x1 = "21" y1 = "21" x2 = "16.65" y2 = "16.65" /> < line x1 =
                "11" y1 = "8" x2 = "11" y2 = "14" /> < line x1 = "8" y1 = "11" x2 = "14"
                y2 = "11" /> < / svg >
            }
                .into_view(cx)
        }
        #[cfg(feature = "FiZoomOut")]
        FiIcon::FiZoomOut => {
            view! {
                cx, < svg class = class style = format!(" {}", style) width = width
                height = height viewBox = "0 0 24 24" stroke - linecap = "round" stroke -
                linejoin = "round" stroke - width = "2" stroke = "currentColor" fill =
                "none" role = "graphics-symbol" > < title > { title.unwrap_or_else(||
                "FiZoomOut".to_owned()) } < / title > < circle cx = "11" cy = "11" r =
                "8" /> < line x1 = "21" y1 = "21" x2 = "16.65" y2 = "16.65" /> < line x1
                = "8" y1 = "11" x2 = "14" y2 = "11" /> < / svg >
            }
                .into_view(cx)
        }
    }
}
