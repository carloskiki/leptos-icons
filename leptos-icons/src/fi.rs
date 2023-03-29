use leptos::*;

#[cfg(feature = "FiActivity")]
///This icon requires the feature `FiActivity` to be enabled.
#[component]
pub fn FiActivity(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"22 12 18 12 15 21 9 3 6 12 2 12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiAirplay")]
///This icon requires the feature `FiAirplay` to be enabled.
#[component]
pub fn FiAirplay(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M5 17H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v10a2 2 0 0 1-2 2h-1\" /><polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"12 15 17 21 7 21 12 15\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiAlertCircle")]
///This icon requires the feature `FiAlertCircle` to be enabled.
#[component]
pub fn FiAlertCircle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"8\" x2=\"12\" y2=\"12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"16\" x2=\"12.01\" y2=\"16\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiAlertOctagon")]
///This icon requires the feature `FiAlertOctagon` to be enabled.
#[component]
pub fn FiAlertOctagon(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"8\" x2=\"12\" y2=\"12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"16\" x2=\"12.01\" y2=\"16\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiAlertTriangle")]
///This icon requires the feature `FiAlertTriangle` to be enabled.
#[component]
pub fn FiAlertTriangle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"9\" x2=\"12\" y2=\"13\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"17\" x2=\"12.01\" y2=\"17\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiAlignCenter")]
///This icon requires the feature `FiAlignCenter` to be enabled.
#[component]
pub fn FiAlignCenter(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"18\" y1=\"10\" x2=\"6\" y2=\"10\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"21\" y1=\"6\" x2=\"3\" y2=\"6\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"21\" y1=\"14\" x2=\"3\" y2=\"14\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"18\" y1=\"18\" x2=\"6\" y2=\"18\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiAlignJustify")]
///This icon requires the feature `FiAlignJustify` to be enabled.
#[component]
pub fn FiAlignJustify(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"21\" y1=\"10\" x2=\"3\" y2=\"10\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"21\" y1=\"6\" x2=\"3\" y2=\"6\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"21\" y1=\"14\" x2=\"3\" y2=\"14\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"21\" y1=\"18\" x2=\"3\" y2=\"18\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiAlignLeft")]
///This icon requires the feature `FiAlignLeft` to be enabled.
#[component]
pub fn FiAlignLeft(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"17\" y1=\"10\" x2=\"3\" y2=\"10\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"21\" y1=\"6\" x2=\"3\" y2=\"6\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"21\" y1=\"14\" x2=\"3\" y2=\"14\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"17\" y1=\"18\" x2=\"3\" y2=\"18\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiAlignRight")]
///This icon requires the feature `FiAlignRight` to be enabled.
#[component]
pub fn FiAlignRight(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"21\" y1=\"10\" x2=\"7\" y2=\"10\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"21\" y1=\"6\" x2=\"3\" y2=\"6\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"21\" y1=\"14\" x2=\"3\" y2=\"14\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"21\" y1=\"18\" x2=\"7\" y2=\"18\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiAnchor")]
///This icon requires the feature `FiAnchor` to be enabled.
#[component]
pub fn FiAnchor(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"5\" r=\"3\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"22\" x2=\"12\" y2=\"8\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M5 12H2a10 10 0 0 0 20 0h-3\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiAperture")]
///This icon requires the feature `FiAperture` to be enabled.
#[component]
pub fn FiAperture(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"14.31\" y1=\"8\" x2=\"20.05\" y2=\"17.94\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"9.69\" y1=\"8\" x2=\"21.17\" y2=\"8\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"7.38\" y1=\"12\" x2=\"13.12\" y2=\"2.06\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"9.69\" y1=\"16\" x2=\"3.95\" y2=\"6.06\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"14.31\" y1=\"16\" x2=\"2.83\" y2=\"16\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"16.62\" y1=\"12\" x2=\"10.88\" y2=\"21.94\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiArchive")]
///This icon requires the feature `FiArchive` to be enabled.
#[component]
pub fn FiArchive(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"21 8 21 21 3 21 3 8\" /><rect xmlns=\"http://www.w3.org/2000/svg\" x=\"1\" y=\"3\" width=\"22\" height=\"5\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"10\" y1=\"12\" x2=\"14\" y2=\"12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiArrowDown")]
///This icon requires the feature `FiArrowDown` to be enabled.
#[component]
pub fn FiArrowDown(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"5\" x2=\"12\" y2=\"19\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"19 12 12 19 5 12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiArrowDownCircle")]
///This icon requires the feature `FiArrowDownCircle` to be enabled.
#[component]
pub fn FiArrowDownCircle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"8 12 12 16 16 12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"8\" x2=\"12\" y2=\"16\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiArrowDownLeft")]
///This icon requires the feature `FiArrowDownLeft` to be enabled.
#[component]
pub fn FiArrowDownLeft(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"17\" y1=\"7\" x2=\"7\" y2=\"17\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"17 17 7 17 7 7\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiArrowDownRight")]
///This icon requires the feature `FiArrowDownRight` to be enabled.
#[component]
pub fn FiArrowDownRight(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"7\" y1=\"7\" x2=\"17\" y2=\"17\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"17 7 17 17 7 17\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiArrowLeft")]
///This icon requires the feature `FiArrowLeft` to be enabled.
#[component]
pub fn FiArrowLeft(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"19\" y1=\"12\" x2=\"5\" y2=\"12\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"12 19 5 12 12 5\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiArrowLeftCircle")]
///This icon requires the feature `FiArrowLeftCircle` to be enabled.
#[component]
pub fn FiArrowLeftCircle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"12 8 8 12 12 16\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"16\" y1=\"12\" x2=\"8\" y2=\"12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiArrowRight")]
///This icon requires the feature `FiArrowRight` to be enabled.
#[component]
pub fn FiArrowRight(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"5\" y1=\"12\" x2=\"19\" y2=\"12\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"12 5 19 12 12 19\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiArrowRightCircle")]
///This icon requires the feature `FiArrowRightCircle` to be enabled.
#[component]
pub fn FiArrowRightCircle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"12 16 16 12 12 8\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"12\" x2=\"16\" y2=\"12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiArrowUp")]
///This icon requires the feature `FiArrowUp` to be enabled.
#[component]
pub fn FiArrowUp(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"19\" x2=\"12\" y2=\"5\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"5 12 12 5 19 12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiArrowUpCircle")]
///This icon requires the feature `FiArrowUpCircle` to be enabled.
#[component]
pub fn FiArrowUpCircle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"16 12 12 8 8 12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"16\" x2=\"12\" y2=\"8\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiArrowUpLeft")]
///This icon requires the feature `FiArrowUpLeft` to be enabled.
#[component]
pub fn FiArrowUpLeft(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"17\" y1=\"17\" x2=\"7\" y2=\"7\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"7 17 7 7 17 7\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiArrowUpRight")]
///This icon requires the feature `FiArrowUpRight` to be enabled.
#[component]
pub fn FiArrowUpRight(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"7\" y1=\"17\" x2=\"17\" y2=\"7\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"7 7 17 7 17 17\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiAtSign")]
///This icon requires the feature `FiAtSign` to be enabled.
#[component]
pub fn FiAtSign(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"4\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M16 8v5a3 3 0 0 0 6 0v-1a10 10 0 1 0-3.92 7.94\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiAward")]
///This icon requires the feature `FiAward` to be enabled.
#[component]
pub fn FiAward(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"8\" r=\"7\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"8.21 13.89 7 23 12 20 17 23 15.79 13.88\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiBarChart")]
///This icon requires the feature `FiBarChart` to be enabled.
#[component]
pub fn FiBarChart(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"20\" x2=\"12\" y2=\"10\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"18\" y1=\"20\" x2=\"18\" y2=\"4\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"6\" y1=\"20\" x2=\"6\" y2=\"16\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiBarChart2")]
///This icon requires the feature `FiBarChart2` to be enabled.
#[component]
pub fn FiBarChart2(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"18\" y1=\"20\" x2=\"18\" y2=\"10\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"20\" x2=\"12\" y2=\"4\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"6\" y1=\"20\" x2=\"6\" y2=\"14\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiBattery")]
///This icon requires the feature `FiBattery` to be enabled.
#[component]
pub fn FiBattery(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"1\" y=\"6\" width=\"18\" height=\"12\" rx=\"2\" ry=\"2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"23\" y1=\"13\" x2=\"23\" y2=\"11\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiBatteryCharging")]
///This icon requires the feature `FiBatteryCharging` to be enabled.
#[component]
pub fn FiBatteryCharging(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M5 18H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3.19M15 6h2a2 2 0 0 1 2 2v8a2 2 0 0 1-2 2h-3.19\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"23\" y1=\"13\" x2=\"23\" y2=\"11\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"11 6 7 12 13 12 9 18\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiBell")]
///This icon requires the feature `FiBell` to be enabled.
#[component]
pub fn FiBell(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M18 8A6 6 0 0 0 6 8c0 7-3 9-3 9h18s-3-2-3-9\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M13.73 21a2 2 0 0 1-3.46 0\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiBellOff")]
///This icon requires the feature `FiBellOff` to be enabled.
#[component]
pub fn FiBellOff(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M13.73 21a2 2 0 0 1-3.46 0\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M18.63 13A17.89 17.89 0 0 1 18 8\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M6.26 6.26A5.86 5.86 0 0 0 6 8c0 7-3 9-3 9h14\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M18 8a6 6 0 0 0-9.33-5\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"1\" y1=\"1\" x2=\"23\" y2=\"23\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiBluetooth")]
///This icon requires the feature `FiBluetooth` to be enabled.
#[component]
pub fn FiBluetooth(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"6.5 6.5 17.5 17.5 12 23 12 1 17.5 6.5 6.5 17.5\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiBold")]
///This icon requires the feature `FiBold` to be enabled.
#[component]
pub fn FiBold(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M6 4h8a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M6 12h9a4 4 0 0 1 4 4 4 4 0 0 1-4 4H6z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiBook")]
///This icon requires the feature `FiBook` to be enabled.
#[component]
pub fn FiBook(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M4 19.5A2.5 2.5 0 0 1 6.5 17H20\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M6.5 2H20v20H6.5A2.5 2.5 0 0 1 4 19.5v-15A2.5 2.5 0 0 1 6.5 2z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiBookOpen")]
///This icon requires the feature `FiBookOpen` to be enabled.
#[component]
pub fn FiBookOpen(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M2 3h6a4 4 0 0 1 4 4v14a3 3 0 0 0-3-3H2z\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M22 3h-6a4 4 0 0 0-4 4v14a3 3 0 0 1 3-3h7z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiBookmark")]
///This icon requires the feature `FiBookmark` to be enabled.
#[component]
pub fn FiBookmark(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M19 21l-7-5-7 5V5a2 2 0 0 1 2-2h10a2 2 0 0 1 2 2z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiBox")]
///This icon requires the feature `FiBox` to be enabled.
#[component]
pub fn FiBox(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"3.27 6.96 12 12.01 20.73 6.96\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"22.08\" x2=\"12\" y2=\"12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiBriefcase")]
///This icon requires the feature `FiBriefcase` to be enabled.
#[component]
pub fn FiBriefcase(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"2\" y=\"7\" width=\"20\" height=\"14\" rx=\"2\" ry=\"2\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M16 21V5a2 2 0 0 0-2-2h-4a2 2 0 0 0-2 2v16\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCalendar")]
///This icon requires the feature `FiCalendar` to be enabled.
#[component]
pub fn FiCalendar(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"3\" y=\"4\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"16\" y1=\"2\" x2=\"16\" y2=\"6\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"2\" x2=\"8\" y2=\"6\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"3\" y1=\"10\" x2=\"21\" y2=\"10\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCamera")]
///This icon requires the feature `FiCamera` to be enabled.
#[component]
pub fn FiCamera(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M23 19a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h4l2-3h6l2 3h4a2 2 0 0 1 2 2z\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"13\" r=\"4\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCameraOff")]
///This icon requires the feature `FiCameraOff` to be enabled.
#[component]
pub fn FiCameraOff(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"1\" y1=\"1\" x2=\"23\" y2=\"23\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M21 21H3a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h3m3-3h6l2 3h4a2 2 0 0 1 2 2v9.34m-7.72-2.06a4 4 0 1 1-5.56-5.56\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCast")]
///This icon requires the feature `FiCast` to be enabled.
#[component]
pub fn FiCast(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M2 16.1A5 5 0 0 1 5.9 20M2 12.05A9 9 0 0 1 9.95 20M2 8V6a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v12a2 2 0 0 1-2 2h-6\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"2\" y1=\"20\" x2=\"2.01\" y2=\"20\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCheck")]
///This icon requires the feature `FiCheck` to be enabled.
#[component]
pub fn FiCheck(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"20 6 9 17 4 12\" />" > <
        title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCheckCircle")]
///This icon requires the feature `FiCheckCircle` to be enabled.
#[component]
pub fn FiCheckCircle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M22 11.08V12a10 10 0 1 1-5.93-9.14\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"22 4 12 14.01 9 11.01\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCheckSquare")]
///This icon requires the feature `FiCheckSquare` to be enabled.
#[component]
pub fn FiCheckSquare(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"9 11 12 14 22 4\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M21 12v7a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiChevronDown")]
///This icon requires the feature `FiChevronDown` to be enabled.
#[component]
pub fn FiChevronDown(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"6 9 12 15 18 9\" />" > <
        title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiChevronLeft")]
///This icon requires the feature `FiChevronLeft` to be enabled.
#[component]
pub fn FiChevronLeft(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"15 18 9 12 15 6\" />" >
        < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiChevronRight")]
///This icon requires the feature `FiChevronRight` to be enabled.
#[component]
pub fn FiChevronRight(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"9 18 15 12 9 6\" />" > <
        title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiChevronUp")]
///This icon requires the feature `FiChevronUp` to be enabled.
#[component]
pub fn FiChevronUp(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"18 15 12 9 6 15\" />" >
        < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiChevronsDown")]
///This icon requires the feature `FiChevronsDown` to be enabled.
#[component]
pub fn FiChevronsDown(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"7 13 12 18 17 13\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"7 6 12 11 17 6\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiChevronsLeft")]
///This icon requires the feature `FiChevronsLeft` to be enabled.
#[component]
pub fn FiChevronsLeft(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"11 17 6 12 11 7\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"18 17 13 12 18 7\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiChevronsRight")]
///This icon requires the feature `FiChevronsRight` to be enabled.
#[component]
pub fn FiChevronsRight(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"13 17 18 12 13 7\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"6 17 11 12 6 7\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiChevronsUp")]
///This icon requires the feature `FiChevronsUp` to be enabled.
#[component]
pub fn FiChevronsUp(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"17 11 12 6 7 11\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"17 18 12 13 7 18\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiChrome")]
///This icon requires the feature `FiChrome` to be enabled.
#[component]
pub fn FiChrome(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"4\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"21.17\" y1=\"8\" x2=\"12\" y2=\"8\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"3.95\" y1=\"6.06\" x2=\"8.54\" y2=\"14\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"10.88\" y1=\"21.94\" x2=\"15.46\" y2=\"14\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCircle")]
///This icon requires the feature `FiCircle` to be enabled.
#[component]
pub fn FiCircle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" />" >
        < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiClipboard")]
///This icon requires the feature `FiClipboard` to be enabled.
#[component]
pub fn FiClipboard(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M16 4h2a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H6a2 2 0 0 1-2-2V6a2 2 0 0 1 2-2h2\" /><rect xmlns=\"http://www.w3.org/2000/svg\" x=\"8\" y=\"2\" width=\"8\" height=\"4\" rx=\"1\" ry=\"1\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiClock")]
///This icon requires the feature `FiClock` to be enabled.
#[component]
pub fn FiClock(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"12 6 12 12 16 14\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCloud")]
///This icon requires the feature `FiCloud` to be enabled.
#[component]
pub fn FiCloud(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M18 10h-1.26A8 8 0 1 0 9 20h9a5 5 0 0 0 0-10z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCloudDrizzle")]
///This icon requires the feature `FiCloudDrizzle` to be enabled.
#[component]
pub fn FiCloudDrizzle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"19\" x2=\"8\" y2=\"21\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"13\" x2=\"8\" y2=\"15\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"16\" y1=\"19\" x2=\"16\" y2=\"21\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"16\" y1=\"13\" x2=\"16\" y2=\"15\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"21\" x2=\"12\" y2=\"23\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"15\" x2=\"12\" y2=\"17\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M20 16.58A5 5 0 0 0 18 7h-1.26A8 8 0 1 0 4 15.25\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCloudLightning")]
///This icon requires the feature `FiCloudLightning` to be enabled.
#[component]
pub fn FiCloudLightning(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M19 16.9A5 5 0 0 0 18 7h-1.26a8 8 0 1 0-11.62 9\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"13 11 9 17 15 17 11 23\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCloudOff")]
///This icon requires the feature `FiCloudOff` to be enabled.
#[component]
pub fn FiCloudOff(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M22.61 16.95A5 5 0 0 0 18 10h-1.26a8 8 0 0 0-7.05-6M5 5a8 8 0 0 0 4 15h9a5 5 0 0 0 1.7-.3\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"1\" y1=\"1\" x2=\"23\" y2=\"23\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCloudRain")]
///This icon requires the feature `FiCloudRain` to be enabled.
#[component]
pub fn FiCloudRain(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"16\" y1=\"13\" x2=\"16\" y2=\"21\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"13\" x2=\"8\" y2=\"21\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"15\" x2=\"12\" y2=\"23\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M20 16.58A5 5 0 0 0 18 7h-1.26A8 8 0 1 0 4 15.25\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCloudSnow")]
///This icon requires the feature `FiCloudSnow` to be enabled.
#[component]
pub fn FiCloudSnow(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M20 17.58A5 5 0 0 0 18 8h-1.26A8 8 0 1 0 4 16.25\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"16\" x2=\"8.01\" y2=\"16\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"20\" x2=\"8.01\" y2=\"20\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"18\" x2=\"12.01\" y2=\"18\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"22\" x2=\"12.01\" y2=\"22\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"16\" y1=\"16\" x2=\"16.01\" y2=\"16\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"16\" y1=\"20\" x2=\"16.01\" y2=\"20\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCode")]
///This icon requires the feature `FiCode` to be enabled.
#[component]
pub fn FiCode(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"16 18 22 12 16 6\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"8 6 2 12 8 18\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCodepen")]
///This icon requires the feature `FiCodepen` to be enabled.
#[component]
pub fn FiCodepen(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"12 2 22 8.5 22 15.5 12 22 2 15.5 2 8.5 12 2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"22\" x2=\"12\" y2=\"15.5\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"22 8.5 12 15.5 2 8.5\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"2 15.5 12 8.5 22 15.5\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"2\" x2=\"12\" y2=\"8.5\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCodesandbox")]
///This icon requires the feature `FiCodesandbox` to be enabled.
#[component]
pub fn FiCodesandbox(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"7.5 4.21 12 6.81 16.5 4.21\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"7.5 19.79 7.5 14.6 3 12\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"21 12 16.5 14.6 16.5 19.79\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"3.27 6.96 12 12.01 20.73 6.96\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"22.08\" x2=\"12\" y2=\"12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCoffee")]
///This icon requires the feature `FiCoffee` to be enabled.
#[component]
pub fn FiCoffee(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) viewBox =
        "0 0 24 24" width = "24" height = "24" fill = "none" stroke = "currentColor"
        stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width
        = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M18 8h1a4 4 0 0 1 0 8h-1\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M2 8h16v9a4 4 0 0 1-4 4H6a4 4 0 0 1-4-4V8z\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"6\" y1=\"1\" x2=\"6\" y2=\"4\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"10\" y1=\"1\" x2=\"10\" y2=\"4\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"14\" y1=\"1\" x2=\"14\" y2=\"4\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiColumns")]
///This icon requires the feature `FiColumns` to be enabled.
#[component]
pub fn FiColumns(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M12 3h7a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-7m0-18H5a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h7m0-18v18\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCommand")]
///This icon requires the feature `FiCommand` to be enabled.
#[component]
pub fn FiCommand(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M18 3a3 3 0 0 0-3 3v12a3 3 0 0 0 3 3 3 3 0 0 0 3-3 3 3 0 0 0-3-3H6a3 3 0 0 0-3 3 3 3 0 0 0 3 3 3 3 0 0 0 3-3V6a3 3 0 0 0-3-3 3 3 0 0 0-3 3 3 3 0 0 0 3 3h12a3 3 0 0 0 3-3 3 3 0 0 0-3-3z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCompass")]
///This icon requires the feature `FiCompass` to be enabled.
#[component]
pub fn FiCompass(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"16.24 7.76 14.12 14.12 7.76 16.24 9.88 9.88 16.24 7.76\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCopy")]
///This icon requires the feature `FiCopy` to be enabled.
#[component]
pub fn FiCopy(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"9\" y=\"9\" width=\"13\" height=\"13\" rx=\"2\" ry=\"2\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M5 15H4a2 2 0 0 1-2-2V4a2 2 0 0 1 2-2h9a2 2 0 0 1 2 2v1\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCornerDownLeft")]
///This icon requires the feature `FiCornerDownLeft` to be enabled.
#[component]
pub fn FiCornerDownLeft(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"9 10 4 15 9 20\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M20 4v7a4 4 0 0 1-4 4H4\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCornerDownRight")]
///This icon requires the feature `FiCornerDownRight` to be enabled.
#[component]
pub fn FiCornerDownRight(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"15 10 20 15 15 20\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M4 4v7a4 4 0 0 0 4 4h12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCornerLeftDown")]
///This icon requires the feature `FiCornerLeftDown` to be enabled.
#[component]
pub fn FiCornerLeftDown(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"14 15 9 20 4 15\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M20 4h-7a4 4 0 0 0-4 4v12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCornerLeftUp")]
///This icon requires the feature `FiCornerLeftUp` to be enabled.
#[component]
pub fn FiCornerLeftUp(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"14 9 9 4 4 9\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M20 20h-7a4 4 0 0 1-4-4V4\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCornerRightDown")]
///This icon requires the feature `FiCornerRightDown` to be enabled.
#[component]
pub fn FiCornerRightDown(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"10 15 15 20 20 15\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M4 4h7a4 4 0 0 1 4 4v12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCornerRightUp")]
///This icon requires the feature `FiCornerRightUp` to be enabled.
#[component]
pub fn FiCornerRightUp(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"10 9 15 4 20 9\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M4 20h7a4 4 0 0 0 4-4V4\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCornerUpLeft")]
///This icon requires the feature `FiCornerUpLeft` to be enabled.
#[component]
pub fn FiCornerUpLeft(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"9 14 4 9 9 4\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M20 20v-7a4 4 0 0 0-4-4H4\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCornerUpRight")]
///This icon requires the feature `FiCornerUpRight` to be enabled.
#[component]
pub fn FiCornerUpRight(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"15 14 20 9 15 4\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M4 20v-7a4 4 0 0 1 4-4h12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCpu")]
///This icon requires the feature `FiCpu` to be enabled.
#[component]
pub fn FiCpu(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"4\" y=\"4\" width=\"16\" height=\"16\" rx=\"2\" ry=\"2\" /><rect xmlns=\"http://www.w3.org/2000/svg\" x=\"9\" y=\"9\" width=\"6\" height=\"6\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"9\" y1=\"1\" x2=\"9\" y2=\"4\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"15\" y1=\"1\" x2=\"15\" y2=\"4\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"9\" y1=\"20\" x2=\"9\" y2=\"23\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"15\" y1=\"20\" x2=\"15\" y2=\"23\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"20\" y1=\"9\" x2=\"23\" y2=\"9\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"20\" y1=\"14\" x2=\"23\" y2=\"14\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"1\" y1=\"9\" x2=\"4\" y2=\"9\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"1\" y1=\"14\" x2=\"4\" y2=\"14\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCreditCard")]
///This icon requires the feature `FiCreditCard` to be enabled.
#[component]
pub fn FiCreditCard(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"1\" y=\"4\" width=\"22\" height=\"16\" rx=\"2\" ry=\"2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"1\" y1=\"10\" x2=\"23\" y2=\"10\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCrop")]
///This icon requires the feature `FiCrop` to be enabled.
#[component]
pub fn FiCrop(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M6.13 1L6 16a2 2 0 0 0 2 2h15\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M1 6.13L16 6a2 2 0 0 1 2 2v15\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiCrosshair")]
///This icon requires the feature `FiCrosshair` to be enabled.
#[component]
pub fn FiCrosshair(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"22\" y1=\"12\" x2=\"18\" y2=\"12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"6\" y1=\"12\" x2=\"2\" y2=\"12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"6\" x2=\"12\" y2=\"2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"22\" x2=\"12\" y2=\"18\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiDatabase")]
///This icon requires the feature `FiDatabase` to be enabled.
#[component]
pub fn FiDatabase(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<ellipse xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"5\" rx=\"9\" ry=\"3\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M21 12c0 1.66-4 3-9 3s-9-1.34-9-3\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M3 5v14c0 1.66 4 3 9 3s9-1.34 9-3V5\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiDelete")]
///This icon requires the feature `FiDelete` to be enabled.
#[component]
pub fn FiDelete(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M21 4H8l-7 8 7 8h13a2 2 0 0 0 2-2V6a2 2 0 0 0-2-2z\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"18\" y1=\"9\" x2=\"12\" y2=\"15\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"9\" x2=\"18\" y2=\"15\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiDisc")]
///This icon requires the feature `FiDisc` to be enabled.
#[component]
pub fn FiDisc(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"3\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiDivide")]
///This icon requires the feature `FiDivide` to be enabled.
#[component]
pub fn FiDivide(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"6\" r=\"2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"5\" y1=\"12\" x2=\"19\" y2=\"12\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"18\" r=\"2\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiDivideCircle")]
///This icon requires the feature `FiDivideCircle` to be enabled.
#[component]
pub fn FiDivideCircle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"12\" x2=\"16\" y2=\"12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"16\" x2=\"12\" y2=\"16\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"8\" x2=\"12\" y2=\"8\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiDivideSquare")]
///This icon requires the feature `FiDivideSquare` to be enabled.
#[component]
pub fn FiDivideSquare(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"12\" x2=\"16\" y2=\"12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"16\" x2=\"12\" y2=\"16\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"8\" x2=\"12\" y2=\"8\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiDollarSign")]
///This icon requires the feature `FiDollarSign` to be enabled.
#[component]
pub fn FiDollarSign(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"1\" x2=\"12\" y2=\"23\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M17 5H9.5a3.5 3.5 0 0 0 0 7h5a3.5 3.5 0 0 1 0 7H6\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiDownload")]
///This icon requires the feature `FiDownload` to be enabled.
#[component]
pub fn FiDownload(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"7 10 12 15 17 10\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"15\" x2=\"12\" y2=\"3\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiDownloadCloud")]
///This icon requires the feature `FiDownloadCloud` to be enabled.
#[component]
pub fn FiDownloadCloud(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"8 17 12 21 16 17\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"12\" x2=\"12\" y2=\"21\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M20.88 18.09A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.29\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiDribbble")]
///This icon requires the feature `FiDribbble` to be enabled.
#[component]
pub fn FiDribbble(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M8.56 2.75c4.37 6.03 6.02 9.42 8.03 17.72m2.54-15.38c-3.72 4.35-8.94 5.66-16.88 5.85m19.5 1.9c-3.5-.93-6.63-.82-8.94 0-2.58.92-5.01 2.86-7.44 6.32\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiDroplet")]
///This icon requires the feature `FiDroplet` to be enabled.
#[component]
pub fn FiDroplet(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M12 2.69l5.66 5.66a8 8 0 1 1-11.31 0z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiEdit")]
///This icon requires the feature `FiEdit` to be enabled.
#[component]
pub fn FiEdit(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M11 4H4a2 2 0 0 0-2 2v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2v-7\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M18.5 2.5a2.121 2.121 0 0 1 3 3L12 15l-4 1 1-4 9.5-9.5z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiEdit2")]
///This icon requires the feature `FiEdit2` to be enabled.
#[component]
pub fn FiEdit2(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M17 3a2.828 2.828 0 1 1 4 4L7.5 20.5 2 22l1.5-5.5L17 3z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiEdit3")]
///This icon requires the feature `FiEdit3` to be enabled.
#[component]
pub fn FiEdit3(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M12 20h9\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M16.5 3.5a2.121 2.121 0 0 1 3 3L7 19l-4 1 1-4L16.5 3.5z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiExternalLink")]
///This icon requires the feature `FiExternalLink` to be enabled.
#[component]
pub fn FiExternalLink(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M18 13v6a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V8a2 2 0 0 1 2-2h6\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"15 3 21 3 21 9\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"10\" y1=\"14\" x2=\"21\" y2=\"3\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiEye")]
///This icon requires the feature `FiEye` to be enabled.
#[component]
pub fn FiEye(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M1 12s4-8 11-8 11 8 11 8-4 8-11 8-11-8-11-8z\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"3\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiEyeOff")]
///This icon requires the feature `FiEyeOff` to be enabled.
#[component]
pub fn FiEyeOff(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M17.94 17.94A10.07 10.07 0 0 1 12 20c-7 0-11-8-11-8a18.45 18.45 0 0 1 5.06-5.94M9.9 4.24A9.12 9.12 0 0 1 12 4c7 0 11 8 11 8a18.5 18.5 0 0 1-2.16 3.19m-6.72-1.07a3 3 0 1 1-4.24-4.24\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"1\" y1=\"1\" x2=\"23\" y2=\"23\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiFacebook")]
///This icon requires the feature `FiFacebook` to be enabled.
#[component]
pub fn FiFacebook(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M18 2h-3a5 5 0 0 0-5 5v3H7v4h3v8h4v-8h3l1-4h-4V7a1 1 0 0 1 1-1h3z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiFastForward")]
///This icon requires the feature `FiFastForward` to be enabled.
#[component]
pub fn FiFastForward(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"13 19 22 12 13 5 13 19\" /><polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"2 19 11 12 2 5 2 19\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiFeather")]
///This icon requires the feature `FiFeather` to be enabled.
#[component]
pub fn FiFeather(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M20.24 12.24a6 6 0 0 0-8.49-8.49L5 10.5V19h8.5z\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"16\" y1=\"8\" x2=\"2\" y2=\"22\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"17.5\" y1=\"15\" x2=\"9\" y2=\"15\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiFigma")]
///This icon requires the feature `FiFigma` to be enabled.
#[component]
pub fn FiFigma(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M5 5.5A3.5 3.5 0 0 1 8.5 2H12v7H8.5A3.5 3.5 0 0 1 5 5.5z\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M12 2h3.5a3.5 3.5 0 1 1 0 7H12V2z\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M12 12.5a3.5 3.5 0 1 1 7 0 3.5 3.5 0 1 1-7 0z\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M5 19.5A3.5 3.5 0 0 1 8.5 16H12v3.5a3.5 3.5 0 1 1-7 0z\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M5 12.5A3.5 3.5 0 0 1 8.5 9H12v7H8.5A3.5 3.5 0 0 1 5 12.5z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiFile")]
///This icon requires the feature `FiFile` to be enabled.
#[component]
pub fn FiFile(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M13 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V9z\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"13 2 13 9 20 9\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiFileMinus")]
///This icon requires the feature `FiFileMinus` to be enabled.
#[component]
pub fn FiFileMinus(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"14 2 14 8 20 8\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"9\" y1=\"15\" x2=\"15\" y2=\"15\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiFilePlus")]
///This icon requires the feature `FiFilePlus` to be enabled.
#[component]
pub fn FiFilePlus(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"14 2 14 8 20 8\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"18\" x2=\"12\" y2=\"12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"9\" y1=\"15\" x2=\"15\" y2=\"15\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiFileText")]
///This icon requires the feature `FiFileText` to be enabled.
#[component]
pub fn FiFileText(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M14 2H6a2 2 0 0 0-2 2v16a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2V8z\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"14 2 14 8 20 8\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"16\" y1=\"13\" x2=\"8\" y2=\"13\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"16\" y1=\"17\" x2=\"8\" y2=\"17\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"10 9 9 9 8 9\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiFilm")]
///This icon requires the feature `FiFilm` to be enabled.
#[component]
pub fn FiFilm(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"2\" y=\"2\" width=\"20\" height=\"20\" rx=\"2.18\" ry=\"2.18\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"7\" y1=\"2\" x2=\"7\" y2=\"22\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"17\" y1=\"2\" x2=\"17\" y2=\"22\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"2\" y1=\"12\" x2=\"22\" y2=\"12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"2\" y1=\"7\" x2=\"7\" y2=\"7\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"2\" y1=\"17\" x2=\"7\" y2=\"17\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"17\" y1=\"17\" x2=\"22\" y2=\"17\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"17\" y1=\"7\" x2=\"22\" y2=\"7\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiFilter")]
///This icon requires the feature `FiFilter` to be enabled.
#[component]
pub fn FiFilter(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"22 3 2 3 10 12.46 10 19 14 21 14 12.46 22 3\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiFlag")]
///This icon requires the feature `FiFlag` to be enabled.
#[component]
pub fn FiFlag(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M4 15s1-1 4-1 5 2 8 2 4-1 4-1V3s-1 1-4 1-5-2-8-2-4 1-4 1z\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"4\" y1=\"22\" x2=\"4\" y2=\"15\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiFolder")]
///This icon requires the feature `FiFolder` to be enabled.
#[component]
pub fn FiFolder(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiFolderMinus")]
///This icon requires the feature `FiFolderMinus` to be enabled.
#[component]
pub fn FiFolderMinus(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"9\" y1=\"14\" x2=\"15\" y2=\"14\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiFolderPlus")]
///This icon requires the feature `FiFolderPlus` to be enabled.
#[component]
pub fn FiFolderPlus(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M22 19a2 2 0 0 1-2 2H4a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h5l2 3h9a2 2 0 0 1 2 2z\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"11\" x2=\"12\" y2=\"17\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"9\" y1=\"14\" x2=\"15\" y2=\"14\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiFramer")]
///This icon requires the feature `FiFramer` to be enabled.
#[component]
pub fn FiFramer(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M5 16V9h14V2H5l14 14h-7m-7 0l7 7v-7m-7 0h7\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiFrown")]
///This icon requires the feature `FiFrown` to be enabled.
#[component]
pub fn FiFrown(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) viewBox =
        "0 0 24 24" width = "24" height = "24" fill = "none" stroke = "currentColor"
        stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width
        = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M16 16s-1.5-2-4-2-4 2-4 2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"9\" y1=\"9\" x2=\"9.01\" y2=\"9\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"15\" y1=\"9\" x2=\"15.01\" y2=\"9\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiGift")]
///This icon requires the feature `FiGift` to be enabled.
#[component]
pub fn FiGift(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"20 12 20 22 4 22 4 12\" /><rect xmlns=\"http://www.w3.org/2000/svg\" x=\"2\" y=\"7\" width=\"20\" height=\"5\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"22\" x2=\"12\" y2=\"7\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M12 7H7.5a2.5 2.5 0 0 1 0-5C11 2 12 7 12 7z\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M12 7h4.5a2.5 2.5 0 0 0 0-5C13 2 12 7 12 7z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiGitBranch")]
///This icon requires the feature `FiGitBranch` to be enabled.
#[component]
pub fn FiGitBranch(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"6\" y1=\"3\" x2=\"6\" y2=\"15\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"18\" cy=\"6\" r=\"3\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"6\" cy=\"18\" r=\"3\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M18 9a9 9 0 0 1-9 9\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiGitCommit")]
///This icon requires the feature `FiGitCommit` to be enabled.
#[component]
pub fn FiGitCommit(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"4\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"1.05\" y1=\"12\" x2=\"7\" y2=\"12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"17.01\" y1=\"12\" x2=\"22.96\" y2=\"12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiGitMerge")]
///This icon requires the feature `FiGitMerge` to be enabled.
#[component]
pub fn FiGitMerge(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"18\" cy=\"18\" r=\"3\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"6\" cy=\"6\" r=\"3\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M6 21V9a9 9 0 0 0 9 9\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiGitPullRequest")]
///This icon requires the feature `FiGitPullRequest` to be enabled.
#[component]
pub fn FiGitPullRequest(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"18\" cy=\"18\" r=\"3\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"6\" cy=\"6\" r=\"3\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M13 6h3a2 2 0 0 1 2 2v7\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"6\" y1=\"9\" x2=\"6\" y2=\"21\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiGithub")]
///This icon requires the feature `FiGithub` to be enabled.
#[component]
pub fn FiGithub(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M9 19c-5 1.5-5-2.5-7-3m14 6v-3.87a3.37 3.37 0 0 0-.94-2.61c3.14-.35 6.44-1.54 6.44-7A5.44 5.44 0 0 0 20 4.77 5.07 5.07 0 0 0 19.91 1S18.73.65 16 2.48a13.38 13.38 0 0 0-7 0C6.27.65 5.09 1 5.09 1A5.07 5.07 0 0 0 5 4.77a5.44 5.44 0 0 0-1.5 3.78c0 5.42 3.3 6.61 6.44 7A3.37 3.37 0 0 0 9 18.13V22\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiGitlab")]
///This icon requires the feature `FiGitlab` to be enabled.
#[component]
pub fn FiGitlab(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M22.65 14.39L12 22.13 1.35 14.39a.84.84 0 0 1-.3-.94l1.22-3.78 2.44-7.51A.42.42 0 0 1 4.82 2a.43.43 0 0 1 .58 0 .42.42 0 0 1 .11.18l2.44 7.49h8.1l2.44-7.51A.42.42 0 0 1 18.6 2a.43.43 0 0 1 .58 0 .42.42 0 0 1 .11.18l2.44 7.51L23 13.45a.84.84 0 0 1-.35.94z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiGlobe")]
///This icon requires the feature `FiGlobe` to be enabled.
#[component]
pub fn FiGlobe(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"2\" y1=\"12\" x2=\"22\" y2=\"12\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M12 2a15.3 15.3 0 0 1 4 10 15.3 15.3 0 0 1-4 10 15.3 15.3 0 0 1-4-10 15.3 15.3 0 0 1 4-10z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiGrid")]
///This icon requires the feature `FiGrid` to be enabled.
#[component]
pub fn FiGrid(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"3\" y=\"3\" width=\"7\" height=\"7\" /><rect xmlns=\"http://www.w3.org/2000/svg\" x=\"14\" y=\"3\" width=\"7\" height=\"7\" /><rect xmlns=\"http://www.w3.org/2000/svg\" x=\"14\" y=\"14\" width=\"7\" height=\"7\" /><rect xmlns=\"http://www.w3.org/2000/svg\" x=\"3\" y=\"14\" width=\"7\" height=\"7\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiHardDrive")]
///This icon requires the feature `FiHardDrive` to be enabled.
#[component]
pub fn FiHardDrive(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"22\" y1=\"12\" x2=\"2\" y2=\"12\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"6\" y1=\"16\" x2=\"6.01\" y2=\"16\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"10\" y1=\"16\" x2=\"10.01\" y2=\"16\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiHash")]
///This icon requires the feature `FiHash` to be enabled.
#[component]
pub fn FiHash(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"4\" y1=\"9\" x2=\"20\" y2=\"9\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"4\" y1=\"15\" x2=\"20\" y2=\"15\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"10\" y1=\"3\" x2=\"8\" y2=\"21\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"16\" y1=\"3\" x2=\"14\" y2=\"21\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiHeadphones")]
///This icon requires the feature `FiHeadphones` to be enabled.
#[component]
pub fn FiHeadphones(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M3 18v-6a9 9 0 0 1 18 0v6\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M21 19a2 2 0 0 1-2 2h-1a2 2 0 0 1-2-2v-3a2 2 0 0 1 2-2h3zM3 19a2 2 0 0 0 2 2h1a2 2 0 0 0 2-2v-3a2 2 0 0 0-2-2H3z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiHeart")]
///This icon requires the feature `FiHeart` to be enabled.
#[component]
pub fn FiHeart(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M20.84 4.61a5.5 5.5 0 0 0-7.78 0L12 5.67l-1.06-1.06a5.5 5.5 0 0 0-7.78 7.78l1.06 1.06L12 21.23l7.78-7.78 1.06-1.06a5.5 5.5 0 0 0 0-7.78z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiHelpCircle")]
///This icon requires the feature `FiHelpCircle` to be enabled.
#[component]
pub fn FiHelpCircle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M9.09 9a3 3 0 0 1 5.83 1c0 2-3 3-3 3\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"17\" x2=\"12.01\" y2=\"17\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiHexagon")]
///This icon requires the feature `FiHexagon` to be enabled.
#[component]
pub fn FiHexagon(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiHome")]
///This icon requires the feature `FiHome` to be enabled.
#[component]
pub fn FiHome(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M3 9l9-7 9 7v11a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2z\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"9 22 9 12 15 12 15 22\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiImage")]
///This icon requires the feature `FiImage` to be enabled.
#[component]
pub fn FiImage(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"8.5\" cy=\"8.5\" r=\"1.5\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"21 15 16 10 5 21\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiInbox")]
///This icon requires the feature `FiInbox` to be enabled.
#[component]
pub fn FiInbox(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"22 12 16 12 14 15 10 15 8 12 2 12\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M5.45 5.11L2 12v6a2 2 0 0 0 2 2h16a2 2 0 0 0 2-2v-6l-3.45-6.89A2 2 0 0 0 16.76 4H7.24a2 2 0 0 0-1.79 1.11z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiInfo")]
///This icon requires the feature `FiInfo` to be enabled.
#[component]
pub fn FiInfo(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"16\" x2=\"12\" y2=\"12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"8\" x2=\"12.01\" y2=\"8\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiInstagram")]
///This icon requires the feature `FiInstagram` to be enabled.
#[component]
pub fn FiInstagram(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"2\" y=\"2\" width=\"20\" height=\"20\" rx=\"5\" ry=\"5\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M16 11.37A4 4 0 1 1 12.63 8 4 4 0 0 1 16 11.37z\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"17.5\" y1=\"6.5\" x2=\"17.51\" y2=\"6.5\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiItalic")]
///This icon requires the feature `FiItalic` to be enabled.
#[component]
pub fn FiItalic(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"19\" y1=\"4\" x2=\"10\" y2=\"4\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"14\" y1=\"20\" x2=\"5\" y2=\"20\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"15\" y1=\"4\" x2=\"9\" y2=\"20\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiKey")]
///This icon requires the feature `FiKey` to be enabled.
#[component]
pub fn FiKey(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M21 2l-2 2m-7.61 7.61a5.5 5.5 0 1 1-7.778 7.778 5.5 5.5 0 0 1 7.777-7.777zm0 0L15.5 7.5m0 0l3 3L22 7l-3-3m-3.5 3.5L19 4\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiLayers")]
///This icon requires the feature `FiLayers` to be enabled.
#[component]
pub fn FiLayers(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"12 2 2 7 12 12 22 7 12 2\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"2 17 12 22 22 17\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"2 12 12 17 22 12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiLayout")]
///This icon requires the feature `FiLayout` to be enabled.
#[component]
pub fn FiLayout(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"3\" y1=\"9\" x2=\"21\" y2=\"9\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"9\" y1=\"21\" x2=\"9\" y2=\"9\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiLifeBuoy")]
///This icon requires the feature `FiLifeBuoy` to be enabled.
#[component]
pub fn FiLifeBuoy(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"4\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"4.93\" y1=\"4.93\" x2=\"9.17\" y2=\"9.17\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"14.83\" y1=\"14.83\" x2=\"19.07\" y2=\"19.07\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"14.83\" y1=\"9.17\" x2=\"19.07\" y2=\"4.93\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"14.83\" y1=\"9.17\" x2=\"18.36\" y2=\"5.64\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"4.93\" y1=\"19.07\" x2=\"9.17\" y2=\"14.83\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiLink")]
///This icon requires the feature `FiLink` to be enabled.
#[component]
pub fn FiLink(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M10 13a5 5 0 0 0 7.54.54l3-3a5 5 0 0 0-7.07-7.07l-1.72 1.71\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M14 11a5 5 0 0 0-7.54-.54l-3 3a5 5 0 0 0 7.07 7.07l1.71-1.71\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiLink2")]
///This icon requires the feature `FiLink2` to be enabled.
#[component]
pub fn FiLink2(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M15 7h3a5 5 0 0 1 5 5 5 5 0 0 1-5 5h-3m-6 0H6a5 5 0 0 1-5-5 5 5 0 0 1 5-5h3\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"12\" x2=\"16\" y2=\"12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiLinkedin")]
///This icon requires the feature `FiLinkedin` to be enabled.
#[component]
pub fn FiLinkedin(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M16 8a6 6 0 0 1 6 6v7h-4v-7a2 2 0 0 0-2-2 2 2 0 0 0-2 2v7h-4v-7a6 6 0 0 1 6-6z\" /><rect xmlns=\"http://www.w3.org/2000/svg\" x=\"2\" y=\"9\" width=\"4\" height=\"12\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"4\" cy=\"4\" r=\"2\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiList")]
///This icon requires the feature `FiList` to be enabled.
#[component]
pub fn FiList(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"6\" x2=\"21\" y2=\"6\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"12\" x2=\"21\" y2=\"12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"18\" x2=\"21\" y2=\"18\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"3\" y1=\"6\" x2=\"3.01\" y2=\"6\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"3\" y1=\"12\" x2=\"3.01\" y2=\"12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"3\" y1=\"18\" x2=\"3.01\" y2=\"18\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiLoader")]
///This icon requires the feature `FiLoader` to be enabled.
#[component]
pub fn FiLoader(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"2\" x2=\"12\" y2=\"6\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"18\" x2=\"12\" y2=\"22\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"4.93\" y1=\"4.93\" x2=\"7.76\" y2=\"7.76\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"16.24\" y1=\"16.24\" x2=\"19.07\" y2=\"19.07\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"2\" y1=\"12\" x2=\"6\" y2=\"12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"18\" y1=\"12\" x2=\"22\" y2=\"12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"4.93\" y1=\"19.07\" x2=\"7.76\" y2=\"16.24\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"16.24\" y1=\"7.76\" x2=\"19.07\" y2=\"4.93\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiLock")]
///This icon requires the feature `FiLock` to be enabled.
#[component]
pub fn FiLock(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"3\" y=\"11\" width=\"18\" height=\"11\" rx=\"2\" ry=\"2\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M7 11V7a5 5 0 0 1 10 0v4\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiLogIn")]
///This icon requires the feature `FiLogIn` to be enabled.
#[component]
pub fn FiLogIn(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M15 3h4a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2h-4\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"10 17 15 12 10 7\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"15\" y1=\"12\" x2=\"3\" y2=\"12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiLogOut")]
///This icon requires the feature `FiLogOut` to be enabled.
#[component]
pub fn FiLogOut(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"16 17 21 12 16 7\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"21\" y1=\"12\" x2=\"9\" y2=\"12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMail")]
///This icon requires the feature `FiMail` to be enabled.
#[component]
pub fn FiMail(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M4 4h16c1.1 0 2 .9 2 2v12c0 1.1-.9 2-2 2H4c-1.1 0-2-.9-2-2V6c0-1.1.9-2 2-2z\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"22,6 12,13 2,6\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMap")]
///This icon requires the feature `FiMap` to be enabled.
#[component]
pub fn FiMap(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"1 6 1 22 8 18 16 22 23 18 23 2 16 6 8 2 1 6\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"2\" x2=\"8\" y2=\"18\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"16\" y1=\"6\" x2=\"16\" y2=\"22\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMapPin")]
///This icon requires the feature `FiMapPin` to be enabled.
#[component]
pub fn FiMapPin(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M21 10c0 7-9 13-9 13s-9-6-9-13a9 9 0 0 1 18 0z\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"10\" r=\"3\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMaximize")]
///This icon requires the feature `FiMaximize` to be enabled.
#[component]
pub fn FiMaximize(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M8 3H5a2 2 0 0 0-2 2v3m18 0V5a2 2 0 0 0-2-2h-3m0 18h3a2 2 0 0 0 2-2v-3M3 16v3a2 2 0 0 0 2 2h3\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMaximize2")]
///This icon requires the feature `FiMaximize2` to be enabled.
#[component]
pub fn FiMaximize2(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"15 3 21 3 21 9\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"9 21 3 21 3 15\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"21\" y1=\"3\" x2=\"14\" y2=\"10\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"3\" y1=\"21\" x2=\"10\" y2=\"14\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMeh")]
///This icon requires the feature `FiMeh` to be enabled.
#[component]
pub fn FiMeh(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) viewBox =
        "0 0 24 24" width = "24" height = "24" fill = "none" stroke = "currentColor"
        stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width
        = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"15\" x2=\"16\" y2=\"15\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"9\" y1=\"9\" x2=\"9.01\" y2=\"9\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"15\" y1=\"9\" x2=\"15.01\" y2=\"9\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMenu")]
///This icon requires the feature `FiMenu` to be enabled.
#[component]
pub fn FiMenu(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"3\" y1=\"12\" x2=\"21\" y2=\"12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"3\" y1=\"6\" x2=\"21\" y2=\"6\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"3\" y1=\"18\" x2=\"21\" y2=\"18\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMessageCircle")]
///This icon requires the feature `FiMessageCircle` to be enabled.
#[component]
pub fn FiMessageCircle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M21 11.5a8.38 8.38 0 0 1-.9 3.8 8.5 8.5 0 0 1-7.6 4.7 8.38 8.38 0 0 1-3.8-.9L3 21l1.9-5.7a8.38 8.38 0 0 1-.9-3.8 8.5 8.5 0 0 1 4.7-7.6 8.38 8.38 0 0 1 3.8-.9h.5a8.48 8.48 0 0 1 8 8v.5z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMessageSquare")]
///This icon requires the feature `FiMessageSquare` to be enabled.
#[component]
pub fn FiMessageSquare(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M21 15a2 2 0 0 1-2 2H7l-4 4V5a2 2 0 0 1 2-2h14a2 2 0 0 1 2 2z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMic")]
///This icon requires the feature `FiMic` to be enabled.
#[component]
pub fn FiMic(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M12 1a3 3 0 0 0-3 3v8a3 3 0 0 0 6 0V4a3 3 0 0 0-3-3z\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M19 10v2a7 7 0 0 1-14 0v-2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"19\" x2=\"12\" y2=\"23\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"23\" x2=\"16\" y2=\"23\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMicOff")]
///This icon requires the feature `FiMicOff` to be enabled.
#[component]
pub fn FiMicOff(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"1\" y1=\"1\" x2=\"23\" y2=\"23\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M9 9v3a3 3 0 0 0 5.12 2.12M15 9.34V4a3 3 0 0 0-5.94-.6\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M17 16.95A7 7 0 0 1 5 12v-2m14 0v2a7 7 0 0 1-.11 1.23\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"19\" x2=\"12\" y2=\"23\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"23\" x2=\"16\" y2=\"23\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMinimize")]
///This icon requires the feature `FiMinimize` to be enabled.
#[component]
pub fn FiMinimize(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M8 3v3a2 2 0 0 1-2 2H3m18 0h-3a2 2 0 0 1-2-2V3m0 18v-3a2 2 0 0 1 2-2h3M3 16h3a2 2 0 0 1 2 2v3\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMinimize2")]
///This icon requires the feature `FiMinimize2` to be enabled.
#[component]
pub fn FiMinimize2(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"4 14 10 14 10 20\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"20 10 14 10 14 4\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"14\" y1=\"10\" x2=\"21\" y2=\"3\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"3\" y1=\"21\" x2=\"10\" y2=\"14\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMinus")]
///This icon requires the feature `FiMinus` to be enabled.
#[component]
pub fn FiMinus(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"5\" y1=\"12\" x2=\"19\" y2=\"12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMinusCircle")]
///This icon requires the feature `FiMinusCircle` to be enabled.
#[component]
pub fn FiMinusCircle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"12\" x2=\"16\" y2=\"12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMinusSquare")]
///This icon requires the feature `FiMinusSquare` to be enabled.
#[component]
pub fn FiMinusSquare(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"12\" x2=\"16\" y2=\"12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMonitor")]
///This icon requires the feature `FiMonitor` to be enabled.
#[component]
pub fn FiMonitor(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"2\" y=\"3\" width=\"20\" height=\"14\" rx=\"2\" ry=\"2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"21\" x2=\"16\" y2=\"21\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"17\" x2=\"12\" y2=\"21\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMoon")]
///This icon requires the feature `FiMoon` to be enabled.
#[component]
pub fn FiMoon(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M21 12.79A9 9 0 1 1 11.21 3 7 7 0 0 0 21 12.79z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMoreHorizontal")]
///This icon requires the feature `FiMoreHorizontal` to be enabled.
#[component]
pub fn FiMoreHorizontal(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"1\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"19\" cy=\"12\" r=\"1\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"5\" cy=\"12\" r=\"1\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMoreVertical")]
///This icon requires the feature `FiMoreVertical` to be enabled.
#[component]
pub fn FiMoreVertical(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"1\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"5\" r=\"1\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"19\" r=\"1\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMousePointer")]
///This icon requires the feature `FiMousePointer` to be enabled.
#[component]
pub fn FiMousePointer(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M3 3l7.07 16.97 2.51-7.39 7.39-2.51L3 3z\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M13 13l6 6\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMove")]
///This icon requires the feature `FiMove` to be enabled.
#[component]
pub fn FiMove(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"5 9 2 12 5 15\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"9 5 12 2 15 5\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"15 19 12 22 9 19\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"19 9 22 12 19 15\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"2\" y1=\"12\" x2=\"22\" y2=\"12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"2\" x2=\"12\" y2=\"22\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiMusic")]
///This icon requires the feature `FiMusic` to be enabled.
#[component]
pub fn FiMusic(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M9 18V5l12-2v13\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"6\" cy=\"18\" r=\"3\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"18\" cy=\"16\" r=\"3\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiNavigation")]
///This icon requires the feature `FiNavigation` to be enabled.
#[component]
pub fn FiNavigation(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"3 11 22 2 13 21 11 13 3 11\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiNavigation2")]
///This icon requires the feature `FiNavigation2` to be enabled.
#[component]
pub fn FiNavigation2(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"12 2 19 21 12 17 5 21 12 2\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiOctagon")]
///This icon requires the feature `FiOctagon` to be enabled.
#[component]
pub fn FiOctagon(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPackage")]
///This icon requires the feature `FiPackage` to be enabled.
#[component]
pub fn FiPackage(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"16.5\" y1=\"9.4\" x2=\"7.5\" y2=\"4.21\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M21 16V8a2 2 0 0 0-1-1.73l-7-4a2 2 0 0 0-2 0l-7 4A2 2 0 0 0 3 8v8a2 2 0 0 0 1 1.73l7 4a2 2 0 0 0 2 0l7-4A2 2 0 0 0 21 16z\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"3.27 6.96 12 12.01 20.73 6.96\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"22.08\" x2=\"12\" y2=\"12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPaperclip")]
///This icon requires the feature `FiPaperclip` to be enabled.
#[component]
pub fn FiPaperclip(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M21.44 11.05l-9.19 9.19a6 6 0 0 1-8.49-8.49l9.19-9.19a4 4 0 0 1 5.66 5.66l-9.2 9.19a2 2 0 0 1-2.83-2.83l8.49-8.48\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPause")]
///This icon requires the feature `FiPause` to be enabled.
#[component]
pub fn FiPause(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"6\" y=\"4\" width=\"4\" height=\"16\" /><rect xmlns=\"http://www.w3.org/2000/svg\" x=\"14\" y=\"4\" width=\"4\" height=\"16\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPauseCircle")]
///This icon requires the feature `FiPauseCircle` to be enabled.
#[component]
pub fn FiPauseCircle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"10\" y1=\"15\" x2=\"10\" y2=\"9\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"14\" y1=\"15\" x2=\"14\" y2=\"9\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPenTool")]
///This icon requires the feature `FiPenTool` to be enabled.
#[component]
pub fn FiPenTool(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M12 19l7-7 3 3-7 7-3-3z\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M18 13l-1.5-7.5L2 2l3.5 14.5L13 18l5-5z\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M2 2l7.586 7.586\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"11\" cy=\"11\" r=\"2\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPercent")]
///This icon requires the feature `FiPercent` to be enabled.
#[component]
pub fn FiPercent(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"19\" y1=\"5\" x2=\"5\" y2=\"19\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"6.5\" cy=\"6.5\" r=\"2.5\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"17.5\" cy=\"17.5\" r=\"2.5\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPhone")]
///This icon requires the feature `FiPhone` to be enabled.
#[component]
pub fn FiPhone(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPhoneCall")]
///This icon requires the feature `FiPhoneCall` to be enabled.
#[component]
pub fn FiPhoneCall(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M15.05 5A5 5 0 0 1 19 8.95M15.05 1A9 9 0 0 1 23 8.94m-1 7.98v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPhoneForwarded")]
///This icon requires the feature `FiPhoneForwarded` to be enabled.
#[component]
pub fn FiPhoneForwarded(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"19 1 23 5 19 9\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"15\" y1=\"5\" x2=\"23\" y2=\"5\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPhoneIncoming")]
///This icon requires the feature `FiPhoneIncoming` to be enabled.
#[component]
pub fn FiPhoneIncoming(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"16 2 16 8 22 8\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"23\" y1=\"1\" x2=\"16\" y2=\"8\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPhoneMissed")]
///This icon requires the feature `FiPhoneMissed` to be enabled.
#[component]
pub fn FiPhoneMissed(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"23\" y1=\"1\" x2=\"17\" y2=\"7\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"17\" y1=\"1\" x2=\"23\" y2=\"7\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPhoneOff")]
///This icon requires the feature `FiPhoneOff` to be enabled.
#[component]
pub fn FiPhoneOff(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M10.68 13.31a16 16 0 0 0 3.41 2.6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7 2 2 0 0 1 1.72 2v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.42 19.42 0 0 1-3.33-2.67m-2.67-3.34a19.79 19.79 0 0 1-3.07-8.63A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"23\" y1=\"1\" x2=\"1\" y2=\"23\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPhoneOutgoing")]
///This icon requires the feature `FiPhoneOutgoing` to be enabled.
#[component]
pub fn FiPhoneOutgoing(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"23 7 23 1 17 1\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"16\" y1=\"8\" x2=\"23\" y2=\"1\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M22 16.92v3a2 2 0 0 1-2.18 2 19.79 19.79 0 0 1-8.63-3.07 19.5 19.5 0 0 1-6-6 19.79 19.79 0 0 1-3.07-8.67A2 2 0 0 1 4.11 2h3a2 2 0 0 1 2 1.72 12.84 12.84 0 0 0 .7 2.81 2 2 0 0 1-.45 2.11L8.09 9.91a16 16 0 0 0 6 6l1.27-1.27a2 2 0 0 1 2.11-.45 12.84 12.84 0 0 0 2.81.7A2 2 0 0 1 22 16.92z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPieChart")]
///This icon requires the feature `FiPieChart` to be enabled.
#[component]
pub fn FiPieChart(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M21.21 15.89A10 10 0 1 1 8 2.83\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M22 12A10 10 0 0 0 12 2v10z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPlay")]
///This icon requires the feature `FiPlay` to be enabled.
#[component]
pub fn FiPlay(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"5 3 19 12 5 21 5 3\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPlayCircle")]
///This icon requires the feature `FiPlayCircle` to be enabled.
#[component]
pub fn FiPlayCircle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"10 8 16 12 10 16 10 8\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPlus")]
///This icon requires the feature `FiPlus` to be enabled.
#[component]
pub fn FiPlus(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"5\" x2=\"12\" y2=\"19\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"5\" y1=\"12\" x2=\"19\" y2=\"12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPlusCircle")]
///This icon requires the feature `FiPlusCircle` to be enabled.
#[component]
pub fn FiPlusCircle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"8\" x2=\"12\" y2=\"16\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"12\" x2=\"16\" y2=\"12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPlusSquare")]
///This icon requires the feature `FiPlusSquare` to be enabled.
#[component]
pub fn FiPlusSquare(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"8\" x2=\"12\" y2=\"16\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8\" y1=\"12\" x2=\"16\" y2=\"12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPocket")]
///This icon requires the feature `FiPocket` to be enabled.
#[component]
pub fn FiPocket(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M4 3h16a2 2 0 0 1 2 2v6a10 10 0 0 1-10 10A10 10 0 0 1 2 11V5a2 2 0 0 1 2-2z\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"8 10 12 14 16 10\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPower")]
///This icon requires the feature `FiPower` to be enabled.
#[component]
pub fn FiPower(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M18.36 6.64a9 9 0 1 1-12.73 0\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"2\" x2=\"12\" y2=\"12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiPrinter")]
///This icon requires the feature `FiPrinter` to be enabled.
#[component]
pub fn FiPrinter(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"6 9 6 2 18 2 18 9\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M6 18H4a2 2 0 0 1-2-2v-5a2 2 0 0 1 2-2h16a2 2 0 0 1 2 2v5a2 2 0 0 1-2 2h-2\" /><rect xmlns=\"http://www.w3.org/2000/svg\" x=\"6\" y=\"14\" width=\"12\" height=\"8\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiRadio")]
///This icon requires the feature `FiRadio` to be enabled.
#[component]
pub fn FiRadio(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"2\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M16.24 7.76a6 6 0 0 1 0 8.49m-8.48-.01a6 6 0 0 1 0-8.49m11.31-2.82a10 10 0 0 1 0 14.14m-14.14 0a10 10 0 0 1 0-14.14\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiRefreshCcw")]
///This icon requires the feature `FiRefreshCcw` to be enabled.
#[component]
pub fn FiRefreshCcw(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"1 4 1 10 7 10\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"23 20 23 14 17 14\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M20.49 9A9 9 0 0 0 5.64 5.64L1 10m22 4l-4.64 4.36A9 9 0 0 1 3.51 15\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiRefreshCw")]
///This icon requires the feature `FiRefreshCw` to be enabled.
#[component]
pub fn FiRefreshCw(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"23 4 23 10 17 10\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"1 20 1 14 7 14\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M3.51 9a9 9 0 0 1 14.85-3.36L23 10M1 14l4.64 4.36A9 9 0 0 0 20.49 15\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiRepeat")]
///This icon requires the feature `FiRepeat` to be enabled.
#[component]
pub fn FiRepeat(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"17 1 21 5 17 9\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M3 11V9a4 4 0 0 1 4-4h14\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"7 23 3 19 7 15\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M21 13v2a4 4 0 0 1-4 4H3\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiRewind")]
///This icon requires the feature `FiRewind` to be enabled.
#[component]
pub fn FiRewind(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"11 19 2 12 11 5 11 19\" /><polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"22 19 13 12 22 5 22 19\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiRotateCcw")]
///This icon requires the feature `FiRotateCcw` to be enabled.
#[component]
pub fn FiRotateCcw(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"1 4 1 10 7 10\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M3.51 15a9 9 0 1 0 2.13-9.36L1 10\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiRotateCw")]
///This icon requires the feature `FiRotateCw` to be enabled.
#[component]
pub fn FiRotateCw(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"23 4 23 10 17 10\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M20.49 15a9 9 0 1 1-2.12-9.36L23 10\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiRss")]
///This icon requires the feature `FiRss` to be enabled.
#[component]
pub fn FiRss(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M4 11a9 9 0 0 1 9 9\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M4 4a16 16 0 0 1 16 16\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"5\" cy=\"19\" r=\"1\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiSave")]
///This icon requires the feature `FiSave` to be enabled.
#[component]
pub fn FiSave(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M19 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h11l5 5v11a2 2 0 0 1-2 2z\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"17 21 17 13 7 13 7 21\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"7 3 7 8 15 8\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiScissors")]
///This icon requires the feature `FiScissors` to be enabled.
#[component]
pub fn FiScissors(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"6\" cy=\"6\" r=\"3\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"6\" cy=\"18\" r=\"3\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"20\" y1=\"4\" x2=\"8.12\" y2=\"15.88\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"14.47\" y1=\"14.48\" x2=\"20\" y2=\"20\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8.12\" y1=\"8.12\" x2=\"12\" y2=\"12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiSearch")]
///This icon requires the feature `FiSearch` to be enabled.
#[component]
pub fn FiSearch(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"11\" cy=\"11\" r=\"8\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"21\" y1=\"21\" x2=\"16.65\" y2=\"16.65\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiSend")]
///This icon requires the feature `FiSend` to be enabled.
#[component]
pub fn FiSend(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"22\" y1=\"2\" x2=\"11\" y2=\"13\" /><polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"22 2 15 22 11 13 2 9 22 2\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiServer")]
///This icon requires the feature `FiServer` to be enabled.
#[component]
pub fn FiServer(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"2\" y=\"2\" width=\"20\" height=\"8\" rx=\"2\" ry=\"2\" /><rect xmlns=\"http://www.w3.org/2000/svg\" x=\"2\" y=\"14\" width=\"20\" height=\"8\" rx=\"2\" ry=\"2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"6\" y1=\"6\" x2=\"6.01\" y2=\"6\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"6\" y1=\"18\" x2=\"6.01\" y2=\"18\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiSettings")]
///This icon requires the feature `FiSettings` to be enabled.
#[component]
pub fn FiSettings(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"3\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M19.4 15a1.65 1.65 0 0 0 .33 1.82l.06.06a2 2 0 0 1 0 2.83 2 2 0 0 1-2.83 0l-.06-.06a1.65 1.65 0 0 0-1.82-.33 1.65 1.65 0 0 0-1 1.51V21a2 2 0 0 1-2 2 2 2 0 0 1-2-2v-.09A1.65 1.65 0 0 0 9 19.4a1.65 1.65 0 0 0-1.82.33l-.06.06a2 2 0 0 1-2.83 0 2 2 0 0 1 0-2.83l.06-.06a1.65 1.65 0 0 0 .33-1.82 1.65 1.65 0 0 0-1.51-1H3a2 2 0 0 1-2-2 2 2 0 0 1 2-2h.09A1.65 1.65 0 0 0 4.6 9a1.65 1.65 0 0 0-.33-1.82l-.06-.06a2 2 0 0 1 0-2.83 2 2 0 0 1 2.83 0l.06.06a1.65 1.65 0 0 0 1.82.33H9a1.65 1.65 0 0 0 1-1.51V3a2 2 0 0 1 2-2 2 2 0 0 1 2 2v.09a1.65 1.65 0 0 0 1 1.51 1.65 1.65 0 0 0 1.82-.33l.06-.06a2 2 0 0 1 2.83 0 2 2 0 0 1 0 2.83l-.06.06a1.65 1.65 0 0 0-.33 1.82V9a1.65 1.65 0 0 0 1.51 1H21a2 2 0 0 1 2 2 2 2 0 0 1-2 2h-.09a1.65 1.65 0 0 0-1.51 1z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiShare")]
///This icon requires the feature `FiShare` to be enabled.
#[component]
pub fn FiShare(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M4 12v8a2 2 0 0 0 2 2h12a2 2 0 0 0 2-2v-8\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"16 6 12 2 8 6\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"2\" x2=\"12\" y2=\"15\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiShare2")]
///This icon requires the feature `FiShare2` to be enabled.
#[component]
pub fn FiShare2(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"18\" cy=\"5\" r=\"3\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"6\" cy=\"12\" r=\"3\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"18\" cy=\"19\" r=\"3\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"8.59\" y1=\"13.51\" x2=\"15.42\" y2=\"17.49\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"15.41\" y1=\"6.51\" x2=\"8.59\" y2=\"10.49\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiShield")]
///This icon requires the feature `FiShield` to be enabled.
#[component]
pub fn FiShield(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M12 22s8-4 8-10V5l-8-3-8 3v7c0 6 8 10 8 10z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiShieldOff")]
///This icon requires the feature `FiShieldOff` to be enabled.
#[component]
pub fn FiShieldOff(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M19.69 14a6.9 6.9 0 0 0 .31-2V5l-8-3-3.16 1.18\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M4.73 4.73L4 5v7c0 6 8 10 8 10a20.29 20.29 0 0 0 5.62-4.38\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"1\" y1=\"1\" x2=\"23\" y2=\"23\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiShoppingBag")]
///This icon requires the feature `FiShoppingBag` to be enabled.
#[component]
pub fn FiShoppingBag(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M6 2L3 6v14a2 2 0 0 0 2 2h14a2 2 0 0 0 2-2V6l-3-4z\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"3\" y1=\"6\" x2=\"21\" y2=\"6\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M16 10a4 4 0 0 1-8 0\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiShoppingCart")]
///This icon requires the feature `FiShoppingCart` to be enabled.
#[component]
pub fn FiShoppingCart(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"9\" cy=\"21\" r=\"1\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"20\" cy=\"21\" r=\"1\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M1 1h4l2.68 13.39a2 2 0 0 0 2 1.61h9.72a2 2 0 0 0 2-1.61L23 6H6\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiShuffle")]
///This icon requires the feature `FiShuffle` to be enabled.
#[component]
pub fn FiShuffle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"16 3 21 3 21 8\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"4\" y1=\"20\" x2=\"21\" y2=\"3\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"21 16 21 21 16 21\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"15\" y1=\"15\" x2=\"21\" y2=\"21\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"4\" y1=\"4\" x2=\"9\" y2=\"9\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiSidebar")]
///This icon requires the feature `FiSidebar` to be enabled.
#[component]
pub fn FiSidebar(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"9\" y1=\"3\" x2=\"9\" y2=\"21\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiSkipBack")]
///This icon requires the feature `FiSkipBack` to be enabled.
#[component]
pub fn FiSkipBack(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"19 20 9 12 19 4 19 20\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"5\" y1=\"19\" x2=\"5\" y2=\"5\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiSkipForward")]
///This icon requires the feature `FiSkipForward` to be enabled.
#[component]
pub fn FiSkipForward(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"5 4 15 12 5 20 5 4\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"19\" y1=\"5\" x2=\"19\" y2=\"19\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiSlack")]
///This icon requires the feature `FiSlack` to be enabled.
#[component]
pub fn FiSlack(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M14.5 10c-.83 0-1.5-.67-1.5-1.5v-5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5v5c0 .83-.67 1.5-1.5 1.5z\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M20.5 10H19V8.5c0-.83.67-1.5 1.5-1.5s1.5.67 1.5 1.5-.67 1.5-1.5 1.5z\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M9.5 14c.83 0 1.5.67 1.5 1.5v5c0 .83-.67 1.5-1.5 1.5S8 21.33 8 20.5v-5c0-.83.67-1.5 1.5-1.5z\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M3.5 14H5v1.5c0 .83-.67 1.5-1.5 1.5S2 16.33 2 15.5 2.67 14 3.5 14z\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M14 14.5c0-.83.67-1.5 1.5-1.5h5c.83 0 1.5.67 1.5 1.5s-.67 1.5-1.5 1.5h-5c-.83 0-1.5-.67-1.5-1.5z\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M15.5 19H14v1.5c0 .83.67 1.5 1.5 1.5s1.5-.67 1.5-1.5-.67-1.5-1.5-1.5z\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M10 9.5C10 8.67 9.33 8 8.5 8h-5C2.67 8 2 8.67 2 9.5S2.67 11 3.5 11h5c.83 0 1.5-.67 1.5-1.5z\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M8.5 5H10V3.5C10 2.67 9.33 2 8.5 2S7 2.67 7 3.5 7.67 5 8.5 5z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiSlash")]
///This icon requires the feature `FiSlash` to be enabled.
#[component]
pub fn FiSlash(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"4.93\" y1=\"4.93\" x2=\"19.07\" y2=\"19.07\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiSliders")]
///This icon requires the feature `FiSliders` to be enabled.
#[component]
pub fn FiSliders(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"4\" y1=\"21\" x2=\"4\" y2=\"14\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"4\" y1=\"10\" x2=\"4\" y2=\"3\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"21\" x2=\"12\" y2=\"12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"8\" x2=\"12\" y2=\"3\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"20\" y1=\"21\" x2=\"20\" y2=\"16\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"20\" y1=\"12\" x2=\"20\" y2=\"3\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"1\" y1=\"14\" x2=\"7\" y2=\"14\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"9\" y1=\"8\" x2=\"15\" y2=\"8\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"17\" y1=\"16\" x2=\"23\" y2=\"16\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiSmartphone")]
///This icon requires the feature `FiSmartphone` to be enabled.
#[component]
pub fn FiSmartphone(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"5\" y=\"2\" width=\"14\" height=\"20\" rx=\"2\" ry=\"2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"18\" x2=\"12.01\" y2=\"18\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiSmile")]
///This icon requires the feature `FiSmile` to be enabled.
#[component]
pub fn FiSmile(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) viewBox =
        "0 0 24 24" width = "24" height = "24" fill = "none" stroke = "currentColor"
        stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width
        = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M8 14s1.5 2 4 2 4-2 4-2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"9\" y1=\"9\" x2=\"9.01\" y2=\"9\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"15\" y1=\"9\" x2=\"15.01\" y2=\"9\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiSpeaker")]
///This icon requires the feature `FiSpeaker` to be enabled.
#[component]
pub fn FiSpeaker(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"4\" y=\"2\" width=\"16\" height=\"20\" rx=\"2\" ry=\"2\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"14\" r=\"4\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"6\" x2=\"12.01\" y2=\"6\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiSquare")]
///This icon requires the feature `FiSquare` to be enabled.
#[component]
pub fn FiSquare(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiStar")]
///This icon requires the feature `FiStar` to be enabled.
#[component]
pub fn FiStar(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"12 2 15.09 8.26 22 9.27 17 14.14 18.18 21.02 12 17.77 5.82 21.02 7 14.14 2 9.27 8.91 8.26 12 2\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiStopCircle")]
///This icon requires the feature `FiStopCircle` to be enabled.
#[component]
pub fn FiStopCircle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><rect xmlns=\"http://www.w3.org/2000/svg\" x=\"9\" y=\"9\" width=\"6\" height=\"6\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiSun")]
///This icon requires the feature `FiSun` to be enabled.
#[component]
pub fn FiSun(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"5\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"1\" x2=\"12\" y2=\"3\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"21\" x2=\"12\" y2=\"23\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"4.22\" y1=\"4.22\" x2=\"5.64\" y2=\"5.64\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"18.36\" y1=\"18.36\" x2=\"19.78\" y2=\"19.78\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"1\" y1=\"12\" x2=\"3\" y2=\"12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"21\" y1=\"12\" x2=\"23\" y2=\"12\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"4.22\" y1=\"19.78\" x2=\"5.64\" y2=\"18.36\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"18.36\" y1=\"5.64\" x2=\"19.78\" y2=\"4.22\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiSunrise")]
///This icon requires the feature `FiSunrise` to be enabled.
#[component]
pub fn FiSunrise(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M17 18a5 5 0 0 0-10 0\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"2\" x2=\"12\" y2=\"9\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"4.22\" y1=\"10.22\" x2=\"5.64\" y2=\"11.64\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"1\" y1=\"18\" x2=\"3\" y2=\"18\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"21\" y1=\"18\" x2=\"23\" y2=\"18\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"18.36\" y1=\"11.64\" x2=\"19.78\" y2=\"10.22\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"23\" y1=\"22\" x2=\"1\" y2=\"22\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"8 6 12 2 16 6\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiSunset")]
///This icon requires the feature `FiSunset` to be enabled.
#[component]
pub fn FiSunset(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M17 18a5 5 0 0 0-10 0\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"9\" x2=\"12\" y2=\"2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"4.22\" y1=\"10.22\" x2=\"5.64\" y2=\"11.64\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"1\" y1=\"18\" x2=\"3\" y2=\"18\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"21\" y1=\"18\" x2=\"23\" y2=\"18\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"18.36\" y1=\"11.64\" x2=\"19.78\" y2=\"10.22\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"23\" y1=\"22\" x2=\"1\" y2=\"22\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"16 5 12 9 8 5\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiTable")]
///This icon requires the feature `FiTable` to be enabled.
#[component]
pub fn FiTable(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M9 3H5a2 2 0 0 0-2 2v4m6-6h10a2 2 0 0 1 2 2v4M9 3v18m0 0h10a2 2 0 0 0 2-2V9M9 21H5a2 2 0 0 1-2-2V9m0 0h18\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiTablet")]
///This icon requires the feature `FiTablet` to be enabled.
#[component]
pub fn FiTablet(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"4\" y=\"2\" width=\"16\" height=\"20\" rx=\"2\" ry=\"2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"18\" x2=\"12.01\" y2=\"18\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiTag")]
///This icon requires the feature `FiTag` to be enabled.
#[component]
pub fn FiTag(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M20.59 13.41l-7.17 7.17a2 2 0 0 1-2.83 0L2 12V2h10l8.59 8.59a2 2 0 0 1 0 2.82z\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"7\" y1=\"7\" x2=\"7.01\" y2=\"7\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiTarget")]
///This icon requires the feature `FiTarget` to be enabled.
#[component]
pub fn FiTarget(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"6\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"2\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiTerminal")]
///This icon requires the feature `FiTerminal` to be enabled.
#[component]
pub fn FiTerminal(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"4 17 10 11 4 5\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"19\" x2=\"20\" y2=\"19\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiThermometer")]
///This icon requires the feature `FiThermometer` to be enabled.
#[component]
pub fn FiThermometer(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M14 14.76V3.5a2.5 2.5 0 0 0-5 0v11.26a4.5 4.5 0 1 0 5 0z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiThumbsDown")]
///This icon requires the feature `FiThumbsDown` to be enabled.
#[component]
pub fn FiThumbsDown(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M10 15v4a3 3 0 0 0 3 3l4-9V2H5.72a2 2 0 0 0-2 1.7l-1.38 9a2 2 0 0 0 2 2.3zm7-13h2.67A2.31 2.31 0 0 1 22 4v7a2.31 2.31 0 0 1-2.33 2H17\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiThumbsUp")]
///This icon requires the feature `FiThumbsUp` to be enabled.
#[component]
pub fn FiThumbsUp(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M14 9V5a3 3 0 0 0-3-3l-4 9v11h11.28a2 2 0 0 0 2-1.7l1.38-9a2 2 0 0 0-2-2.3zM7 22H4a2 2 0 0 1-2-2v-7a2 2 0 0 1 2-2h3\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiToggleLeft")]
///This icon requires the feature `FiToggleLeft` to be enabled.
#[component]
pub fn FiToggleLeft(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"1\" y=\"5\" width=\"22\" height=\"14\" rx=\"7\" ry=\"7\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"8\" cy=\"12\" r=\"3\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiToggleRight")]
///This icon requires the feature `FiToggleRight` to be enabled.
#[component]
pub fn FiToggleRight(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"1\" y=\"5\" width=\"22\" height=\"14\" rx=\"7\" ry=\"7\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"16\" cy=\"12\" r=\"3\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiTool")]
///This icon requires the feature `FiTool` to be enabled.
#[component]
pub fn FiTool(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M14.7 6.3a1 1 0 0 0 0 1.4l1.6 1.6a1 1 0 0 0 1.4 0l3.77-3.77a6 6 0 0 1-7.94 7.94l-6.91 6.91a2.12 2.12 0 0 1-3-3l6.91-6.91a6 6 0 0 1 7.94-7.94l-3.76 3.76z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiTrash")]
///This icon requires the feature `FiTrash` to be enabled.
#[component]
pub fn FiTrash(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"3 6 5 6 21 6\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiTrash2")]
///This icon requires the feature `FiTrash2` to be enabled.
#[component]
pub fn FiTrash2(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"3 6 5 6 21 6\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M19 6v14a2 2 0 0 1-2 2H7a2 2 0 0 1-2-2V6m3 0V4a2 2 0 0 1 2-2h4a2 2 0 0 1 2 2v2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"10\" y1=\"11\" x2=\"10\" y2=\"17\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"14\" y1=\"11\" x2=\"14\" y2=\"17\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiTrello")]
///This icon requires the feature `FiTrello` to be enabled.
#[component]
pub fn FiTrello(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) viewBox =
        "0 0 24 24" width = "24" height = "24" fill = "none" stroke = "currentColor"
        stroke - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width
        = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" /><rect xmlns=\"http://www.w3.org/2000/svg\" x=\"7\" y=\"7\" width=\"3\" height=\"9\" /><rect xmlns=\"http://www.w3.org/2000/svg\" x=\"14\" y=\"7\" width=\"3\" height=\"5\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiTrendingDown")]
///This icon requires the feature `FiTrendingDown` to be enabled.
#[component]
pub fn FiTrendingDown(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"23 18 13.5 8.5 8.5 13.5 1 6\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"17 18 23 18 23 12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiTrendingUp")]
///This icon requires the feature `FiTrendingUp` to be enabled.
#[component]
pub fn FiTrendingUp(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"23 6 13.5 15.5 8.5 10.5 1 18\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"17 6 23 6 23 12\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiTriangle")]
///This icon requires the feature `FiTriangle` to be enabled.
#[component]
pub fn FiTriangle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M10.29 3.86L1.82 18a2 2 0 0 0 1.71 3h16.94a2 2 0 0 0 1.71-3L13.71 3.86a2 2 0 0 0-3.42 0z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiTruck")]
///This icon requires the feature `FiTruck` to be enabled.
#[component]
pub fn FiTruck(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"1\" y=\"3\" width=\"15\" height=\"13\" /><polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"16 8 20 8 23 11 23 16 16 16 16 8\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"5.5\" cy=\"18.5\" r=\"2.5\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"18.5\" cy=\"18.5\" r=\"2.5\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiTv")]
///This icon requires the feature `FiTv` to be enabled.
#[component]
pub fn FiTv(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"2\" y=\"7\" width=\"20\" height=\"15\" rx=\"2\" ry=\"2\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"17 2 12 7 7 2\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiTwitch")]
///This icon requires the feature `FiTwitch` to be enabled.
#[component]
pub fn FiTwitch(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M21 2H3v16h5v4l4-4h5l4-4V2zM11 11V7M16 11V7\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiTwitter")]
///This icon requires the feature `FiTwitter` to be enabled.
#[component]
pub fn FiTwitter(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M23 3a10.9 10.9 0 0 1-3.14 1.53 4.48 4.48 0 0 0-7.86 3v1A10.66 10.66 0 0 1 3 4s-4 9 5 13a11.64 11.64 0 0 1-7 2c9 5 20 0 20-11.5a4.5 4.5 0 0 0-.08-.83A7.72 7.72 0 0 0 23 3z\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiType")]
///This icon requires the feature `FiType` to be enabled.
#[component]
pub fn FiType(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"4 7 4 4 20 4 20 7\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"9\" y1=\"20\" x2=\"15\" y2=\"20\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"4\" x2=\"12\" y2=\"20\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiUmbrella")]
///This icon requires the feature `FiUmbrella` to be enabled.
#[component]
pub fn FiUmbrella(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M23 12a11.05 11.05 0 0 0-22 0zm-5 7a3 3 0 0 1-6 0v-7\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiUnderline")]
///This icon requires the feature `FiUnderline` to be enabled.
#[component]
pub fn FiUnderline(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M6 3v7a6 6 0 0 0 6 6 6 6 0 0 0 6-6V3\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"4\" y1=\"21\" x2=\"20\" y2=\"21\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiUnlock")]
///This icon requires the feature `FiUnlock` to be enabled.
#[component]
pub fn FiUnlock(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"3\" y=\"11\" width=\"18\" height=\"11\" rx=\"2\" ry=\"2\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M7 11V7a5 5 0 0 1 9.9-1\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiUpload")]
///This icon requires the feature `FiUpload` to be enabled.
#[component]
pub fn FiUpload(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"17 8 12 3 7 8\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"3\" x2=\"12\" y2=\"15\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiUploadCloud")]
///This icon requires the feature `FiUploadCloud` to be enabled.
#[component]
pub fn FiUploadCloud(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"16 16 12 12 8 16\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"12\" x2=\"12\" y2=\"21\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M20.39 18.39A5 5 0 0 0 18 9h-1.26A8 8 0 1 0 3 16.3\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"16 16 12 12 8 16\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiUser")]
///This icon requires the feature `FiUser` to be enabled.
#[component]
pub fn FiUser(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M20 21v-2a4 4 0 0 0-4-4H8a4 4 0 0 0-4 4v2\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"7\" r=\"4\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiUserCheck")]
///This icon requires the feature `FiUserCheck` to be enabled.
#[component]
pub fn FiUserCheck(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"8.5\" cy=\"7\" r=\"4\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"17 11 19 13 23 9\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiUserMinus")]
///This icon requires the feature `FiUserMinus` to be enabled.
#[component]
pub fn FiUserMinus(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"8.5\" cy=\"7\" r=\"4\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"23\" y1=\"11\" x2=\"17\" y2=\"11\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiUserPlus")]
///This icon requires the feature `FiUserPlus` to be enabled.
#[component]
pub fn FiUserPlus(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"8.5\" cy=\"7\" r=\"4\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"20\" y1=\"8\" x2=\"20\" y2=\"14\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"23\" y1=\"11\" x2=\"17\" y2=\"11\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiUserX")]
///This icon requires the feature `FiUserX` to be enabled.
#[component]
pub fn FiUserX(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M16 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"8.5\" cy=\"7\" r=\"4\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"18\" y1=\"8\" x2=\"23\" y2=\"13\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"23\" y1=\"8\" x2=\"18\" y2=\"13\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiUsers")]
///This icon requires the feature `FiUsers` to be enabled.
#[component]
pub fn FiUsers(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M17 21v-2a4 4 0 0 0-4-4H5a4 4 0 0 0-4 4v2\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"9\" cy=\"7\" r=\"4\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M23 21v-2a4 4 0 0 0-3-3.87\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M16 3.13a4 4 0 0 1 0 7.75\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiVideo")]
///This icon requires the feature `FiVideo` to be enabled.
#[component]
pub fn FiVideo(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"23 7 16 12 23 17 23 7\" /><rect xmlns=\"http://www.w3.org/2000/svg\" x=\"1\" y=\"5\" width=\"15\" height=\"14\" rx=\"2\" ry=\"2\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiVideoOff")]
///This icon requires the feature `FiVideoOff` to be enabled.
#[component]
pub fn FiVideoOff(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M16 16v1a2 2 0 0 1-2 2H3a2 2 0 0 1-2-2V7a2 2 0 0 1 2-2h2m5.66 0H14a2 2 0 0 1 2 2v3.34l1 1L23 7v10\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"1\" y1=\"1\" x2=\"23\" y2=\"23\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiVoicemail")]
///This icon requires the feature `FiVoicemail` to be enabled.
#[component]
pub fn FiVoicemail(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"5.5\" cy=\"11.5\" r=\"4.5\" /><circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"18.5\" cy=\"11.5\" r=\"4.5\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"5.5\" y1=\"16\" x2=\"18.5\" y2=\"16\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiVolume")]
///This icon requires the feature `FiVolume` to be enabled.
#[component]
pub fn FiVolume(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"11 5 6 9 2 9 2 15 6 15 11 19 11 5\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiVolume1")]
///This icon requires the feature `FiVolume1` to be enabled.
#[component]
pub fn FiVolume1(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"11 5 6 9 2 9 2 15 6 15 11 19 11 5\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M15.54 8.46a5 5 0 0 1 0 7.07\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiVolume2")]
///This icon requires the feature `FiVolume2` to be enabled.
#[component]
pub fn FiVolume2(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"11 5 6 9 2 9 2 15 6 15 11 19 11 5\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M19.07 4.93a10 10 0 0 1 0 14.14M15.54 8.46a5 5 0 0 1 0 7.07\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiVolumeX")]
///This icon requires the feature `FiVolumeX` to be enabled.
#[component]
pub fn FiVolumeX(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"11 5 6 9 2 9 2 15 6 15 11 19 11 5\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"23\" y1=\"9\" x2=\"17\" y2=\"15\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"17\" y1=\"9\" x2=\"23\" y2=\"15\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiWatch")]
///This icon requires the feature `FiWatch` to be enabled.
#[component]
pub fn FiWatch(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"7\" /><polyline xmlns=\"http://www.w3.org/2000/svg\" points=\"12 9 12 12 13.5 13.5\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M16.51 17.35l-.35 3.83a2 2 0 0 1-2 1.82H9.83a2 2 0 0 1-2-1.82l-.35-3.83m.01-10.7l.35-3.83A2 2 0 0 1 9.83 1h4.35a2 2 0 0 1 2 1.82l.35 3.83\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiWifi")]
///This icon requires the feature `FiWifi` to be enabled.
#[component]
pub fn FiWifi(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M5 12.55a11 11 0 0 1 14.08 0\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M1.42 9a16 16 0 0 1 21.16 0\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M8.53 16.11a6 6 0 0 1 6.95 0\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"20\" x2=\"12.01\" y2=\"20\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiWifiOff")]
///This icon requires the feature `FiWifiOff` to be enabled.
#[component]
pub fn FiWifiOff(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"1\" y1=\"1\" x2=\"23\" y2=\"23\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M16.72 11.06A10.94 10.94 0 0 1 19 12.55\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M5 12.55a10.94 10.94 0 0 1 5.17-2.39\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M10.71 5.05A16 16 0 0 1 22.58 9\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M1.42 9a15.91 15.91 0 0 1 4.7-2.88\" /><path xmlns=\"http://www.w3.org/2000/svg\" d=\"M8.53 16.11a6 6 0 0 1 6.95 0\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"12\" y1=\"20\" x2=\"12.01\" y2=\"20\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiWind")]
///This icon requires the feature `FiWind` to be enabled.
#[component]
pub fn FiWind(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M9.59 4.59A2 2 0 1 1 11 8H2m10.59 11.41A2 2 0 1 0 14 16H2m15.73-8.27A2.5 2.5 0 1 1 19.5 12H2\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiX")]
///This icon requires the feature `FiX` to be enabled.
#[component]
pub fn FiX(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<line xmlns=\"http://www.w3.org/2000/svg\" x1=\"18\" y1=\"6\" x2=\"6\" y2=\"18\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"6\" y1=\"6\" x2=\"18\" y2=\"18\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiXCircle")]
///This icon requires the feature `FiXCircle` to be enabled.
#[component]
pub fn FiXCircle(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<circle xmlns=\"http://www.w3.org/2000/svg\" cx=\"12\" cy=\"12\" r=\"10\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"15\" y1=\"9\" x2=\"9\" y2=\"15\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"9\" y1=\"9\" x2=\"15\" y2=\"15\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiXOctagon")]
///This icon requires the feature `FiXOctagon` to be enabled.
#[component]
pub fn FiXOctagon(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"7.86 2 16.14 2 22 7.86 22 16.14 16.14 22 7.86 22 2 16.14 2 7.86 7.86 2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"15\" y1=\"9\" x2=\"9\" y2=\"15\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"9\" y1=\"9\" x2=\"15\" y2=\"15\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiXSquare")]
///This icon requires the feature `FiXSquare` to be enabled.
#[component]
pub fn FiXSquare(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<rect xmlns=\"http://www.w3.org/2000/svg\" x=\"3\" y=\"3\" width=\"18\" height=\"18\" rx=\"2\" ry=\"2\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"9\" y1=\"9\" x2=\"15\" y2=\"15\" /><line xmlns=\"http://www.w3.org/2000/svg\" x1=\"15\" y1=\"9\" x2=\"9\" y2=\"15\" />"
        > < title > { title } < / title > < / svg >
    }
}
#[cfg(feature = "FiYoutube")]
///This icon requires the feature `FiYoutube` to be enabled.
#[component]
pub fn FiYoutube(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon). Defaults to "1em".
    #[prop(into, optional, default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into, optional)]
    class: String,
    /// Color of the icon. For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into, optional, default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into, optional)]
    style: String,
    /// Accessibility title.
    #[prop(into, optional)]
    title: String,
) -> impl IntoView {
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_width = "0" style = format!("{} color: {};", style, color) width = "24"
        height = "24" viewBox = "0 0 24 24" fill = "none" stroke = "currentColor" stroke
        - width = "2" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" inner_html =
        "<path xmlns=\"http://www.w3.org/2000/svg\" d=\"M22.54 6.42a2.78 2.78 0 0 0-1.94-2C18.88 4 12 4 12 4s-6.88 0-8.6.46a2.78 2.78 0 0 0-1.94 2A29 29 0 0 0 1 11.75a29 29 0 0 0 .46 5.33A2.78 2.78 0 0 0 3.4 19c1.72.46 8.6.46 8.6.46s6.88 0 8.6-.46a2.78 2.78 0 0 0 1.94-2 29 29 0 0 0 .46-5.25 29 29 0 0 0-.46-5.33z\" /><polygon xmlns=\"http://www.w3.org/2000/svg\" points=\"9.75 15.02 15.5 11.75 9.75 8.48 9.75 15.02\" />"
        > < title > { title } < / title > < / svg >
    }
}
