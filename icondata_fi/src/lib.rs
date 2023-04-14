
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

#[cfg(feature = "FiActivity")]
const FI_ACTIVITY: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"22 12 18 12 15 21 9 3 6 12 2 12\" />",
};
#[cfg(feature = "FiAirplay")]
const FI_AIRPLAY: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M5 17H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-1\" />\n<polygon points=\"12 15 17 21 7 21 12 15\" />",
};
#[cfg(feature = "FiAlertCircle")]
const FI_ALERT_CIRCLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<line x1=\"12\" y1=\"8\" x2=\"12\" y2=\"12\" />\n<line x1=\"12\" y1=\"16\" x2=\"12.01\" y2=\"16\" />",
};
#[cfg(feature = "FiAlertOctagon")]
const FI_ALERT_OCTAGON: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2\" />\n<line x1=\"12\" y1=\"8\" x2=\"12\" y2=\"12\" />\n<line x1=\"12\" y1=\"16\" x2=\"12.01\" y2=\"16\" />",
};
#[cfg(feature = "FiAlertTriangle")]
const FI_ALERT_TRIANGLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z\" />\n<line x1=\"12\" y1=\"9\" x2=\"12\" y2=\"13\" />\n<line x1=\"12\" y1=\"17\" x2=\"12.01\" y2=\"17\" />",
};
#[cfg(feature = "FiAlignCenter")]
const FI_ALIGN_CENTER: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"18\" y1=\"10\" x2=\"6\" y2=\"10\" />\n<line x1=\"21\" y1=\"6\" x2=\"3\" y2=\"6\" />\n<line x1=\"21\" y1=\"14\" x2=\"3\" y2=\"14\" />\n<line x1=\"18\" y1=\"18\" x2=\"6\" y2=\"18\" />",
};
#[cfg(feature = "FiAlignJustify")]
const FI_ALIGN_JUSTIFY: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"21\" y1=\"10\" x2=\"3\" y2=\"10\" />\n<line x1=\"21\" y1=\"6\" x2=\"3\" y2=\"6\" />\n<line x1=\"21\" y1=\"14\" x2=\"3\" y2=\"14\" />\n<line x1=\"21\" y1=\"18\" x2=\"3\" y2=\"18\" />",
};
#[cfg(feature = "FiAlignLeft")]
const FI_ALIGN_LEFT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"17\" y1=\"10\" x2=\"3\" y2=\"10\" />\n<line x1=\"21\" y1=\"6\" x2=\"3\" y2=\"6\" />\n<line x1=\"21\" y1=\"14\" x2=\"3\" y2=\"14\" />\n<line x1=\"17\" y1=\"18\" x2=\"3\" y2=\"18\" />",
};
#[cfg(feature = "FiAlignRight")]
const FI_ALIGN_RIGHT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"21\" y1=\"10\" x2=\"7\" y2=\"10\" />\n<line x1=\"21\" y1=\"6\" x2=\"3\" y2=\"6\" />\n<line x1=\"21\" y1=\"14\" x2=\"3\" y2=\"14\" />\n<line x1=\"21\" y1=\"18\" x2=\"7\" y2=\"18\" />",
};
#[cfg(feature = "FiAnchor")]
const FI_ANCHOR: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"5\" r=\"3\" />\n<line x1=\"12\" y1=\"22\" x2=\"12\" y2=\"8\" />\n<path d=\"M5 12H2a10 10 0 0 0 20 0h-3\" />",
};
#[cfg(feature = "FiAperture")]
const FI_APERTURE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<line x1=\"14.31\" y1=\"8\" x2=\"20.05\" y2=\"17.94\" />\n<line x1=\"9.69\" y1=\"8\" x2=\"21.17\" y2=\"8\" />\n<line x1=\"7.38\" y1=\"12\" x2=\"13.12\" y2=\"2.06\" />\n<line x1=\"9.69\" y1=\"16\" x2=\"3.95\" y2=\"6.06\" />\n<line x1=\"14.31\" y1=\"16\" x2=\"2.83\" y2=\"16\" />\n<line x1=\"16.62\" y1=\"12\" x2=\"10.88\" y2=\"21.94\" />",
};
#[cfg(feature = "FiArchive")]
const FI_ARCHIVE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"21 8 21 21 3 21 3 8\" />\n<rect x=\"1\" y=\"3\" width=\"22\" height=\"5\" />\n<line x1=\"10\" y1=\"12\" x2=\"14\" y2=\"12\" />",
};
#[cfg(feature = "FiArrowDown")]
const FI_ARROW_DOWN: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"12\" y1=\"5\" x2=\"12\" y2=\"19\" />\n<polyline points=\"19 12 12 19 5 12\" />",
};
#[cfg(feature = "FiArrowDownCircle")]
const FI_ARROW_DOWN_CIRCLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<polyline points=\"8 12 12 16 16 12\" />\n<line x1=\"12\" y1=\"8\" x2=\"12\" y2=\"16\" />",
};
#[cfg(feature = "FiArrowDownLeft")]
const FI_ARROW_DOWN_LEFT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"17\" y1=\"7\" x2=\"7\" y2=\"17\" />\n<polyline points=\"17 17 7 17 7 7\" />",
};
#[cfg(feature = "FiArrowDownRight")]
const FI_ARROW_DOWN_RIGHT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"7\" y1=\"7\" x2=\"17\" y2=\"17\" />\n<polyline points=\"17 7 17 17 7 17\" />",
};
#[cfg(feature = "FiArrowLeft")]
const FI_ARROW_LEFT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"19\" y1=\"12\" x2=\"5\" y2=\"12\" />\n<polyline points=\"12 19 5 12 12 5\" />",
};
#[cfg(feature = "FiArrowLeftCircle")]
const FI_ARROW_LEFT_CIRCLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<polyline points=\"12 8 8 12 12 16\" />\n<line x1=\"16\" y1=\"12\" x2=\"8\" y2=\"12\" />",
};
#[cfg(feature = "FiArrowRight")]
const FI_ARROW_RIGHT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"5\" y1=\"12\" x2=\"19\" y2=\"12\" />\n<polyline points=\"12 5 19 12 12 19\" />",
};
#[cfg(feature = "FiArrowRightCircle")]
const FI_ARROW_RIGHT_CIRCLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<polyline points=\"12 16 16 12 12 8\" />\n<line x1=\"8\" y1=\"12\" x2=\"16\" y2=\"12\" />",
};
#[cfg(feature = "FiArrowUp")]
const FI_ARROW_UP: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"12\" y1=\"19\" x2=\"12\" y2=\"5\" />\n<polyline points=\"5 12 12 5 19 12\" />",
};
#[cfg(feature = "FiArrowUpCircle")]
const FI_ARROW_UP_CIRCLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<polyline points=\"16 12 12 8 8 12\" />\n<line x1=\"12\" y1=\"16\" x2=\"12\" y2=\"8\" />",
};
#[cfg(feature = "FiArrowUpLeft")]
const FI_ARROW_UP_LEFT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"17\" y1=\"17\" x2=\"7\" y2=\"7\" />\n<polyline points=\"7 17 7 7 17 7\" />",
};
#[cfg(feature = "FiArrowUpRight")]
const FI_ARROW_UP_RIGHT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"7\" y1=\"17\" x2=\"17\" y2=\"7\" />\n<polyline points=\"7 7 17 7 17 17\" />",
};
#[cfg(feature = "FiAtSign")]
const FI_AT_SIGN: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"4\" />\n<path d=\"M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-3.92 7.94\" />",
};
#[cfg(feature = "FiAward")]
const FI_AWARD: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"8\" r=\"7\" />\n<polyline points=\"8.21 13.89 7 23 12 20 17 23 15.79 13.88\" />",
};
#[cfg(feature = "FiBarChart")]
const FI_BAR_CHART: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"12\" y1=\"20\" x2=\"12\" y2=\"10\" />\n<line x1=\"18\" y1=\"20\" x2=\"18\" y2=\"4\" />\n<line x1=\"6\" y1=\"20\" x2=\"6\" y2=\"16\" />",
};
#[cfg(feature = "FiBarChart2")]
const FI_BAR_CHART2: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"18\" y1=\"20\" x2=\"18\" y2=\"10\" />\n<line x1=\"12\" y1=\"20\" x2=\"12\" y2=\"4\" />\n<line x1=\"6\" y1=\"20\" x2=\"6\" y2=\"14\" />",
};
#[cfg(feature = "FiBattery")]
const FI_BATTERY: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"1\" y=\"6\" width=\"18\" height=\"12\" rx=\"2\" ry=\"2\" />\n<line x1=\"23\" y1=\"13\" x2=\"23\" y2=\"11\" />",
};
#[cfg(feature = "FiBatteryCharging")]
const FI_BATTERY_CHARGING: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M5 18H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3.19M15 6h2a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-3.19\" />\n<line x1=\"23\" y1=\"13\" x2=\"23\" y2=\"11\" />\n<polyline points=\"11 6 7 12 13 12 9 18\" />",
};
#[cfg(feature = "FiBell")]
const FI_BELL: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9\" />\n<path d=\"M13.73 21a2 2 0 0 1-3.46 0\" />",
};
#[cfg(feature = "FiBellOff")]
const FI_BELL_OFF: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M13.73 21a2 2 0 0 1-3.46 0\" />\n<path d=\"M18.63 13A17.89 17.89 0 0 1 18 8\" />\n<path d=\"M6.26 6.26A5.86 5.86 0 0 0 6 8c0 7-3 9-3 9h14\" />\n<path d=\"M18 8a6 6 0 0 0-9.33-5\" />\n<line x1=\"1\" y1=\"1\" x2=\"23\" y2=\"23\" />",
};
#[cfg(feature = "FiBluetooth")]
const FI_BLUETOOTH: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"6.5 6.5 17.5 17.5 12 23 12 1 17.5 6.5 6.5 17.5\" />",
};
#[cfg(feature = "FiBold")]
const FI_BOLD: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z\" />\n<path d=\"M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z\" />",
};
#[cfg(feature = "FiBook")]
const FI_BOOK: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M4 19.5A2.5 2.5 0 0 1 6.5 17H20\" />\n<path d=\"M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z\" />",
};
#[cfg(feature = "FiBookOpen")]
const FI_BOOK_OPEN: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z\" />\n<path d=\"M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z\" />",
};
#[cfg(feature = "FiBookmark")]
const FI_BOOKMARK: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z\" />",
};
#[cfg(feature = "FiBox")]
const FI_BOX: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z\" />\n<polyline points=\"3.27 6.96 12 12.01 20.73 6.96\" />\n<line x1=\"12\" y1=\"22.08\" x2=\"12\" y2=\"12\" />",
};
#[cfg(feature = "FiBriefcase")]
const FI_BRIEFCASE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"2\" y=\"7\" width=\"20\" height=\"14\" rx=\"2\" ry=\"2\" />\n<path d=\"M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16\" />",
};
#[cfg(feature = "FiCalendar")]
const FI_CALENDAR: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"3\" y=\"4\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" />\n<line x1=\"16\" y1=\"2\" x2=\"16\" y2=\"6\" />\n<line x1=\"8\" y1=\"2\" x2=\"8\" y2=\"6\" />\n<line x1=\"3\" y1=\"10\" x2=\"21\" y2=\"10\" />",
};
#[cfg(feature = "FiCamera")]
const FI_CAMERA: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z\" />\n<circle cx=\"12\" cy=\"13\" r=\"4\" />",
};
#[cfg(feature = "FiCameraOff")]
const FI_CAMERA_OFF: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"1\" y1=\"1\" x2=\"23\" y2=\"23\" />\n<path d=\"M21 21H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3m3-3h6l2 3h4a2 2 0 0 1 2 2v9.34m-7.72-2.06a4 4 0 1 1-5.56-5.56\" />",
};
#[cfg(feature = "FiCast")]
const FI_CAST: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M2 16.1A5 5 0 0 1 5.9 20M2 12.05A9 9 0 0 1 9.95 20M2 8V6a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2h-6\" />\n<line x1=\"2\" y1=\"20\" x2=\"2.01\" y2=\"20\" />",
};
#[cfg(feature = "FiCheck")]
const FI_CHECK: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"20 6 9 17 4 12\" />",
};
#[cfg(feature = "FiCheckCircle")]
const FI_CHECK_CIRCLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M22 11.08V12a10 10 0 1 1-5.93-9.14\" />\n<polyline points=\"22 4 12 14.01 9 11.01\" />",
};
#[cfg(feature = "FiCheckSquare")]
const FI_CHECK_SQUARE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"9 11 12 14 22 4\" />\n<path d=\"M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11\" />",
};
#[cfg(feature = "FiChevronDown")]
const FI_CHEVRON_DOWN: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"6 9 12 15 18 9\" />",
};
#[cfg(feature = "FiChevronLeft")]
const FI_CHEVRON_LEFT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"15 18 9 12 15 6\" />",
};
#[cfg(feature = "FiChevronRight")]
const FI_CHEVRON_RIGHT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"9 18 15 12 9 6\" />",
};
#[cfg(feature = "FiChevronUp")]
const FI_CHEVRON_UP: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"18 15 12 9 6 15\" />",
};
#[cfg(feature = "FiChevronsDown")]
const FI_CHEVRONS_DOWN: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"7 13 12 18 17 13\" />\n<polyline points=\"7 6 12 11 17 6\" />",
};
#[cfg(feature = "FiChevronsLeft")]
const FI_CHEVRONS_LEFT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"11 17 6 12 11 7\" />\n<polyline points=\"18 17 13 12 18 7\" />",
};
#[cfg(feature = "FiChevronsRight")]
const FI_CHEVRONS_RIGHT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"13 17 18 12 13 7\" />\n<polyline points=\"6 17 11 12 6 7\" />",
};
#[cfg(feature = "FiChevronsUp")]
const FI_CHEVRONS_UP: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"17 11 12 6 7 11\" />\n<polyline points=\"17 18 12 13 7 18\" />",
};
#[cfg(feature = "FiChrome")]
const FI_CHROME: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<circle cx=\"12\" cy=\"12\" r=\"4\" />\n<line x1=\"21.17\" y1=\"8\" x2=\"12\" y2=\"8\" />\n<line x1=\"3.95\" y1=\"6.06\" x2=\"8.54\" y2=\"14\" />\n<line x1=\"10.88\" y1=\"21.94\" x2=\"15.46\" y2=\"14\" />",
};
#[cfg(feature = "FiCircle")]
const FI_CIRCLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />",
};
#[cfg(feature = "FiClipboard")]
const FI_CLIPBOARD: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2\" />\n<rect x=\"8\" y=\"2\" width=\"8\" height=\"4\" rx=\"1\" ry=\"1\" />",
};
#[cfg(feature = "FiClock")]
const FI_CLOCK: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<polyline points=\"12 6 12 12 16 14\" />",
};
#[cfg(feature = "FiCloud")]
const FI_CLOUD: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M18 10h-1.26A8 8 0 1 0 9 20h9a5 5 0 0 0 0-10z\" />",
};
#[cfg(feature = "FiCloudDrizzle")]
const FI_CLOUD_DRIZZLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"8\" y1=\"19\" x2=\"8\" y2=\"21\" />\n<line x1=\"8\" y1=\"13\" x2=\"8\" y2=\"15\" />\n<line x1=\"16\" y1=\"19\" x2=\"16\" y2=\"21\" />\n<line x1=\"16\" y1=\"13\" x2=\"16\" y2=\"15\" />\n<line x1=\"12\" y1=\"21\" x2=\"12\" y2=\"23\" />\n<line x1=\"12\" y1=\"15\" x2=\"12\" y2=\"17\" />\n<path d=\"M20 16.58A5 5 0 0 0 18 7h-1.26A8 8 0 1 0 4 15.25\" />",
};
#[cfg(feature = "FiCloudLightning")]
const FI_CLOUD_LIGHTNING: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M19 16.9A5 5 0 0 0 18 7h-1.26a8 8 0 1 0-11.62 9\" />\n<polyline points=\"13 11 9 17 15 17 11 23\" />",
};
#[cfg(feature = "FiCloudOff")]
const FI_CLOUD_OFF: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M22.61 16.95A5 5 0 0 0 18 10h-1.26a8 8 0 0 0-7.05-6M5 5a8 8 0 0 0 4 15h9a5 5 0 0 0 1.7-.3\" />\n<line x1=\"1\" y1=\"1\" x2=\"23\" y2=\"23\" />",
};
#[cfg(feature = "FiCloudRain")]
const FI_CLOUD_RAIN: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"16\" y1=\"13\" x2=\"16\" y2=\"21\" />\n<line x1=\"8\" y1=\"13\" x2=\"8\" y2=\"21\" />\n<line x1=\"12\" y1=\"15\" x2=\"12\" y2=\"23\" />\n<path d=\"M20 16.58A5 5 0 0 0 18 7h-1.26A8 8 0 1 0 4 15.25\" />",
};
#[cfg(feature = "FiCloudSnow")]
const FI_CLOUD_SNOW: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M20 17.58A5 5 0 0 0 18 8h-1.26A8 8 0 1 0 4 16.25\" />\n<line x1=\"8\" y1=\"16\" x2=\"8.01\" y2=\"16\" />\n<line x1=\"8\" y1=\"20\" x2=\"8.01\" y2=\"20\" />\n<line x1=\"12\" y1=\"18\" x2=\"12.01\" y2=\"18\" />\n<line x1=\"12\" y1=\"22\" x2=\"12.01\" y2=\"22\" />\n<line x1=\"16\" y1=\"16\" x2=\"16.01\" y2=\"16\" />\n<line x1=\"16\" y1=\"20\" x2=\"16.01\" y2=\"20\" />",
};
#[cfg(feature = "FiCode")]
const FI_CODE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"16 18 22 12 16 6\" />\n<polyline points=\"8 6 2 12 8 18\" />",
};
#[cfg(feature = "FiCodepen")]
const FI_CODEPEN: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"12 2 22 8.5 22 15.5 12 22 2 15.5 2 8.5 12 2\" />\n<line x1=\"12\" y1=\"22\" x2=\"12\" y2=\"15.5\" />\n<polyline points=\"22 8.5 12 15.5 2 8.5\" />\n<polyline points=\"2 15.5 12 8.5 22 15.5\" />\n<line x1=\"12\" y1=\"2\" x2=\"12\" y2=\"8.5\" />",
};
#[cfg(feature = "FiCodesandbox")]
const FI_CODESANDBOX: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z\" />\n<polyline points=\"7.5 4.21 12 6.81 16.5 4.21\" />\n<polyline points=\"7.5 19.79 7.5 14.6 3 12\" />\n<polyline points=\"21 12 16.5 14.6 16.5 19.79\" />\n<polyline points=\"3.27 6.96 12 12.01 20.73 6.96\" />\n<line x1=\"12\" y1=\"22.08\" x2=\"12\" y2=\"12\" />",
};
#[cfg(feature = "FiCoffee")]
const FI_COFFEE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M18 8h1a4 4 0 0 1 0 8h-1\" />\n<path d=\"M2 8h16v9a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V8z\" />\n<line x1=\"6\" y1=\"1\" x2=\"6\" y2=\"4\" />\n<line x1=\"10\" y1=\"1\" x2=\"10\" y2=\"4\" />\n<line x1=\"14\" y1=\"1\" x2=\"14\" y2=\"4\" />",
};
#[cfg(feature = "FiColumns")]
const FI_COLUMNS: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M12 3h7a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-7m0-18H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h7m0-18v18\" />",
};
#[cfg(feature = "FiCommand")]
const FI_COMMAND: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M18 3a3 3 0 0 0-3 3v12a3 3 0 0 0 3 3 3 3 0 0 0 3-3 3 3 0 0 0-3-3H6a3 3 0 0 0-3 3 3 3 0 0 0 3 3 3 3 0 0 0 3-3V6a3 3 0 0 0-3-3 3 3 0 0 0-3 3 3 3 0 0 0 3 3h12a3 3 0 0 0 3-3 3 3 0 0 0-3-3z\" />",
};
#[cfg(feature = "FiCompass")]
const FI_COMPASS: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<polygon points=\"16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76\" />",
};
#[cfg(feature = "FiCopy")]
const FI_COPY: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"9\" y=\"9\" width=\"13\" height=\"13\" rx=\"2\" ry=\"2\" />\n<path d=\"M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1\" />",
};
#[cfg(feature = "FiCornerDownLeft")]
const FI_CORNER_DOWN_LEFT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"9 10 4 15 9 20\" />\n<path d=\"M20 4v7a4 4 0 0 1-4 4H4\" />",
};
#[cfg(feature = "FiCornerDownRight")]
const FI_CORNER_DOWN_RIGHT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"15 10 20 15 15 20\" />\n<path d=\"M4 4v7a4 4 0 0 0 4 4h12\" />",
};
#[cfg(feature = "FiCornerLeftDown")]
const FI_CORNER_LEFT_DOWN: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"14 15 9 20 4 15\" />\n<path d=\"M20 4h-7a4 4 0 0 0-4 4v12\" />",
};
#[cfg(feature = "FiCornerLeftUp")]
const FI_CORNER_LEFT_UP: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"14 9 9 4 4 9\" />\n<path d=\"M20 20h-7a4 4 0 0 1-4-4V4\" />",
};
#[cfg(feature = "FiCornerRightDown")]
const FI_CORNER_RIGHT_DOWN: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"10 15 15 20 20 15\" />\n<path d=\"M4 4h7a4 4 0 0 1 4 4v12\" />",
};
#[cfg(feature = "FiCornerRightUp")]
const FI_CORNER_RIGHT_UP: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"10 9 15 4 20 9\" />\n<path d=\"M4 20h7a4 4 0 0 0 4-4V4\" />",
};
#[cfg(feature = "FiCornerUpLeft")]
const FI_CORNER_UP_LEFT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"9 14 4 9 9 4\" />\n<path d=\"M20 20v-7a4 4 0 0 0-4-4H4\" />",
};
#[cfg(feature = "FiCornerUpRight")]
const FI_CORNER_UP_RIGHT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"15 14 20 9 15 4\" />\n<path d=\"M4 20v-7a4 4 0 0 1 4-4h12\" />",
};
#[cfg(feature = "FiCpu")]
const FI_CPU: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"4\" y=\"4\" width=\"16\" height=\"16\" rx=\"2\" ry=\"2\" />\n<rect x=\"9\" y=\"9\" width=\"6\" height=\"6\" />\n<line x1=\"9\" y1=\"1\" x2=\"9\" y2=\"4\" />\n<line x1=\"15\" y1=\"1\" x2=\"15\" y2=\"4\" />\n<line x1=\"9\" y1=\"20\" x2=\"9\" y2=\"23\" />\n<line x1=\"15\" y1=\"20\" x2=\"15\" y2=\"23\" />\n<line x1=\"20\" y1=\"9\" x2=\"23\" y2=\"9\" />\n<line x1=\"20\" y1=\"14\" x2=\"23\" y2=\"14\" />\n<line x1=\"1\" y1=\"9\" x2=\"4\" y2=\"9\" />\n<line x1=\"1\" y1=\"14\" x2=\"4\" y2=\"14\" />",
};
#[cfg(feature = "FiCreditCard")]
const FI_CREDIT_CARD: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"1\" y=\"4\" width=\"22\" height=\"16\" rx=\"2\" ry=\"2\" />\n<line x1=\"1\" y1=\"10\" x2=\"23\" y2=\"10\" />",
};
#[cfg(feature = "FiCrop")]
const FI_CROP: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M6.13 1L6 16a2 2 0 0 0 2 2h15\" />\n<path d=\"M1 6.13L16 6a2 2 0 0 1 2 2v15\" />",
};
#[cfg(feature = "FiCrosshair")]
const FI_CROSSHAIR: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<line x1=\"22\" y1=\"12\" x2=\"18\" y2=\"12\" />\n<line x1=\"6\" y1=\"12\" x2=\"2\" y2=\"12\" />\n<line x1=\"12\" y1=\"6\" x2=\"12\" y2=\"2\" />\n<line x1=\"12\" y1=\"22\" x2=\"12\" y2=\"18\" />",
};
#[cfg(feature = "FiDatabase")]
const FI_DATABASE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<ellipse cx=\"12\" cy=\"5\" rx=\"9\" ry=\"3\" />\n<path d=\"M21 12c0 1.66-4 3-9 3s-9-1.34-9-3\" />\n<path d=\"M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5\" />",
};
#[cfg(feature = "FiDelete")]
const FI_DELETE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M21 4H8l-7 8 7 8h13a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2z\" />\n<line x1=\"18\" y1=\"9\" x2=\"12\" y2=\"15\" />\n<line x1=\"12\" y1=\"9\" x2=\"18\" y2=\"15\" />",
};
#[cfg(feature = "FiDisc")]
const FI_DISC: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<circle cx=\"12\" cy=\"12\" r=\"3\" />",
};
#[cfg(feature = "FiDivide")]
const FI_DIVIDE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"6\" r=\"2\" />\n<line x1=\"5\" y1=\"12\" x2=\"19\" y2=\"12\" />\n<circle cx=\"12\" cy=\"18\" r=\"2\" />",
};
#[cfg(feature = "FiDivideCircle")]
const FI_DIVIDE_CIRCLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"8\" y1=\"12\" x2=\"16\" y2=\"12\" />\n<line x1=\"12\" y1=\"16\" x2=\"12\" y2=\"16\" />\n<line x1=\"12\" y1=\"8\" x2=\"12\" y2=\"8\" />\n<circle cx=\"12\" cy=\"12\" r=\"10\" />",
};
#[cfg(feature = "FiDivideSquare")]
const FI_DIVIDE_SQUARE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" />\n<line x1=\"8\" y1=\"12\" x2=\"16\" y2=\"12\" />\n<line x1=\"12\" y1=\"16\" x2=\"12\" y2=\"16\" />\n<line x1=\"12\" y1=\"8\" x2=\"12\" y2=\"8\" />",
};
#[cfg(feature = "FiDollarSign")]
const FI_DOLLAR_SIGN: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"12\" y1=\"1\" x2=\"12\" y2=\"23\" />\n<path d=\"M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6\" />",
};
#[cfg(feature = "FiDownload")]
const FI_DOWNLOAD: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4\" />\n<polyline points=\"7 10 12 15 17 10\" />\n<line x1=\"12\" y1=\"15\" x2=\"12\" y2=\"3\" />",
};
#[cfg(feature = "FiDownloadCloud")]
const FI_DOWNLOAD_CLOUD: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"8 17 12 21 16 17\" />\n<line x1=\"12\" y1=\"12\" x2=\"12\" y2=\"21\" />\n<path d=\"M20.88 18.09A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.29\" />",
};
#[cfg(feature = "FiDribbble")]
const FI_DRIBBBLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<path d=\"M8.56 2.75c4.37 6.03 6.02 9.42 8.03 17.72m2.54-15.38c-3.72 4.35-8.94 5.66-16.88 5.85m19.5 1.9c-3.5-.93-6.63-.82-8.94 0-2.58.92-5.01 2.86-7.44 6.32\" />",
};
#[cfg(feature = "FiDroplet")]
const FI_DROPLET: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M12 2.69l5.66 5.66a8 8 0 1 1-11.31 0z\" />",
};
#[cfg(feature = "FiEdit")]
const FI_EDIT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7\" />\n<path d=\"M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z\" />",
};
#[cfg(feature = "FiEdit2")]
const FI_EDIT2: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z\" />",
};
#[cfg(feature = "FiEdit3")]
const FI_EDIT3: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M12 20h9\" />\n<path d=\"M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z\" />",
};
#[cfg(feature = "FiExternalLink")]
const FI_EXTERNAL_LINK: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6\" />\n<polyline points=\"15 3 21 3 21 9\" />\n<line x1=\"10\" y1=\"14\" x2=\"21\" y2=\"3\" />",
};
#[cfg(feature = "FiEye")]
const FI_EYE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z\" />\n<circle cx=\"12\" cy=\"12\" r=\"3\" />",
};
#[cfg(feature = "FiEyeOff")]
const FI_EYE_OFF: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24\" />\n<line x1=\"1\" y1=\"1\" x2=\"23\" y2=\"23\" />",
};
#[cfg(feature = "FiFacebook")]
const FI_FACEBOOK: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z\" />",
};
#[cfg(feature = "FiFastForward")]
const FI_FAST_FORWARD: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"13 19 22 12 13 5 13 19\" />\n<polygon points=\"2 19 11 12 2 5 2 19\" />",
};
#[cfg(feature = "FiFeather")]
const FI_FEATHER: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M20.24 12.24a6 6 0 0 0-8.49-8.49L5 10.5V19h8.5z\" />\n<line x1=\"16\" y1=\"8\" x2=\"2\" y2=\"22\" />\n<line x1=\"17.5\" y1=\"15\" x2=\"9\" y2=\"15\" />",
};
#[cfg(feature = "FiFigma")]
const FI_FIGMA: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M5 5.5A3.5 3.5 0 0 1 8.5 2H12v7H8.5A3.5 3.5 0 0 1 5 5.5z\" />\n<path d=\"M12 2h3.5a3.5 3.5 0 1 1 0 7H12V2z\" />\n<path d=\"M12 12.5a3.5 3.5 0 1 1 7 0 3.5 3.5 0 1 1-7 0z\" />\n<path d=\"M5 19.5A3.5 3.5 0 0 1 8.5 16H12v3.5a3.5 3.5 0 1 1-7 0z\" />\n<path d=\"M5 12.5A3.5 3.5 0 0 1 8.5 9H12v7H8.5A3.5 3.5 0 0 1 5 12.5z\" />",
};
#[cfg(feature = "FiFile")]
const FI_FILE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z\" />\n<polyline points=\"13 2 13 9 20 9\" />",
};
#[cfg(feature = "FiFileMinus")]
const FI_FILE_MINUS: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z\" />\n<polyline points=\"14 2 14 8 20 8\" />\n<line x1=\"9\" y1=\"15\" x2=\"15\" y2=\"15\" />",
};
#[cfg(feature = "FiFilePlus")]
const FI_FILE_PLUS: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z\" />\n<polyline points=\"14 2 14 8 20 8\" />\n<line x1=\"12\" y1=\"18\" x2=\"12\" y2=\"12\" />\n<line x1=\"9\" y1=\"15\" x2=\"15\" y2=\"15\" />",
};
#[cfg(feature = "FiFileText")]
const FI_FILE_TEXT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z\" />\n<polyline points=\"14 2 14 8 20 8\" />\n<line x1=\"16\" y1=\"13\" x2=\"8\" y2=\"13\" />\n<line x1=\"16\" y1=\"17\" x2=\"8\" y2=\"17\" />\n<polyline points=\"10 9 9 9 8 9\" />",
};
#[cfg(feature = "FiFilm")]
const FI_FILM: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"2\" y=\"2\" width=\"20\" height=\"20\" rx=\"2.18\" ry=\"2.18\" />\n<line x1=\"7\" y1=\"2\" x2=\"7\" y2=\"22\" />\n<line x1=\"17\" y1=\"2\" x2=\"17\" y2=\"22\" />\n<line x1=\"2\" y1=\"12\" x2=\"22\" y2=\"12\" />\n<line x1=\"2\" y1=\"7\" x2=\"7\" y2=\"7\" />\n<line x1=\"2\" y1=\"17\" x2=\"7\" y2=\"17\" />\n<line x1=\"17\" y1=\"17\" x2=\"22\" y2=\"17\" />\n<line x1=\"17\" y1=\"7\" x2=\"22\" y2=\"7\" />",
};
#[cfg(feature = "FiFilter")]
const FI_FILTER: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3\" />",
};
#[cfg(feature = "FiFlag")]
const FI_FLAG: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1z\" />\n<line x1=\"4\" y1=\"22\" x2=\"4\" y2=\"15\" />",
};
#[cfg(feature = "FiFolder")]
const FI_FOLDER: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z\" />",
};
#[cfg(feature = "FiFolderMinus")]
const FI_FOLDER_MINUS: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z\" />\n<line x1=\"9\" y1=\"14\" x2=\"15\" y2=\"14\" />",
};
#[cfg(feature = "FiFolderPlus")]
const FI_FOLDER_PLUS: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z\" />\n<line x1=\"12\" y1=\"11\" x2=\"12\" y2=\"17\" />\n<line x1=\"9\" y1=\"14\" x2=\"15\" y2=\"14\" />",
};
#[cfg(feature = "FiFramer")]
const FI_FRAMER: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M5 16V9h14V2H5l14 14h-7m-7 0l7 7v-7m-7 0h7\" />",
};
#[cfg(feature = "FiFrown")]
const FI_FROWN: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<path d=\"M16 16s-1.5-2-4-2-4 2-4 2\" />\n<line x1=\"9\" y1=\"9\" x2=\"9.01\" y2=\"9\" />\n<line x1=\"15\" y1=\"9\" x2=\"15.01\" y2=\"9\" />",
};
#[cfg(feature = "FiGift")]
const FI_GIFT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"20 12 20 22 4 22 4 12\" />\n<rect x=\"2\" y=\"7\" width=\"20\" height=\"5\" />\n<line x1=\"12\" y1=\"22\" x2=\"12\" y2=\"7\" />\n<path d=\"M12 7H7.5a2.5 2.5 0 0 1 0-5C11 2 12 7 12 7z\" />\n<path d=\"M12 7h4.5a2.5 2.5 0 0 0 0-5C13 2 12 7 12 7z\" />",
};
#[cfg(feature = "FiGitBranch")]
const FI_GIT_BRANCH: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"6\" y1=\"3\" x2=\"6\" y2=\"15\" />\n<circle cx=\"18\" cy=\"6\" r=\"3\" />\n<circle cx=\"6\" cy=\"18\" r=\"3\" />\n<path d=\"M18 9a9 9 0 0 1-9 9\" />",
};
#[cfg(feature = "FiGitCommit")]
const FI_GIT_COMMIT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"4\" />\n<line x1=\"1.05\" y1=\"12\" x2=\"7\" y2=\"12\" />\n<line x1=\"17.01\" y1=\"12\" x2=\"22.96\" y2=\"12\" />",
};
#[cfg(feature = "FiGitMerge")]
const FI_GIT_MERGE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"18\" cy=\"18\" r=\"3\" />\n<circle cx=\"6\" cy=\"6\" r=\"3\" />\n<path d=\"M6 21V9a9 9 0 0 0 9 9\" />",
};
#[cfg(feature = "FiGitPullRequest")]
const FI_GIT_PULL_REQUEST: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"18\" cy=\"18\" r=\"3\" />\n<circle cx=\"6\" cy=\"6\" r=\"3\" />\n<path d=\"M13 6h3a2 2 0 0 1 2 2v7\" />\n<line x1=\"6\" y1=\"9\" x2=\"6\" y2=\"21\" />",
};
#[cfg(feature = "FiGithub")]
const FI_GITHUB: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22\" />",
};
#[cfg(feature = "FiGitlab")]
const FI_GITLAB: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M22.65 14.39L12 22.13 1.35 14.39a.84.84 0 0 1-.3-.94l1.22-3.78 2.44-7.51A.42.42 0 0 1 4.82 2a.43.43 0 0 1 .58 0 .42.42 0 0 1 .11.18l2.44 7.49h8.1l2.44-7.51A.42.42 0 0 1 18.6 2a.43.43 0 0 1 .58 0 .42.42 0 0 1 .11.18l2.44 7.51L23 13.45a.84.84 0 0 1-.35.94z\" />",
};
#[cfg(feature = "FiGlobe")]
const FI_GLOBE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<line x1=\"2\" y1=\"12\" x2=\"22\" y2=\"12\" />\n<path d=\"M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z\" />",
};
#[cfg(feature = "FiGrid")]
const FI_GRID: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"3\" y=\"3\" width=\"7\" height=\"7\" />\n<rect x=\"14\" y=\"3\" width=\"7\" height=\"7\" />\n<rect x=\"14\" y=\"14\" width=\"7\" height=\"7\" />\n<rect x=\"3\" y=\"14\" width=\"7\" height=\"7\" />",
};
#[cfg(feature = "FiHardDrive")]
const FI_HARD_DRIVE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"22\" y1=\"12\" x2=\"2\" y2=\"12\" />\n<path d=\"M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z\" />\n<line x1=\"6\" y1=\"16\" x2=\"6.01\" y2=\"16\" />\n<line x1=\"10\" y1=\"16\" x2=\"10.01\" y2=\"16\" />",
};
#[cfg(feature = "FiHash")]
const FI_HASH: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"4\" y1=\"9\" x2=\"20\" y2=\"9\" />\n<line x1=\"4\" y1=\"15\" x2=\"20\" y2=\"15\" />\n<line x1=\"10\" y1=\"3\" x2=\"8\" y2=\"21\" />\n<line x1=\"16\" y1=\"3\" x2=\"14\" y2=\"21\" />",
};
#[cfg(feature = "FiHeadphones")]
const FI_HEADPHONES: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M3 18v-6a9 9 0 0 1 18 0v6\" />\n<path d=\"M21 19a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3zM3 19a2 2 0 0 0 2 2h1a2 2 0 0 0 2-2v-3a2 2 0 0 0-2-2H3z\" />",
};
#[cfg(feature = "FiHeart")]
const FI_HEART: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z\" />",
};
#[cfg(feature = "FiHelpCircle")]
const FI_HELP_CIRCLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<path d=\"M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3\" />\n<line x1=\"12\" y1=\"17\" x2=\"12.01\" y2=\"17\" />",
};
#[cfg(feature = "FiHexagon")]
const FI_HEXAGON: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z\" />",
};
#[cfg(feature = "FiHome")]
const FI_HOME: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z\" />\n<polyline points=\"9 22 9 12 15 12 15 22\" />",
};
#[cfg(feature = "FiImage")]
const FI_IMAGE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" />\n<circle cx=\"8.5\" cy=\"8.5\" r=\"1.5\" />\n<polyline points=\"21 15 16 10 5 21\" />",
};
#[cfg(feature = "FiInbox")]
const FI_INBOX: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"22 12 16 12 14 15 10 15 8 12 2 12\" />\n<path d=\"M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z\" />",
};
#[cfg(feature = "FiInfo")]
const FI_INFO: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<line x1=\"12\" y1=\"16\" x2=\"12\" y2=\"12\" />\n<line x1=\"12\" y1=\"8\" x2=\"12.01\" y2=\"8\" />",
};
#[cfg(feature = "FiInstagram")]
const FI_INSTAGRAM: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"2\" y=\"2\" width=\"20\" height=\"20\" rx=\"5\" ry=\"5\" />\n<path d=\"M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z\" />\n<line x1=\"17.5\" y1=\"6.5\" x2=\"17.51\" y2=\"6.5\" />",
};
#[cfg(feature = "FiItalic")]
const FI_ITALIC: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"19\" y1=\"4\" x2=\"10\" y2=\"4\" />\n<line x1=\"14\" y1=\"20\" x2=\"5\" y2=\"20\" />\n<line x1=\"15\" y1=\"4\" x2=\"9\" y2=\"20\" />",
};
#[cfg(feature = "FiKey")]
const FI_KEY: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M21 2l-2 2m-7.61 7.61a5.5 5.5 0 1 1-7.778 7.778 5.5 5.5 0 0 1 7.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4\" />",
};
#[cfg(feature = "FiLayers")]
const FI_LAYERS: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"12 2 2 7 12 12 22 7 12 2\" />\n<polyline points=\"2 17 12 22 22 17\" />\n<polyline points=\"2 12 12 17 22 12\" />",
};
#[cfg(feature = "FiLayout")]
const FI_LAYOUT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" />\n<line x1=\"3\" y1=\"9\" x2=\"21\" y2=\"9\" />\n<line x1=\"9\" y1=\"21\" x2=\"9\" y2=\"9\" />",
};
#[cfg(feature = "FiLifeBuoy")]
const FI_LIFE_BUOY: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<circle cx=\"12\" cy=\"12\" r=\"4\" />\n<line x1=\"4.93\" y1=\"4.93\" x2=\"9.17\" y2=\"9.17\" />\n<line x1=\"14.83\" y1=\"14.83\" x2=\"19.07\" y2=\"19.07\" />\n<line x1=\"14.83\" y1=\"9.17\" x2=\"19.07\" y2=\"4.93\" />\n<line x1=\"14.83\" y1=\"9.17\" x2=\"18.36\" y2=\"5.64\" />\n<line x1=\"4.93\" y1=\"19.07\" x2=\"9.17\" y2=\"14.83\" />",
};
#[cfg(feature = "FiLink")]
const FI_LINK: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71\" />\n<path d=\"M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71\" />",
};
#[cfg(feature = "FiLink2")]
const FI_LINK2: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M15 7h3a5 5 0 0 1 5 5 5 5 0 0 1-5 5h-3m-6 0H6a5 5 0 0 1-5-5 5 5 0 0 1 5-5h3\" />\n<line x1=\"8\" y1=\"12\" x2=\"16\" y2=\"12\" />",
};
#[cfg(feature = "FiLinkedin")]
const FI_LINKEDIN: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z\" />\n<rect x=\"2\" y=\"9\" width=\"4\" height=\"12\" />\n<circle cx=\"4\" cy=\"4\" r=\"2\" />",
};
#[cfg(feature = "FiList")]
const FI_LIST: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"8\" y1=\"6\" x2=\"21\" y2=\"6\" />\n<line x1=\"8\" y1=\"12\" x2=\"21\" y2=\"12\" />\n<line x1=\"8\" y1=\"18\" x2=\"21\" y2=\"18\" />\n<line x1=\"3\" y1=\"6\" x2=\"3.01\" y2=\"6\" />\n<line x1=\"3\" y1=\"12\" x2=\"3.01\" y2=\"12\" />\n<line x1=\"3\" y1=\"18\" x2=\"3.01\" y2=\"18\" />",
};
#[cfg(feature = "FiLoader")]
const FI_LOADER: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"12\" y1=\"2\" x2=\"12\" y2=\"6\" />\n<line x1=\"12\" y1=\"18\" x2=\"12\" y2=\"22\" />\n<line x1=\"4.93\" y1=\"4.93\" x2=\"7.76\" y2=\"7.76\" />\n<line x1=\"16.24\" y1=\"16.24\" x2=\"19.07\" y2=\"19.07\" />\n<line x1=\"2\" y1=\"12\" x2=\"6\" y2=\"12\" />\n<line x1=\"18\" y1=\"12\" x2=\"22\" y2=\"12\" />\n<line x1=\"4.93\" y1=\"19.07\" x2=\"7.76\" y2=\"16.24\" />\n<line x1=\"16.24\" y1=\"7.76\" x2=\"19.07\" y2=\"4.93\" />",
};
#[cfg(feature = "FiLock")]
const FI_LOCK: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"3\" y=\"11\" width=\"18\" height=\"11\" rx=\"2\" ry=\"2\" />\n<path d=\"M7 11V7a5 5 0 0 1 10 0v4\" />",
};
#[cfg(feature = "FiLogIn")]
const FI_LOG_IN: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4\" />\n<polyline points=\"10 17 15 12 10 7\" />\n<line x1=\"15\" y1=\"12\" x2=\"3\" y2=\"12\" />",
};
#[cfg(feature = "FiLogOut")]
const FI_LOG_OUT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4\" />\n<polyline points=\"16 17 21 12 16 7\" />\n<line x1=\"21\" y1=\"12\" x2=\"9\" y2=\"12\" />",
};
#[cfg(feature = "FiMail")]
const FI_MAIL: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z\" />\n<polyline points=\"22,6 12,13 2,6\" />",
};
#[cfg(feature = "FiMap")]
const FI_MAP: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"1 6 1 22 8 18 16 22 23 18 23 2 16 6 8 2 1 6\" />\n<line x1=\"8\" y1=\"2\" x2=\"8\" y2=\"18\" />\n<line x1=\"16\" y1=\"6\" x2=\"16\" y2=\"22\" />",
};
#[cfg(feature = "FiMapPin")]
const FI_MAP_PIN: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z\" />\n<circle cx=\"12\" cy=\"10\" r=\"3\" />",
};
#[cfg(feature = "FiMaximize")]
const FI_MAXIMIZE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3\" />",
};
#[cfg(feature = "FiMaximize2")]
const FI_MAXIMIZE2: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"15 3 21 3 21 9\" />\n<polyline points=\"9 21 3 21 3 15\" />\n<line x1=\"21\" y1=\"3\" x2=\"14\" y2=\"10\" />\n<line x1=\"3\" y1=\"21\" x2=\"10\" y2=\"14\" />",
};
#[cfg(feature = "FiMeh")]
const FI_MEH: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<line x1=\"8\" y1=\"15\" x2=\"16\" y2=\"15\" />\n<line x1=\"9\" y1=\"9\" x2=\"9.01\" y2=\"9\" />\n<line x1=\"15\" y1=\"9\" x2=\"15.01\" y2=\"9\" />",
};
#[cfg(feature = "FiMenu")]
const FI_MENU: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"3\" y1=\"12\" x2=\"21\" y2=\"12\" />\n<line x1=\"3\" y1=\"6\" x2=\"21\" y2=\"6\" />\n<line x1=\"3\" y1=\"18\" x2=\"21\" y2=\"18\" />",
};
#[cfg(feature = "FiMessageCircle")]
const FI_MESSAGE_CIRCLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z\" />",
};
#[cfg(feature = "FiMessageSquare")]
const FI_MESSAGE_SQUARE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z\" />",
};
#[cfg(feature = "FiMic")]
const FI_MIC: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z\" />\n<path d=\"M19 10v2a7 7 0 0 1-14 0v-2\" />\n<line x1=\"12\" y1=\"19\" x2=\"12\" y2=\"23\" />\n<line x1=\"8\" y1=\"23\" x2=\"16\" y2=\"23\" />",
};
#[cfg(feature = "FiMicOff")]
const FI_MIC_OFF: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"1\" y1=\"1\" x2=\"23\" y2=\"23\" />\n<path d=\"M9 9v3a3 3 0 0 0 5.12 2.12M15 9.34V4a3 3 0 0 0-5.94-.6\" />\n<path d=\"M17 16.95A7 7 0 0 1 5 12v-2m14 0v2a7 7 0 0 1-.11 1.23\" />\n<line x1=\"12\" y1=\"19\" x2=\"12\" y2=\"23\" />\n<line x1=\"8\" y1=\"23\" x2=\"16\" y2=\"23\" />",
};
#[cfg(feature = "FiMinimize")]
const FI_MINIMIZE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M8 3v3a2 2 0 0 1-2 2H3m18 0h-3a2 2 0 0 1-2-2V3m0 18v-3a2 2 0 0 1 2-2h3M3 16h3a2 2 0 0 1 2 2v3\" />",
};
#[cfg(feature = "FiMinimize2")]
const FI_MINIMIZE2: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"4 14 10 14 10 20\" />\n<polyline points=\"20 10 14 10 14 4\" />\n<line x1=\"14\" y1=\"10\" x2=\"21\" y2=\"3\" />\n<line x1=\"3\" y1=\"21\" x2=\"10\" y2=\"14\" />",
};
#[cfg(feature = "FiMinus")]
const FI_MINUS: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"5\" y1=\"12\" x2=\"19\" y2=\"12\" />",
};
#[cfg(feature = "FiMinusCircle")]
const FI_MINUS_CIRCLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<line x1=\"8\" y1=\"12\" x2=\"16\" y2=\"12\" />",
};
#[cfg(feature = "FiMinusSquare")]
const FI_MINUS_SQUARE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" />\n<line x1=\"8\" y1=\"12\" x2=\"16\" y2=\"12\" />",
};
#[cfg(feature = "FiMonitor")]
const FI_MONITOR: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"2\" y=\"3\" width=\"20\" height=\"14\" rx=\"2\" ry=\"2\" />\n<line x1=\"8\" y1=\"21\" x2=\"16\" y2=\"21\" />\n<line x1=\"12\" y1=\"17\" x2=\"12\" y2=\"21\" />",
};
#[cfg(feature = "FiMoon")]
const FI_MOON: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z\" />",
};
#[cfg(feature = "FiMoreHorizontal")]
const FI_MORE_HORIZONTAL: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"1\" />\n<circle cx=\"19\" cy=\"12\" r=\"1\" />\n<circle cx=\"5\" cy=\"12\" r=\"1\" />",
};
#[cfg(feature = "FiMoreVertical")]
const FI_MORE_VERTICAL: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"1\" />\n<circle cx=\"12\" cy=\"5\" r=\"1\" />\n<circle cx=\"12\" cy=\"19\" r=\"1\" />",
};
#[cfg(feature = "FiMousePointer")]
const FI_MOUSE_POINTER: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M3 3l7.07 16.97 2.51-7.39 7.39-2.51L3 3z\" />\n<path d=\"M13 13l6 6\" />",
};
#[cfg(feature = "FiMove")]
const FI_MOVE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"5 9 2 12 5 15\" />\n<polyline points=\"9 5 12 2 15 5\" />\n<polyline points=\"15 19 12 22 9 19\" />\n<polyline points=\"19 9 22 12 19 15\" />\n<line x1=\"2\" y1=\"12\" x2=\"22\" y2=\"12\" />\n<line x1=\"12\" y1=\"2\" x2=\"12\" y2=\"22\" />",
};
#[cfg(feature = "FiMusic")]
const FI_MUSIC: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M9 18V5l12-2v13\" />\n<circle cx=\"6\" cy=\"18\" r=\"3\" />\n<circle cx=\"18\" cy=\"16\" r=\"3\" />",
};
#[cfg(feature = "FiNavigation")]
const FI_NAVIGATION: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"3 11 22 2 13 21 11 13 3 11\" />",
};
#[cfg(feature = "FiNavigation2")]
const FI_NAVIGATION2: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"12 2 19 21 12 17 5 21 12 2\" />",
};
#[cfg(feature = "FiOctagon")]
const FI_OCTAGON: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2\" />",
};
#[cfg(feature = "FiPackage")]
const FI_PACKAGE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"16.5\" y1=\"9.4\" x2=\"7.5\" y2=\"4.21\" />\n<path d=\"M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z\" />\n<polyline points=\"3.27 6.96 12 12.01 20.73 6.96\" />\n<line x1=\"12\" y1=\"22.08\" x2=\"12\" y2=\"12\" />",
};
#[cfg(feature = "FiPaperclip")]
const FI_PAPERCLIP: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48\" />",
};
#[cfg(feature = "FiPause")]
const FI_PAUSE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"6\" y=\"4\" width=\"4\" height=\"16\" />\n<rect x=\"14\" y=\"4\" width=\"4\" height=\"16\" />",
};
#[cfg(feature = "FiPauseCircle")]
const FI_PAUSE_CIRCLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<line x1=\"10\" y1=\"15\" x2=\"10\" y2=\"9\" />\n<line x1=\"14\" y1=\"15\" x2=\"14\" y2=\"9\" />",
};
#[cfg(feature = "FiPenTool")]
const FI_PEN_TOOL: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M12 19l7-7 3 3-7 7-3-3z\" />\n<path d=\"M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z\" />\n<path d=\"M2 2l7.586 7.586\" />\n<circle cx=\"11\" cy=\"11\" r=\"2\" />",
};
#[cfg(feature = "FiPercent")]
const FI_PERCENT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"19\" y1=\"5\" x2=\"5\" y2=\"19\" />\n<circle cx=\"6.5\" cy=\"6.5\" r=\"2.5\" />\n<circle cx=\"17.5\" cy=\"17.5\" r=\"2.5\" />",
};
#[cfg(feature = "FiPhone")]
const FI_PHONE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z\" />",
};
#[cfg(feature = "FiPhoneCall")]
const FI_PHONE_CALL: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M15.05 5A5 5 0 0 1 19 8.95M15.05 1A9 9 0 0 1 23 8.94m-1 7.98v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z\" />",
};
#[cfg(feature = "FiPhoneForwarded")]
const FI_PHONE_FORWARDED: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"19 1 23 5 19 9\" />\n<line x1=\"15\" y1=\"5\" x2=\"23\" y2=\"5\" />\n<path d=\"M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z\" />",
};
#[cfg(feature = "FiPhoneIncoming")]
const FI_PHONE_INCOMING: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"16 2 16 8 22 8\" />\n<line x1=\"23\" y1=\"1\" x2=\"16\" y2=\"8\" />\n<path d=\"M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z\" />",
};
#[cfg(feature = "FiPhoneMissed")]
const FI_PHONE_MISSED: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"23\" y1=\"1\" x2=\"17\" y2=\"7\" />\n<line x1=\"17\" y1=\"1\" x2=\"23\" y2=\"7\" />\n<path d=\"M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z\" />",
};
#[cfg(feature = "FiPhoneOff")]
const FI_PHONE_OFF: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M10.68 13.31a16 16 0 0 0 3.41 2.6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7 2 2 0 0 1 1.72 2v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.42 19.42 0 0 1-3.33-2.67m-2.67-3.34a19.79 19.79 0 0 1-3.07-8.63A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91\" />\n<line x1=\"23\" y1=\"1\" x2=\"1\" y2=\"23\" />",
};
#[cfg(feature = "FiPhoneOutgoing")]
const FI_PHONE_OUTGOING: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"23 7 23 1 17 1\" />\n<line x1=\"16\" y1=\"8\" x2=\"23\" y2=\"1\" />\n<path d=\"M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z\" />",
};
#[cfg(feature = "FiPieChart")]
const FI_PIE_CHART: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M21.21 15.89A10 10 0 1 1 8 2.83\" />\n<path d=\"M22 12A10 10 0 0 0 12 2v10z\" />",
};
#[cfg(feature = "FiPlay")]
const FI_PLAY: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"5 3 19 12 5 21 5 3\" />",
};
#[cfg(feature = "FiPlayCircle")]
const FI_PLAY_CIRCLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<polygon points=\"10 8 16 12 10 16 10 8\" />",
};
#[cfg(feature = "FiPlus")]
const FI_PLUS: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"12\" y1=\"5\" x2=\"12\" y2=\"19\" />\n<line x1=\"5\" y1=\"12\" x2=\"19\" y2=\"12\" />",
};
#[cfg(feature = "FiPlusCircle")]
const FI_PLUS_CIRCLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<line x1=\"12\" y1=\"8\" x2=\"12\" y2=\"16\" />\n<line x1=\"8\" y1=\"12\" x2=\"16\" y2=\"12\" />",
};
#[cfg(feature = "FiPlusSquare")]
const FI_PLUS_SQUARE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" />\n<line x1=\"12\" y1=\"8\" x2=\"12\" y2=\"16\" />\n<line x1=\"8\" y1=\"12\" x2=\"16\" y2=\"12\" />",
};
#[cfg(feature = "FiPocket")]
const FI_POCKET: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M4 3h16a2 2 0 0 1 2 2v6a10 10 0 0 1-10 10A10 10 0 0 1 2 11V5a2 2 0 0 1 2-2z\" />\n<polyline points=\"8 10 12 14 16 10\" />",
};
#[cfg(feature = "FiPower")]
const FI_POWER: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M18.36 6.64a9 9 0 1 1-12.73 0\" />\n<line x1=\"12\" y1=\"2\" x2=\"12\" y2=\"12\" />",
};
#[cfg(feature = "FiPrinter")]
const FI_PRINTER: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"6 9 6 2 18 2 18 9\" />\n<path d=\"M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2\" />\n<rect x=\"6\" y=\"14\" width=\"12\" height=\"8\" />",
};
#[cfg(feature = "FiRadio")]
const FI_RADIO: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"2\" />\n<path d=\"M16.24 7.76a6 6 0 0 1 0 8.49m-8.48-.01a6 6 0 0 1 0-8.49m11.31-2.82a10 10 0 0 1 0 14.14m-14.14 0a10 10 0 0 1 0-14.14\" />",
};
#[cfg(feature = "FiRefreshCcw")]
const FI_REFRESH_CCW: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"1 4 1 10 7 10\" />\n<polyline points=\"23 20 23 14 17 14\" />\n<path d=\"M20.49 9A9 9 0 0 0 5.64 5.64L1 10m22 4l-4.64 4.36A9 9 0 0 1 3.51 15\" />",
};
#[cfg(feature = "FiRefreshCw")]
const FI_REFRESH_CW: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"23 4 23 10 17 10\" />\n<polyline points=\"1 20 1 14 7 14\" />\n<path d=\"M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15\" />",
};
#[cfg(feature = "FiRepeat")]
const FI_REPEAT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"17 1 21 5 17 9\" />\n<path d=\"M3 11V9a4 4 0 0 1 4-4h14\" />\n<polyline points=\"7 23 3 19 7 15\" />\n<path d=\"M21 13v2a4 4 0 0 1-4 4H3\" />",
};
#[cfg(feature = "FiRewind")]
const FI_REWIND: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"11 19 2 12 11 5 11 19\" />\n<polygon points=\"22 19 13 12 22 5 22 19\" />",
};
#[cfg(feature = "FiRotateCcw")]
const FI_ROTATE_CCW: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"1 4 1 10 7 10\" />\n<path d=\"M3.51 15a9 9 0 1 0 2.13-9.36L1 10\" />",
};
#[cfg(feature = "FiRotateCw")]
const FI_ROTATE_CW: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"23 4 23 10 17 10\" />\n<path d=\"M20.49 15a9 9 0 1 1-2.12-9.36L23 10\" />",
};
#[cfg(feature = "FiRss")]
const FI_RSS: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M4 11a9 9 0 0 1 9 9\" />\n<path d=\"M4 4a16 16 0 0 1 16 16\" />\n<circle cx=\"5\" cy=\"19\" r=\"1\" />",
};
#[cfg(feature = "FiSave")]
const FI_SAVE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z\" />\n<polyline points=\"17 21 17 13 7 13 7 21\" />\n<polyline points=\"7 3 7 8 15 8\" />",
};
#[cfg(feature = "FiScissors")]
const FI_SCISSORS: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"6\" cy=\"6\" r=\"3\" />\n<circle cx=\"6\" cy=\"18\" r=\"3\" />\n<line x1=\"20\" y1=\"4\" x2=\"8.12\" y2=\"15.88\" />\n<line x1=\"14.47\" y1=\"14.48\" x2=\"20\" y2=\"20\" />\n<line x1=\"8.12\" y1=\"8.12\" x2=\"12\" y2=\"12\" />",
};
#[cfg(feature = "FiSearch")]
const FI_SEARCH: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"11\" cy=\"11\" r=\"8\" />\n<line x1=\"21\" y1=\"21\" x2=\"16.65\" y2=\"16.65\" />",
};
#[cfg(feature = "FiSend")]
const FI_SEND: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"22\" y1=\"2\" x2=\"11\" y2=\"13\" />\n<polygon points=\"22 2 15 22 11 13 2 9 22 2\" />",
};
#[cfg(feature = "FiServer")]
const FI_SERVER: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"2\" y=\"2\" width=\"20\" height=\"8\" rx=\"2\" ry=\"2\" />\n<rect x=\"2\" y=\"14\" width=\"20\" height=\"8\" rx=\"2\" ry=\"2\" />\n<line x1=\"6\" y1=\"6\" x2=\"6.01\" y2=\"6\" />\n<line x1=\"6\" y1=\"18\" x2=\"6.01\" y2=\"18\" />",
};
#[cfg(feature = "FiSettings")]
const FI_SETTINGS: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"3\" />\n<path d=\"M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z\" />",
};
#[cfg(feature = "FiShare")]
const FI_SHARE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8\" />\n<polyline points=\"16 6 12 2 8 6\" />\n<line x1=\"12\" y1=\"2\" x2=\"12\" y2=\"15\" />",
};
#[cfg(feature = "FiShare2")]
const FI_SHARE2: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"18\" cy=\"5\" r=\"3\" />\n<circle cx=\"6\" cy=\"12\" r=\"3\" />\n<circle cx=\"18\" cy=\"19\" r=\"3\" />\n<line x1=\"8.59\" y1=\"13.51\" x2=\"15.42\" y2=\"17.49\" />\n<line x1=\"15.41\" y1=\"6.51\" x2=\"8.59\" y2=\"10.49\" />",
};
#[cfg(feature = "FiShield")]
const FI_SHIELD: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z\" />",
};
#[cfg(feature = "FiShieldOff")]
const FI_SHIELD_OFF: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M19.69 14a6.9 6.9 0 0 0 .31-2V5l-8-3-3.16 1.18\" />\n<path d=\"M4.73 4.73L4 5v7c0 6 8 10 8 10a20.29 20.29 0 0 0 5.62-4.38\" />\n<line x1=\"1\" y1=\"1\" x2=\"23\" y2=\"23\" />",
};
#[cfg(feature = "FiShoppingBag")]
const FI_SHOPPING_BAG: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4z\" />\n<line x1=\"3\" y1=\"6\" x2=\"21\" y2=\"6\" />\n<path d=\"M16 10a4 4 0 0 1-8 0\" />",
};
#[cfg(feature = "FiShoppingCart")]
const FI_SHOPPING_CART: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"9\" cy=\"21\" r=\"1\" />\n<circle cx=\"20\" cy=\"21\" r=\"1\" />\n<path d=\"M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6\" />",
};
#[cfg(feature = "FiShuffle")]
const FI_SHUFFLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"16 3 21 3 21 8\" />\n<line x1=\"4\" y1=\"20\" x2=\"21\" y2=\"3\" />\n<polyline points=\"21 16 21 21 16 21\" />\n<line x1=\"15\" y1=\"15\" x2=\"21\" y2=\"21\" />\n<line x1=\"4\" y1=\"4\" x2=\"9\" y2=\"9\" />",
};
#[cfg(feature = "FiSidebar")]
const FI_SIDEBAR: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" />\n<line x1=\"9\" y1=\"3\" x2=\"9\" y2=\"21\" />",
};
#[cfg(feature = "FiSkipBack")]
const FI_SKIP_BACK: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"19 20 9 12 19 4 19 20\" />\n<line x1=\"5\" y1=\"19\" x2=\"5\" y2=\"5\" />",
};
#[cfg(feature = "FiSkipForward")]
const FI_SKIP_FORWARD: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"5 4 15 12 5 20 5 4\" />\n<line x1=\"19\" y1=\"5\" x2=\"19\" y2=\"19\" />",
};
#[cfg(feature = "FiSlack")]
const FI_SLACK: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M14.5 10c-.83 0-1.5-.67-1.5-1.5v-5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5v5c0 .83-.67 1.5-1.5 1.5z\" />\n<path d=\"M20.5 10H19V8.5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z\" />\n<path d=\"M9.5 14c.83 0 1.5.67 1.5 1.5v5c0 .83-.67 1.5-1.5 1.5S8 21.33 8 20.5v-5c0-.83.67-1.5 1.5-1.5z\" />\n<path d=\"M3.5 14H5v1.5c0 .83-.67 1.5-1.5 1.5S2 16.33 2 15.5 2.67 14 3.5 14z\" />\n<path d=\"M14 14.5c0-.83.67-1.5 1.5-1.5h5c.83 0 1.5.67 1.5 1.5s-.67 1.5-1.5 1.5h-5c-.83 0-1.5-.67-1.5-1.5z\" />\n<path d=\"M15.5 19H14v1.5c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5-.67-1.5-1.5-1.5z\" />\n<path d=\"M10 9.5C10 8.67 9.33 8 8.5 8h-5C2.67 8 2 8.67 2 9.5S2.67 11 3.5 11h5c.83 0 1.5-.67 1.5-1.5z\" />\n<path d=\"M8.5 5H10V3.5C10 2.67 9.33 2 8.5 2S7 2.67 7 3.5 7.67 5 8.5 5z\" />",
};
#[cfg(feature = "FiSlash")]
const FI_SLASH: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<line x1=\"4.93\" y1=\"4.93\" x2=\"19.07\" y2=\"19.07\" />",
};
#[cfg(feature = "FiSliders")]
const FI_SLIDERS: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"4\" y1=\"21\" x2=\"4\" y2=\"14\" />\n<line x1=\"4\" y1=\"10\" x2=\"4\" y2=\"3\" />\n<line x1=\"12\" y1=\"21\" x2=\"12\" y2=\"12\" />\n<line x1=\"12\" y1=\"8\" x2=\"12\" y2=\"3\" />\n<line x1=\"20\" y1=\"21\" x2=\"20\" y2=\"16\" />\n<line x1=\"20\" y1=\"12\" x2=\"20\" y2=\"3\" />\n<line x1=\"1\" y1=\"14\" x2=\"7\" y2=\"14\" />\n<line x1=\"9\" y1=\"8\" x2=\"15\" y2=\"8\" />\n<line x1=\"17\" y1=\"16\" x2=\"23\" y2=\"16\" />",
};
#[cfg(feature = "FiSmartphone")]
const FI_SMARTPHONE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"5\" y=\"2\" width=\"14\" height=\"20\" rx=\"2\" ry=\"2\" />\n<line x1=\"12\" y1=\"18\" x2=\"12.01\" y2=\"18\" />",
};
#[cfg(feature = "FiSmile")]
const FI_SMILE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<path d=\"M8 14s1.5 2 4 2 4-2 4-2\" />\n<line x1=\"9\" y1=\"9\" x2=\"9.01\" y2=\"9\" />\n<line x1=\"15\" y1=\"9\" x2=\"15.01\" y2=\"9\" />",
};
#[cfg(feature = "FiSpeaker")]
const FI_SPEAKER: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"4\" y=\"2\" width=\"16\" height=\"20\" rx=\"2\" ry=\"2\" />\n<circle cx=\"12\" cy=\"14\" r=\"4\" />\n<line x1=\"12\" y1=\"6\" x2=\"12.01\" y2=\"6\" />",
};
#[cfg(feature = "FiSquare")]
const FI_SQUARE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" />",
};
#[cfg(feature = "FiStar")]
const FI_STAR: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2\" />",
};
#[cfg(feature = "FiStopCircle")]
const FI_STOP_CIRCLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<rect x=\"9\" y=\"9\" width=\"6\" height=\"6\" />",
};
#[cfg(feature = "FiSun")]
const FI_SUN: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"5\" />\n<line x1=\"12\" y1=\"1\" x2=\"12\" y2=\"3\" />\n<line x1=\"12\" y1=\"21\" x2=\"12\" y2=\"23\" />\n<line x1=\"4.22\" y1=\"4.22\" x2=\"5.64\" y2=\"5.64\" />\n<line x1=\"18.36\" y1=\"18.36\" x2=\"19.78\" y2=\"19.78\" />\n<line x1=\"1\" y1=\"12\" x2=\"3\" y2=\"12\" />\n<line x1=\"21\" y1=\"12\" x2=\"23\" y2=\"12\" />\n<line x1=\"4.22\" y1=\"19.78\" x2=\"5.64\" y2=\"18.36\" />\n<line x1=\"18.36\" y1=\"5.64\" x2=\"19.78\" y2=\"4.22\" />",
};
#[cfg(feature = "FiSunrise")]
const FI_SUNRISE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M17 18a5 5 0 0 0-10 0\" />\n<line x1=\"12\" y1=\"2\" x2=\"12\" y2=\"9\" />\n<line x1=\"4.22\" y1=\"10.22\" x2=\"5.64\" y2=\"11.64\" />\n<line x1=\"1\" y1=\"18\" x2=\"3\" y2=\"18\" />\n<line x1=\"21\" y1=\"18\" x2=\"23\" y2=\"18\" />\n<line x1=\"18.36\" y1=\"11.64\" x2=\"19.78\" y2=\"10.22\" />\n<line x1=\"23\" y1=\"22\" x2=\"1\" y2=\"22\" />\n<polyline points=\"8 6 12 2 16 6\" />",
};
#[cfg(feature = "FiSunset")]
const FI_SUNSET: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M17 18a5 5 0 0 0-10 0\" />\n<line x1=\"12\" y1=\"9\" x2=\"12\" y2=\"2\" />\n<line x1=\"4.22\" y1=\"10.22\" x2=\"5.64\" y2=\"11.64\" />\n<line x1=\"1\" y1=\"18\" x2=\"3\" y2=\"18\" />\n<line x1=\"21\" y1=\"18\" x2=\"23\" y2=\"18\" />\n<line x1=\"18.36\" y1=\"11.64\" x2=\"19.78\" y2=\"10.22\" />\n<line x1=\"23\" y1=\"22\" x2=\"1\" y2=\"22\" />\n<polyline points=\"16 5 12 9 8 5\" />",
};
#[cfg(feature = "FiTable")]
const FI_TABLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M9 3H5a2 2 0 0 0-2 2v4m6-6h10a2 2 0 0 1 2 2v4M9 3v18m0 0h10a2 2 0 0 0 2-2V9M9 21H5a2 2 0 0 1-2-2V9m0 0h18\" />",
};
#[cfg(feature = "FiTablet")]
const FI_TABLET: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"4\" y=\"2\" width=\"16\" height=\"20\" rx=\"2\" ry=\"2\" />\n<line x1=\"12\" y1=\"18\" x2=\"12.01\" y2=\"18\" />",
};
#[cfg(feature = "FiTag")]
const FI_TAG: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z\" />\n<line x1=\"7\" y1=\"7\" x2=\"7.01\" y2=\"7\" />",
};
#[cfg(feature = "FiTarget")]
const FI_TARGET: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<circle cx=\"12\" cy=\"12\" r=\"6\" />\n<circle cx=\"12\" cy=\"12\" r=\"2\" />",
};
#[cfg(feature = "FiTerminal")]
const FI_TERMINAL: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"4 17 10 11 4 5\" />\n<line x1=\"12\" y1=\"19\" x2=\"20\" y2=\"19\" />",
};
#[cfg(feature = "FiThermometer")]
const FI_THERMOMETER: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M14 14.76V3.5a2.5 2.5 0 0 0-5 0v11.26a4.5 4.5 0 1 0 5 0z\" />",
};
#[cfg(feature = "FiThumbsDown")]
const FI_THUMBS_DOWN: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M10 15v4a3 3 0 0 0 3 3l4-9V2H5.72a2 2 0 0 0-2 1.7l-1.38 9a2 2 0 0 0 2 2.3zm7-13h2.67A2.31 2.31 0 0 1 22 4v7a2.31 2.31 0 0 1-2.33 2H17\" />",
};
#[cfg(feature = "FiThumbsUp")]
const FI_THUMBS_UP: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M14 9V5a3 3 0 0 0-3-3l-4 9v11h11.28a2 2 0 0 0 2-1.7l1.38-9a2 2 0 0 0-2-2.3zM7 22H4a2 2 0 0 1-2-2v-7a2 2 0 0 1 2-2h3\" />",
};
#[cfg(feature = "FiToggleLeft")]
const FI_TOGGLE_LEFT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"1\" y=\"5\" width=\"22\" height=\"14\" rx=\"7\" ry=\"7\" />\n<circle cx=\"8\" cy=\"12\" r=\"3\" />",
};
#[cfg(feature = "FiToggleRight")]
const FI_TOGGLE_RIGHT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"1\" y=\"5\" width=\"22\" height=\"14\" rx=\"7\" ry=\"7\" />\n<circle cx=\"16\" cy=\"12\" r=\"3\" />",
};
#[cfg(feature = "FiTool")]
const FI_TOOL: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z\" />",
};
#[cfg(feature = "FiTrash")]
const FI_TRASH: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"3 6 5 6 21 6\" />\n<path d=\"M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2\" />",
};
#[cfg(feature = "FiTrash2")]
const FI_TRASH2: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"3 6 5 6 21 6\" />\n<path d=\"M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2\" />\n<line x1=\"10\" y1=\"11\" x2=\"10\" y2=\"17\" />\n<line x1=\"14\" y1=\"11\" x2=\"14\" y2=\"17\" />",
};
#[cfg(feature = "FiTrello")]
const FI_TRELLO: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" />\n<rect x=\"7\" y=\"7\" width=\"3\" height=\"9\" />\n<rect x=\"14\" y=\"7\" width=\"3\" height=\"5\" />",
};
#[cfg(feature = "FiTrendingDown")]
const FI_TRENDING_DOWN: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"23 18 13.5 8.5 8.5 13.5 1 6\" />\n<polyline points=\"17 18 23 18 23 12\" />",
};
#[cfg(feature = "FiTrendingUp")]
const FI_TRENDING_UP: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"23 6 13.5 15.5 8.5 10.5 1 18\" />\n<polyline points=\"17 6 23 6 23 12\" />",
};
#[cfg(feature = "FiTriangle")]
const FI_TRIANGLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z\" />",
};
#[cfg(feature = "FiTruck")]
const FI_TRUCK: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"1\" y=\"3\" width=\"15\" height=\"13\" />\n<polygon points=\"16 8 20 8 23 11 23 16 16 16 16 8\" />\n<circle cx=\"5.5\" cy=\"18.5\" r=\"2.5\" />\n<circle cx=\"18.5\" cy=\"18.5\" r=\"2.5\" />",
};
#[cfg(feature = "FiTv")]
const FI_TV: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"2\" y=\"7\" width=\"20\" height=\"15\" rx=\"2\" ry=\"2\" />\n<polyline points=\"17 2 12 7 7 2\" />",
};
#[cfg(feature = "FiTwitch")]
const FI_TWITCH: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M21 2H3v16h5v4l4-4h5l4-4V2zM11 11V7M16 11V7\" />",
};
#[cfg(feature = "FiTwitter")]
const FI_TWITTER: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M23 3a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-7.86 3v1A10.66 10.66 0 0 1 3 4s-4 9 5 13a11.64 11.64 0 0 1-7 2c9 5 20 0 20-11.5a4.5 4.5 0 0 0-.08-.83A7.72 7.72 0 0 0 23 3z\" />",
};
#[cfg(feature = "FiType")]
const FI_TYPE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"4 7 4 4 20 4 20 7\" />\n<line x1=\"9\" y1=\"20\" x2=\"15\" y2=\"20\" />\n<line x1=\"12\" y1=\"4\" x2=\"12\" y2=\"20\" />",
};
#[cfg(feature = "FiUmbrella")]
const FI_UMBRELLA: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M23 12a11.05 11.05 0 0 0-22 0zm-5 7a3 3 0 0 1-6 0v-7\" />",
};
#[cfg(feature = "FiUnderline")]
const FI_UNDERLINE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M6 3v7a6 6 0 0 0 6 6 6 6 0 0 0 6-6V3\" />\n<line x1=\"4\" y1=\"21\" x2=\"20\" y2=\"21\" />",
};
#[cfg(feature = "FiUnlock")]
const FI_UNLOCK: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"3\" y=\"11\" width=\"18\" height=\"11\" rx=\"2\" ry=\"2\" />\n<path d=\"M7 11V7a5 5 0 0 1 9.9-1\" />",
};
#[cfg(feature = "FiUpload")]
const FI_UPLOAD: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4\" />\n<polyline points=\"17 8 12 3 7 8\" />\n<line x1=\"12\" y1=\"3\" x2=\"12\" y2=\"15\" />",
};
#[cfg(feature = "FiUploadCloud")]
const FI_UPLOAD_CLOUD: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"16 16 12 12 8 16\" />\n<line x1=\"12\" y1=\"12\" x2=\"12\" y2=\"21\" />\n<path d=\"M20.39 18.39A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.3\" />\n<polyline points=\"16 16 12 12 8 16\" />",
};
#[cfg(feature = "FiUser")]
const FI_USER: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2\" />\n<circle cx=\"12\" cy=\"7\" r=\"4\" />",
};
#[cfg(feature = "FiUserCheck")]
const FI_USER_CHECK: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2\" />\n<circle cx=\"8.5\" cy=\"7\" r=\"4\" />\n<polyline points=\"17 11 19 13 23 9\" />",
};
#[cfg(feature = "FiUserMinus")]
const FI_USER_MINUS: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2\" />\n<circle cx=\"8.5\" cy=\"7\" r=\"4\" />\n<line x1=\"23\" y1=\"11\" x2=\"17\" y2=\"11\" />",
};
#[cfg(feature = "FiUserPlus")]
const FI_USER_PLUS: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2\" />\n<circle cx=\"8.5\" cy=\"7\" r=\"4\" />\n<line x1=\"20\" y1=\"8\" x2=\"20\" y2=\"14\" />\n<line x1=\"23\" y1=\"11\" x2=\"17\" y2=\"11\" />",
};
#[cfg(feature = "FiUserX")]
const FI_USER_X: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2\" />\n<circle cx=\"8.5\" cy=\"7\" r=\"4\" />\n<line x1=\"18\" y1=\"8\" x2=\"23\" y2=\"13\" />\n<line x1=\"23\" y1=\"8\" x2=\"18\" y2=\"13\" />",
};
#[cfg(feature = "FiUsers")]
const FI_USERS: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2\" />\n<circle cx=\"9\" cy=\"7\" r=\"4\" />\n<path d=\"M23 21v-2a4 4 0 0 0-3-3.87\" />\n<path d=\"M16 3.13a4 4 0 0 1 0 7.75\" />",
};
#[cfg(feature = "FiVideo")]
const FI_VIDEO: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"23 7 16 12 23 17 23 7\" />\n<rect x=\"1\" y=\"5\" width=\"15\" height=\"14\" rx=\"2\" ry=\"2\" />",
};
#[cfg(feature = "FiVideoOff")]
const FI_VIDEO_OFF: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M16 16v1a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h2m5.66 0H14a2 2 0 0 1 2 2v3.34l1 1L23 7v10\" />\n<line x1=\"1\" y1=\"1\" x2=\"23\" y2=\"23\" />",
};
#[cfg(feature = "FiVoicemail")]
const FI_VOICEMAIL: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"5.5\" cy=\"11.5\" r=\"4.5\" />\n<circle cx=\"18.5\" cy=\"11.5\" r=\"4.5\" />\n<line x1=\"5.5\" y1=\"16\" x2=\"18.5\" y2=\"16\" />",
};
#[cfg(feature = "FiVolume")]
const FI_VOLUME: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"11 5 6 9 2 9 2 15 6 15 11 19 11 5\" />",
};
#[cfg(feature = "FiVolume1")]
const FI_VOLUME1: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"11 5 6 9 2 9 2 15 6 15 11 19 11 5\" />\n<path d=\"M15.54 8.46a5 5 0 0 1 0 7.07\" />",
};
#[cfg(feature = "FiVolume2")]
const FI_VOLUME2: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"11 5 6 9 2 9 2 15 6 15 11 19 11 5\" />\n<path d=\"M19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07\" />",
};
#[cfg(feature = "FiVolumeX")]
const FI_VOLUME_X: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"11 5 6 9 2 9 2 15 6 15 11 19 11 5\" />\n<line x1=\"23\" y1=\"9\" x2=\"17\" y2=\"15\" />\n<line x1=\"17\" y1=\"9\" x2=\"23\" y2=\"15\" />",
};
#[cfg(feature = "FiWatch")]
const FI_WATCH: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"7\" />\n<polyline points=\"12 9 12 12 13.5 13.5\" />\n<path d=\"M16.51 17.35l-.35 3.83a2 2 0 0 1-2 1.82H9.83a2 2 0 0 1-2-1.82l-.35-3.83m.01-10.7l.35-3.83A2 2 0 0 1 9.83 1h4.35a2 2 0 0 1 2 1.82l.35 3.83\" />",
};
#[cfg(feature = "FiWifi")]
const FI_WIFI: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M5 12.55a11 11 0 0 1 14.08 0\" />\n<path d=\"M1.42 9a16 16 0 0 1 21.16 0\" />\n<path d=\"M8.53 16.11a6 6 0 0 1 6.95 0\" />\n<line x1=\"12\" y1=\"20\" x2=\"12.01\" y2=\"20\" />",
};
#[cfg(feature = "FiWifiOff")]
const FI_WIFI_OFF: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"1\" y1=\"1\" x2=\"23\" y2=\"23\" />\n<path d=\"M16.72 11.06A10.94 10.94 0 0 1 19 12.55\" />\n<path d=\"M5 12.55a10.94 10.94 0 0 1 5.17-2.39\" />\n<path d=\"M10.71 5.05A16 16 0 0 1 22.58 9\" />\n<path d=\"M1.42 9a15.91 15.91 0 0 1 4.7-2.88\" />\n<path d=\"M8.53 16.11a6 6 0 0 1 6.95 0\" />\n<line x1=\"12\" y1=\"20\" x2=\"12.01\" y2=\"20\" />",
};
#[cfg(feature = "FiWind")]
const FI_WIND: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M9.59 4.59A2 2 0 1 1 11 8H2m10.59 11.41A2 2 0 1 0 14 16H2m15.73-8.27A2.5 2.5 0 1 1 19.5 12H2\" />",
};
#[cfg(feature = "FiX")]
const FI_X: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<line x1=\"18\" y1=\"6\" x2=\"6\" y2=\"18\" />\n<line x1=\"6\" y1=\"6\" x2=\"18\" y2=\"18\" />",
};
#[cfg(feature = "FiXCircle")]
const FI_X_CIRCLE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"12\" cy=\"12\" r=\"10\" />\n<line x1=\"15\" y1=\"9\" x2=\"9\" y2=\"15\" />\n<line x1=\"9\" y1=\"9\" x2=\"15\" y2=\"15\" />",
};
#[cfg(feature = "FiXOctagon")]
const FI_X_OCTAGON: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2\" />\n<line x1=\"15\" y1=\"9\" x2=\"9\" y2=\"15\" />\n<line x1=\"9\" y1=\"9\" x2=\"15\" y2=\"15\" />",
};
#[cfg(feature = "FiXSquare")]
const FI_X_SQUARE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<rect x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" />\n<line x1=\"9\" y1=\"9\" x2=\"15\" y2=\"15\" />\n<line x1=\"15\" y1=\"9\" x2=\"9\" y2=\"15\" />",
};
#[cfg(feature = "FiYoutube")]
const FI_YOUTUBE: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<path d=\"M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z\" />\n<polygon points=\"9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02\" />",
};
#[cfg(feature = "FiZap")]
const FI_ZAP: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polygon points=\"13 2 3 14 12 14 11 22 21 10 12 10 13 2\" />",
};
#[cfg(feature = "FiZapOff")]
const FI_ZAP_OFF: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<polyline points=\"12.41 6.75 13 2 10.57 4.92\" />\n<polyline points=\"18.57 12.91 21 10 15.66 10\" />\n<polyline points=\"8 8 3 14 12 14 11 22 16 16\" />\n<line x1=\"1\" y1=\"1\" x2=\"23\" y2=\"23\" />",
};
#[cfg(feature = "FiZoomIn")]
const FI_ZOOM_IN: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"11\" cy=\"11\" r=\"8\" />\n<line x1=\"21\" y1=\"21\" x2=\"16.65\" y2=\"16.65\" />\n<line x1=\"11\" y1=\"8\" x2=\"11\" y2=\"14\" />\n<line x1=\"8\" y1=\"11\" x2=\"14\" y2=\"11\" />",
};
#[cfg(feature = "FiZoomOut")]
const FI_ZOOM_OUT: icondata_core::Data = icondata_core::Data {
    style: None,
    x: None,
    y: None,
    width: Some("24"),
    height: Some("24"),
    view_box: Some("0 0 24 24"),
    stroke_linecap: Some("round"),
    stroke_linejoin: Some("round"),
    stroke_width: Some("2"),
    stroke: Some("currentColor"),
    fill: Some("none"),
    data: "<circle cx=\"11\" cy=\"11\" r=\"8\" />\n<line x1=\"21\" y1=\"21\" x2=\"16.65\" y2=\"16.65\" />\n<line x1=\"8\" y1=\"11\" x2=\"14\" y2=\"11\" />",
};
impl<'a> icondata_core::IconData<'a> for FiIcon {
    fn data(self) -> &'a icondata_core::Data {
        match self {
            #[cfg(feature = "FiActivity")]
            FiIcon::FiActivity => &FI_ACTIVITY,
            #[cfg(feature = "FiAirplay")]
            FiIcon::FiAirplay => &FI_AIRPLAY,
            #[cfg(feature = "FiAlertCircle")]
            FiIcon::FiAlertCircle => &FI_ALERT_CIRCLE,
            #[cfg(feature = "FiAlertOctagon")]
            FiIcon::FiAlertOctagon => &FI_ALERT_OCTAGON,
            #[cfg(feature = "FiAlertTriangle")]
            FiIcon::FiAlertTriangle => &FI_ALERT_TRIANGLE,
            #[cfg(feature = "FiAlignCenter")]
            FiIcon::FiAlignCenter => &FI_ALIGN_CENTER,
            #[cfg(feature = "FiAlignJustify")]
            FiIcon::FiAlignJustify => &FI_ALIGN_JUSTIFY,
            #[cfg(feature = "FiAlignLeft")]
            FiIcon::FiAlignLeft => &FI_ALIGN_LEFT,
            #[cfg(feature = "FiAlignRight")]
            FiIcon::FiAlignRight => &FI_ALIGN_RIGHT,
            #[cfg(feature = "FiAnchor")]
            FiIcon::FiAnchor => &FI_ANCHOR,
            #[cfg(feature = "FiAperture")]
            FiIcon::FiAperture => &FI_APERTURE,
            #[cfg(feature = "FiArchive")]
            FiIcon::FiArchive => &FI_ARCHIVE,
            #[cfg(feature = "FiArrowDown")]
            FiIcon::FiArrowDown => &FI_ARROW_DOWN,
            #[cfg(feature = "FiArrowDownCircle")]
            FiIcon::FiArrowDownCircle => &FI_ARROW_DOWN_CIRCLE,
            #[cfg(feature = "FiArrowDownLeft")]
            FiIcon::FiArrowDownLeft => &FI_ARROW_DOWN_LEFT,
            #[cfg(feature = "FiArrowDownRight")]
            FiIcon::FiArrowDownRight => &FI_ARROW_DOWN_RIGHT,
            #[cfg(feature = "FiArrowLeft")]
            FiIcon::FiArrowLeft => &FI_ARROW_LEFT,
            #[cfg(feature = "FiArrowLeftCircle")]
            FiIcon::FiArrowLeftCircle => &FI_ARROW_LEFT_CIRCLE,
            #[cfg(feature = "FiArrowRight")]
            FiIcon::FiArrowRight => &FI_ARROW_RIGHT,
            #[cfg(feature = "FiArrowRightCircle")]
            FiIcon::FiArrowRightCircle => &FI_ARROW_RIGHT_CIRCLE,
            #[cfg(feature = "FiArrowUp")]
            FiIcon::FiArrowUp => &FI_ARROW_UP,
            #[cfg(feature = "FiArrowUpCircle")]
            FiIcon::FiArrowUpCircle => &FI_ARROW_UP_CIRCLE,
            #[cfg(feature = "FiArrowUpLeft")]
            FiIcon::FiArrowUpLeft => &FI_ARROW_UP_LEFT,
            #[cfg(feature = "FiArrowUpRight")]
            FiIcon::FiArrowUpRight => &FI_ARROW_UP_RIGHT,
            #[cfg(feature = "FiAtSign")]
            FiIcon::FiAtSign => &FI_AT_SIGN,
            #[cfg(feature = "FiAward")]
            FiIcon::FiAward => &FI_AWARD,
            #[cfg(feature = "FiBarChart")]
            FiIcon::FiBarChart => &FI_BAR_CHART,
            #[cfg(feature = "FiBarChart2")]
            FiIcon::FiBarChart2 => &FI_BAR_CHART2,
            #[cfg(feature = "FiBattery")]
            FiIcon::FiBattery => &FI_BATTERY,
            #[cfg(feature = "FiBatteryCharging")]
            FiIcon::FiBatteryCharging => &FI_BATTERY_CHARGING,
            #[cfg(feature = "FiBell")]
            FiIcon::FiBell => &FI_BELL,
            #[cfg(feature = "FiBellOff")]
            FiIcon::FiBellOff => &FI_BELL_OFF,
            #[cfg(feature = "FiBluetooth")]
            FiIcon::FiBluetooth => &FI_BLUETOOTH,
            #[cfg(feature = "FiBold")]
            FiIcon::FiBold => &FI_BOLD,
            #[cfg(feature = "FiBook")]
            FiIcon::FiBook => &FI_BOOK,
            #[cfg(feature = "FiBookOpen")]
            FiIcon::FiBookOpen => &FI_BOOK_OPEN,
            #[cfg(feature = "FiBookmark")]
            FiIcon::FiBookmark => &FI_BOOKMARK,
            #[cfg(feature = "FiBox")]
            FiIcon::FiBox => &FI_BOX,
            #[cfg(feature = "FiBriefcase")]
            FiIcon::FiBriefcase => &FI_BRIEFCASE,
            #[cfg(feature = "FiCalendar")]
            FiIcon::FiCalendar => &FI_CALENDAR,
            #[cfg(feature = "FiCamera")]
            FiIcon::FiCamera => &FI_CAMERA,
            #[cfg(feature = "FiCameraOff")]
            FiIcon::FiCameraOff => &FI_CAMERA_OFF,
            #[cfg(feature = "FiCast")]
            FiIcon::FiCast => &FI_CAST,
            #[cfg(feature = "FiCheck")]
            FiIcon::FiCheck => &FI_CHECK,
            #[cfg(feature = "FiCheckCircle")]
            FiIcon::FiCheckCircle => &FI_CHECK_CIRCLE,
            #[cfg(feature = "FiCheckSquare")]
            FiIcon::FiCheckSquare => &FI_CHECK_SQUARE,
            #[cfg(feature = "FiChevronDown")]
            FiIcon::FiChevronDown => &FI_CHEVRON_DOWN,
            #[cfg(feature = "FiChevronLeft")]
            FiIcon::FiChevronLeft => &FI_CHEVRON_LEFT,
            #[cfg(feature = "FiChevronRight")]
            FiIcon::FiChevronRight => &FI_CHEVRON_RIGHT,
            #[cfg(feature = "FiChevronUp")]
            FiIcon::FiChevronUp => &FI_CHEVRON_UP,
            #[cfg(feature = "FiChevronsDown")]
            FiIcon::FiChevronsDown => &FI_CHEVRONS_DOWN,
            #[cfg(feature = "FiChevronsLeft")]
            FiIcon::FiChevronsLeft => &FI_CHEVRONS_LEFT,
            #[cfg(feature = "FiChevronsRight")]
            FiIcon::FiChevronsRight => &FI_CHEVRONS_RIGHT,
            #[cfg(feature = "FiChevronsUp")]
            FiIcon::FiChevronsUp => &FI_CHEVRONS_UP,
            #[cfg(feature = "FiChrome")]
            FiIcon::FiChrome => &FI_CHROME,
            #[cfg(feature = "FiCircle")]
            FiIcon::FiCircle => &FI_CIRCLE,
            #[cfg(feature = "FiClipboard")]
            FiIcon::FiClipboard => &FI_CLIPBOARD,
            #[cfg(feature = "FiClock")]
            FiIcon::FiClock => &FI_CLOCK,
            #[cfg(feature = "FiCloud")]
            FiIcon::FiCloud => &FI_CLOUD,
            #[cfg(feature = "FiCloudDrizzle")]
            FiIcon::FiCloudDrizzle => &FI_CLOUD_DRIZZLE,
            #[cfg(feature = "FiCloudLightning")]
            FiIcon::FiCloudLightning => &FI_CLOUD_LIGHTNING,
            #[cfg(feature = "FiCloudOff")]
            FiIcon::FiCloudOff => &FI_CLOUD_OFF,
            #[cfg(feature = "FiCloudRain")]
            FiIcon::FiCloudRain => &FI_CLOUD_RAIN,
            #[cfg(feature = "FiCloudSnow")]
            FiIcon::FiCloudSnow => &FI_CLOUD_SNOW,
            #[cfg(feature = "FiCode")]
            FiIcon::FiCode => &FI_CODE,
            #[cfg(feature = "FiCodepen")]
            FiIcon::FiCodepen => &FI_CODEPEN,
            #[cfg(feature = "FiCodesandbox")]
            FiIcon::FiCodesandbox => &FI_CODESANDBOX,
            #[cfg(feature = "FiCoffee")]
            FiIcon::FiCoffee => &FI_COFFEE,
            #[cfg(feature = "FiColumns")]
            FiIcon::FiColumns => &FI_COLUMNS,
            #[cfg(feature = "FiCommand")]
            FiIcon::FiCommand => &FI_COMMAND,
            #[cfg(feature = "FiCompass")]
            FiIcon::FiCompass => &FI_COMPASS,
            #[cfg(feature = "FiCopy")]
            FiIcon::FiCopy => &FI_COPY,
            #[cfg(feature = "FiCornerDownLeft")]
            FiIcon::FiCornerDownLeft => &FI_CORNER_DOWN_LEFT,
            #[cfg(feature = "FiCornerDownRight")]
            FiIcon::FiCornerDownRight => &FI_CORNER_DOWN_RIGHT,
            #[cfg(feature = "FiCornerLeftDown")]
            FiIcon::FiCornerLeftDown => &FI_CORNER_LEFT_DOWN,
            #[cfg(feature = "FiCornerLeftUp")]
            FiIcon::FiCornerLeftUp => &FI_CORNER_LEFT_UP,
            #[cfg(feature = "FiCornerRightDown")]
            FiIcon::FiCornerRightDown => &FI_CORNER_RIGHT_DOWN,
            #[cfg(feature = "FiCornerRightUp")]
            FiIcon::FiCornerRightUp => &FI_CORNER_RIGHT_UP,
            #[cfg(feature = "FiCornerUpLeft")]
            FiIcon::FiCornerUpLeft => &FI_CORNER_UP_LEFT,
            #[cfg(feature = "FiCornerUpRight")]
            FiIcon::FiCornerUpRight => &FI_CORNER_UP_RIGHT,
            #[cfg(feature = "FiCpu")]
            FiIcon::FiCpu => &FI_CPU,
            #[cfg(feature = "FiCreditCard")]
            FiIcon::FiCreditCard => &FI_CREDIT_CARD,
            #[cfg(feature = "FiCrop")]
            FiIcon::FiCrop => &FI_CROP,
            #[cfg(feature = "FiCrosshair")]
            FiIcon::FiCrosshair => &FI_CROSSHAIR,
            #[cfg(feature = "FiDatabase")]
            FiIcon::FiDatabase => &FI_DATABASE,
            #[cfg(feature = "FiDelete")]
            FiIcon::FiDelete => &FI_DELETE,
            #[cfg(feature = "FiDisc")]
            FiIcon::FiDisc => &FI_DISC,
            #[cfg(feature = "FiDivide")]
            FiIcon::FiDivide => &FI_DIVIDE,
            #[cfg(feature = "FiDivideCircle")]
            FiIcon::FiDivideCircle => &FI_DIVIDE_CIRCLE,
            #[cfg(feature = "FiDivideSquare")]
            FiIcon::FiDivideSquare => &FI_DIVIDE_SQUARE,
            #[cfg(feature = "FiDollarSign")]
            FiIcon::FiDollarSign => &FI_DOLLAR_SIGN,
            #[cfg(feature = "FiDownload")]
            FiIcon::FiDownload => &FI_DOWNLOAD,
            #[cfg(feature = "FiDownloadCloud")]
            FiIcon::FiDownloadCloud => &FI_DOWNLOAD_CLOUD,
            #[cfg(feature = "FiDribbble")]
            FiIcon::FiDribbble => &FI_DRIBBBLE,
            #[cfg(feature = "FiDroplet")]
            FiIcon::FiDroplet => &FI_DROPLET,
            #[cfg(feature = "FiEdit")]
            FiIcon::FiEdit => &FI_EDIT,
            #[cfg(feature = "FiEdit2")]
            FiIcon::FiEdit2 => &FI_EDIT2,
            #[cfg(feature = "FiEdit3")]
            FiIcon::FiEdit3 => &FI_EDIT3,
            #[cfg(feature = "FiExternalLink")]
            FiIcon::FiExternalLink => &FI_EXTERNAL_LINK,
            #[cfg(feature = "FiEye")]
            FiIcon::FiEye => &FI_EYE,
            #[cfg(feature = "FiEyeOff")]
            FiIcon::FiEyeOff => &FI_EYE_OFF,
            #[cfg(feature = "FiFacebook")]
            FiIcon::FiFacebook => &FI_FACEBOOK,
            #[cfg(feature = "FiFastForward")]
            FiIcon::FiFastForward => &FI_FAST_FORWARD,
            #[cfg(feature = "FiFeather")]
            FiIcon::FiFeather => &FI_FEATHER,
            #[cfg(feature = "FiFigma")]
            FiIcon::FiFigma => &FI_FIGMA,
            #[cfg(feature = "FiFile")]
            FiIcon::FiFile => &FI_FILE,
            #[cfg(feature = "FiFileMinus")]
            FiIcon::FiFileMinus => &FI_FILE_MINUS,
            #[cfg(feature = "FiFilePlus")]
            FiIcon::FiFilePlus => &FI_FILE_PLUS,
            #[cfg(feature = "FiFileText")]
            FiIcon::FiFileText => &FI_FILE_TEXT,
            #[cfg(feature = "FiFilm")]
            FiIcon::FiFilm => &FI_FILM,
            #[cfg(feature = "FiFilter")]
            FiIcon::FiFilter => &FI_FILTER,
            #[cfg(feature = "FiFlag")]
            FiIcon::FiFlag => &FI_FLAG,
            #[cfg(feature = "FiFolder")]
            FiIcon::FiFolder => &FI_FOLDER,
            #[cfg(feature = "FiFolderMinus")]
            FiIcon::FiFolderMinus => &FI_FOLDER_MINUS,
            #[cfg(feature = "FiFolderPlus")]
            FiIcon::FiFolderPlus => &FI_FOLDER_PLUS,
            #[cfg(feature = "FiFramer")]
            FiIcon::FiFramer => &FI_FRAMER,
            #[cfg(feature = "FiFrown")]
            FiIcon::FiFrown => &FI_FROWN,
            #[cfg(feature = "FiGift")]
            FiIcon::FiGift => &FI_GIFT,
            #[cfg(feature = "FiGitBranch")]
            FiIcon::FiGitBranch => &FI_GIT_BRANCH,
            #[cfg(feature = "FiGitCommit")]
            FiIcon::FiGitCommit => &FI_GIT_COMMIT,
            #[cfg(feature = "FiGitMerge")]
            FiIcon::FiGitMerge => &FI_GIT_MERGE,
            #[cfg(feature = "FiGitPullRequest")]
            FiIcon::FiGitPullRequest => &FI_GIT_PULL_REQUEST,
            #[cfg(feature = "FiGithub")]
            FiIcon::FiGithub => &FI_GITHUB,
            #[cfg(feature = "FiGitlab")]
            FiIcon::FiGitlab => &FI_GITLAB,
            #[cfg(feature = "FiGlobe")]
            FiIcon::FiGlobe => &FI_GLOBE,
            #[cfg(feature = "FiGrid")]
            FiIcon::FiGrid => &FI_GRID,
            #[cfg(feature = "FiHardDrive")]
            FiIcon::FiHardDrive => &FI_HARD_DRIVE,
            #[cfg(feature = "FiHash")]
            FiIcon::FiHash => &FI_HASH,
            #[cfg(feature = "FiHeadphones")]
            FiIcon::FiHeadphones => &FI_HEADPHONES,
            #[cfg(feature = "FiHeart")]
            FiIcon::FiHeart => &FI_HEART,
            #[cfg(feature = "FiHelpCircle")]
            FiIcon::FiHelpCircle => &FI_HELP_CIRCLE,
            #[cfg(feature = "FiHexagon")]
            FiIcon::FiHexagon => &FI_HEXAGON,
            #[cfg(feature = "FiHome")]
            FiIcon::FiHome => &FI_HOME,
            #[cfg(feature = "FiImage")]
            FiIcon::FiImage => &FI_IMAGE,
            #[cfg(feature = "FiInbox")]
            FiIcon::FiInbox => &FI_INBOX,
            #[cfg(feature = "FiInfo")]
            FiIcon::FiInfo => &FI_INFO,
            #[cfg(feature = "FiInstagram")]
            FiIcon::FiInstagram => &FI_INSTAGRAM,
            #[cfg(feature = "FiItalic")]
            FiIcon::FiItalic => &FI_ITALIC,
            #[cfg(feature = "FiKey")]
            FiIcon::FiKey => &FI_KEY,
            #[cfg(feature = "FiLayers")]
            FiIcon::FiLayers => &FI_LAYERS,
            #[cfg(feature = "FiLayout")]
            FiIcon::FiLayout => &FI_LAYOUT,
            #[cfg(feature = "FiLifeBuoy")]
            FiIcon::FiLifeBuoy => &FI_LIFE_BUOY,
            #[cfg(feature = "FiLink")]
            FiIcon::FiLink => &FI_LINK,
            #[cfg(feature = "FiLink2")]
            FiIcon::FiLink2 => &FI_LINK2,
            #[cfg(feature = "FiLinkedin")]
            FiIcon::FiLinkedin => &FI_LINKEDIN,
            #[cfg(feature = "FiList")]
            FiIcon::FiList => &FI_LIST,
            #[cfg(feature = "FiLoader")]
            FiIcon::FiLoader => &FI_LOADER,
            #[cfg(feature = "FiLock")]
            FiIcon::FiLock => &FI_LOCK,
            #[cfg(feature = "FiLogIn")]
            FiIcon::FiLogIn => &FI_LOG_IN,
            #[cfg(feature = "FiLogOut")]
            FiIcon::FiLogOut => &FI_LOG_OUT,
            #[cfg(feature = "FiMail")]
            FiIcon::FiMail => &FI_MAIL,
            #[cfg(feature = "FiMap")]
            FiIcon::FiMap => &FI_MAP,
            #[cfg(feature = "FiMapPin")]
            FiIcon::FiMapPin => &FI_MAP_PIN,
            #[cfg(feature = "FiMaximize")]
            FiIcon::FiMaximize => &FI_MAXIMIZE,
            #[cfg(feature = "FiMaximize2")]
            FiIcon::FiMaximize2 => &FI_MAXIMIZE2,
            #[cfg(feature = "FiMeh")]
            FiIcon::FiMeh => &FI_MEH,
            #[cfg(feature = "FiMenu")]
            FiIcon::FiMenu => &FI_MENU,
            #[cfg(feature = "FiMessageCircle")]
            FiIcon::FiMessageCircle => &FI_MESSAGE_CIRCLE,
            #[cfg(feature = "FiMessageSquare")]
            FiIcon::FiMessageSquare => &FI_MESSAGE_SQUARE,
            #[cfg(feature = "FiMic")]
            FiIcon::FiMic => &FI_MIC,
            #[cfg(feature = "FiMicOff")]
            FiIcon::FiMicOff => &FI_MIC_OFF,
            #[cfg(feature = "FiMinimize")]
            FiIcon::FiMinimize => &FI_MINIMIZE,
            #[cfg(feature = "FiMinimize2")]
            FiIcon::FiMinimize2 => &FI_MINIMIZE2,
            #[cfg(feature = "FiMinus")]
            FiIcon::FiMinus => &FI_MINUS,
            #[cfg(feature = "FiMinusCircle")]
            FiIcon::FiMinusCircle => &FI_MINUS_CIRCLE,
            #[cfg(feature = "FiMinusSquare")]
            FiIcon::FiMinusSquare => &FI_MINUS_SQUARE,
            #[cfg(feature = "FiMonitor")]
            FiIcon::FiMonitor => &FI_MONITOR,
            #[cfg(feature = "FiMoon")]
            FiIcon::FiMoon => &FI_MOON,
            #[cfg(feature = "FiMoreHorizontal")]
            FiIcon::FiMoreHorizontal => &FI_MORE_HORIZONTAL,
            #[cfg(feature = "FiMoreVertical")]
            FiIcon::FiMoreVertical => &FI_MORE_VERTICAL,
            #[cfg(feature = "FiMousePointer")]
            FiIcon::FiMousePointer => &FI_MOUSE_POINTER,
            #[cfg(feature = "FiMove")]
            FiIcon::FiMove => &FI_MOVE,
            #[cfg(feature = "FiMusic")]
            FiIcon::FiMusic => &FI_MUSIC,
            #[cfg(feature = "FiNavigation")]
            FiIcon::FiNavigation => &FI_NAVIGATION,
            #[cfg(feature = "FiNavigation2")]
            FiIcon::FiNavigation2 => &FI_NAVIGATION2,
            #[cfg(feature = "FiOctagon")]
            FiIcon::FiOctagon => &FI_OCTAGON,
            #[cfg(feature = "FiPackage")]
            FiIcon::FiPackage => &FI_PACKAGE,
            #[cfg(feature = "FiPaperclip")]
            FiIcon::FiPaperclip => &FI_PAPERCLIP,
            #[cfg(feature = "FiPause")]
            FiIcon::FiPause => &FI_PAUSE,
            #[cfg(feature = "FiPauseCircle")]
            FiIcon::FiPauseCircle => &FI_PAUSE_CIRCLE,
            #[cfg(feature = "FiPenTool")]
            FiIcon::FiPenTool => &FI_PEN_TOOL,
            #[cfg(feature = "FiPercent")]
            FiIcon::FiPercent => &FI_PERCENT,
            #[cfg(feature = "FiPhone")]
            FiIcon::FiPhone => &FI_PHONE,
            #[cfg(feature = "FiPhoneCall")]
            FiIcon::FiPhoneCall => &FI_PHONE_CALL,
            #[cfg(feature = "FiPhoneForwarded")]
            FiIcon::FiPhoneForwarded => &FI_PHONE_FORWARDED,
            #[cfg(feature = "FiPhoneIncoming")]
            FiIcon::FiPhoneIncoming => &FI_PHONE_INCOMING,
            #[cfg(feature = "FiPhoneMissed")]
            FiIcon::FiPhoneMissed => &FI_PHONE_MISSED,
            #[cfg(feature = "FiPhoneOff")]
            FiIcon::FiPhoneOff => &FI_PHONE_OFF,
            #[cfg(feature = "FiPhoneOutgoing")]
            FiIcon::FiPhoneOutgoing => &FI_PHONE_OUTGOING,
            #[cfg(feature = "FiPieChart")]
            FiIcon::FiPieChart => &FI_PIE_CHART,
            #[cfg(feature = "FiPlay")]
            FiIcon::FiPlay => &FI_PLAY,
            #[cfg(feature = "FiPlayCircle")]
            FiIcon::FiPlayCircle => &FI_PLAY_CIRCLE,
            #[cfg(feature = "FiPlus")]
            FiIcon::FiPlus => &FI_PLUS,
            #[cfg(feature = "FiPlusCircle")]
            FiIcon::FiPlusCircle => &FI_PLUS_CIRCLE,
            #[cfg(feature = "FiPlusSquare")]
            FiIcon::FiPlusSquare => &FI_PLUS_SQUARE,
            #[cfg(feature = "FiPocket")]
            FiIcon::FiPocket => &FI_POCKET,
            #[cfg(feature = "FiPower")]
            FiIcon::FiPower => &FI_POWER,
            #[cfg(feature = "FiPrinter")]
            FiIcon::FiPrinter => &FI_PRINTER,
            #[cfg(feature = "FiRadio")]
            FiIcon::FiRadio => &FI_RADIO,
            #[cfg(feature = "FiRefreshCcw")]
            FiIcon::FiRefreshCcw => &FI_REFRESH_CCW,
            #[cfg(feature = "FiRefreshCw")]
            FiIcon::FiRefreshCw => &FI_REFRESH_CW,
            #[cfg(feature = "FiRepeat")]
            FiIcon::FiRepeat => &FI_REPEAT,
            #[cfg(feature = "FiRewind")]
            FiIcon::FiRewind => &FI_REWIND,
            #[cfg(feature = "FiRotateCcw")]
            FiIcon::FiRotateCcw => &FI_ROTATE_CCW,
            #[cfg(feature = "FiRotateCw")]
            FiIcon::FiRotateCw => &FI_ROTATE_CW,
            #[cfg(feature = "FiRss")]
            FiIcon::FiRss => &FI_RSS,
            #[cfg(feature = "FiSave")]
            FiIcon::FiSave => &FI_SAVE,
            #[cfg(feature = "FiScissors")]
            FiIcon::FiScissors => &FI_SCISSORS,
            #[cfg(feature = "FiSearch")]
            FiIcon::FiSearch => &FI_SEARCH,
            #[cfg(feature = "FiSend")]
            FiIcon::FiSend => &FI_SEND,
            #[cfg(feature = "FiServer")]
            FiIcon::FiServer => &FI_SERVER,
            #[cfg(feature = "FiSettings")]
            FiIcon::FiSettings => &FI_SETTINGS,
            #[cfg(feature = "FiShare")]
            FiIcon::FiShare => &FI_SHARE,
            #[cfg(feature = "FiShare2")]
            FiIcon::FiShare2 => &FI_SHARE2,
            #[cfg(feature = "FiShield")]
            FiIcon::FiShield => &FI_SHIELD,
            #[cfg(feature = "FiShieldOff")]
            FiIcon::FiShieldOff => &FI_SHIELD_OFF,
            #[cfg(feature = "FiShoppingBag")]
            FiIcon::FiShoppingBag => &FI_SHOPPING_BAG,
            #[cfg(feature = "FiShoppingCart")]
            FiIcon::FiShoppingCart => &FI_SHOPPING_CART,
            #[cfg(feature = "FiShuffle")]
            FiIcon::FiShuffle => &FI_SHUFFLE,
            #[cfg(feature = "FiSidebar")]
            FiIcon::FiSidebar => &FI_SIDEBAR,
            #[cfg(feature = "FiSkipBack")]
            FiIcon::FiSkipBack => &FI_SKIP_BACK,
            #[cfg(feature = "FiSkipForward")]
            FiIcon::FiSkipForward => &FI_SKIP_FORWARD,
            #[cfg(feature = "FiSlack")]
            FiIcon::FiSlack => &FI_SLACK,
            #[cfg(feature = "FiSlash")]
            FiIcon::FiSlash => &FI_SLASH,
            #[cfg(feature = "FiSliders")]
            FiIcon::FiSliders => &FI_SLIDERS,
            #[cfg(feature = "FiSmartphone")]
            FiIcon::FiSmartphone => &FI_SMARTPHONE,
            #[cfg(feature = "FiSmile")]
            FiIcon::FiSmile => &FI_SMILE,
            #[cfg(feature = "FiSpeaker")]
            FiIcon::FiSpeaker => &FI_SPEAKER,
            #[cfg(feature = "FiSquare")]
            FiIcon::FiSquare => &FI_SQUARE,
            #[cfg(feature = "FiStar")]
            FiIcon::FiStar => &FI_STAR,
            #[cfg(feature = "FiStopCircle")]
            FiIcon::FiStopCircle => &FI_STOP_CIRCLE,
            #[cfg(feature = "FiSun")]
            FiIcon::FiSun => &FI_SUN,
            #[cfg(feature = "FiSunrise")]
            FiIcon::FiSunrise => &FI_SUNRISE,
            #[cfg(feature = "FiSunset")]
            FiIcon::FiSunset => &FI_SUNSET,
            #[cfg(feature = "FiTable")]
            FiIcon::FiTable => &FI_TABLE,
            #[cfg(feature = "FiTablet")]
            FiIcon::FiTablet => &FI_TABLET,
            #[cfg(feature = "FiTag")]
            FiIcon::FiTag => &FI_TAG,
            #[cfg(feature = "FiTarget")]
            FiIcon::FiTarget => &FI_TARGET,
            #[cfg(feature = "FiTerminal")]
            FiIcon::FiTerminal => &FI_TERMINAL,
            #[cfg(feature = "FiThermometer")]
            FiIcon::FiThermometer => &FI_THERMOMETER,
            #[cfg(feature = "FiThumbsDown")]
            FiIcon::FiThumbsDown => &FI_THUMBS_DOWN,
            #[cfg(feature = "FiThumbsUp")]
            FiIcon::FiThumbsUp => &FI_THUMBS_UP,
            #[cfg(feature = "FiToggleLeft")]
            FiIcon::FiToggleLeft => &FI_TOGGLE_LEFT,
            #[cfg(feature = "FiToggleRight")]
            FiIcon::FiToggleRight => &FI_TOGGLE_RIGHT,
            #[cfg(feature = "FiTool")]
            FiIcon::FiTool => &FI_TOOL,
            #[cfg(feature = "FiTrash")]
            FiIcon::FiTrash => &FI_TRASH,
            #[cfg(feature = "FiTrash2")]
            FiIcon::FiTrash2 => &FI_TRASH2,
            #[cfg(feature = "FiTrello")]
            FiIcon::FiTrello => &FI_TRELLO,
            #[cfg(feature = "FiTrendingDown")]
            FiIcon::FiTrendingDown => &FI_TRENDING_DOWN,
            #[cfg(feature = "FiTrendingUp")]
            FiIcon::FiTrendingUp => &FI_TRENDING_UP,
            #[cfg(feature = "FiTriangle")]
            FiIcon::FiTriangle => &FI_TRIANGLE,
            #[cfg(feature = "FiTruck")]
            FiIcon::FiTruck => &FI_TRUCK,
            #[cfg(feature = "FiTv")]
            FiIcon::FiTv => &FI_TV,
            #[cfg(feature = "FiTwitch")]
            FiIcon::FiTwitch => &FI_TWITCH,
            #[cfg(feature = "FiTwitter")]
            FiIcon::FiTwitter => &FI_TWITTER,
            #[cfg(feature = "FiType")]
            FiIcon::FiType => &FI_TYPE,
            #[cfg(feature = "FiUmbrella")]
            FiIcon::FiUmbrella => &FI_UMBRELLA,
            #[cfg(feature = "FiUnderline")]
            FiIcon::FiUnderline => &FI_UNDERLINE,
            #[cfg(feature = "FiUnlock")]
            FiIcon::FiUnlock => &FI_UNLOCK,
            #[cfg(feature = "FiUpload")]
            FiIcon::FiUpload => &FI_UPLOAD,
            #[cfg(feature = "FiUploadCloud")]
            FiIcon::FiUploadCloud => &FI_UPLOAD_CLOUD,
            #[cfg(feature = "FiUser")]
            FiIcon::FiUser => &FI_USER,
            #[cfg(feature = "FiUserCheck")]
            FiIcon::FiUserCheck => &FI_USER_CHECK,
            #[cfg(feature = "FiUserMinus")]
            FiIcon::FiUserMinus => &FI_USER_MINUS,
            #[cfg(feature = "FiUserPlus")]
            FiIcon::FiUserPlus => &FI_USER_PLUS,
            #[cfg(feature = "FiUserX")]
            FiIcon::FiUserX => &FI_USER_X,
            #[cfg(feature = "FiUsers")]
            FiIcon::FiUsers => &FI_USERS,
            #[cfg(feature = "FiVideo")]
            FiIcon::FiVideo => &FI_VIDEO,
            #[cfg(feature = "FiVideoOff")]
            FiIcon::FiVideoOff => &FI_VIDEO_OFF,
            #[cfg(feature = "FiVoicemail")]
            FiIcon::FiVoicemail => &FI_VOICEMAIL,
            #[cfg(feature = "FiVolume")]
            FiIcon::FiVolume => &FI_VOLUME,
            #[cfg(feature = "FiVolume1")]
            FiIcon::FiVolume1 => &FI_VOLUME1,
            #[cfg(feature = "FiVolume2")]
            FiIcon::FiVolume2 => &FI_VOLUME2,
            #[cfg(feature = "FiVolumeX")]
            FiIcon::FiVolumeX => &FI_VOLUME_X,
            #[cfg(feature = "FiWatch")]
            FiIcon::FiWatch => &FI_WATCH,
            #[cfg(feature = "FiWifi")]
            FiIcon::FiWifi => &FI_WIFI,
            #[cfg(feature = "FiWifiOff")]
            FiIcon::FiWifiOff => &FI_WIFI_OFF,
            #[cfg(feature = "FiWind")]
            FiIcon::FiWind => &FI_WIND,
            #[cfg(feature = "FiX")]
            FiIcon::FiX => &FI_X,
            #[cfg(feature = "FiXCircle")]
            FiIcon::FiXCircle => &FI_X_CIRCLE,
            #[cfg(feature = "FiXOctagon")]
            FiIcon::FiXOctagon => &FI_X_OCTAGON,
            #[cfg(feature = "FiXSquare")]
            FiIcon::FiXSquare => &FI_X_SQUARE,
            #[cfg(feature = "FiYoutube")]
            FiIcon::FiYoutube => &FI_YOUTUBE,
            #[cfg(feature = "FiZap")]
            FiIcon::FiZap => &FI_ZAP,
            #[cfg(feature = "FiZapOff")]
            FiIcon::FiZapOff => &FI_ZAP_OFF,
            #[cfg(feature = "FiZoomIn")]
            FiIcon::FiZoomIn => &FI_ZOOM_IN,
            #[cfg(feature = "FiZoomOut")]
            FiIcon::FiZoomOut => &FI_ZOOM_OUT,
        }
    }
}
