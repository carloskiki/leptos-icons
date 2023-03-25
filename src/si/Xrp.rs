#[cfg(feature = "SiXrp")]
use leptos::*;
#[cfg(feature = "SiXrp")]
///This icon requires the feature `SiXrp` to be enabled.
#[component]
pub fn Xrp(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
    color: String,
    /// HTML style attribute.
    #[prop(into)]
    #[prop(optional)]
    style: String,
    /// Accessibility title.
    #[prop(into)]
    #[prop(optional)]
    title: String,
) -> impl IntoView {
    let style = format!("{} color: {};", style, color);
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M5.52 2.955A3.521 3.521 0 001.996 6.48v2.558A2.12 2.12 0 010 11.157l.03.562-.03.561a2.12 2.12 0 011.996 2.121v2.948a3.69 3.69 0 003.68 3.696v-1.123a2.56 2.56 0 01-2.557-2.558v-2.963a3.239 3.239 0 00-1.42-2.682 3.26 3.26 0 001.42-2.682V6.48A2.412 2.412 0 015.52 4.078h.437V2.955zm12.538 0v1.123h.437a2.39 2.39 0 012.386 2.401v2.558a3.26 3.26 0 001.42 2.682 3.239 3.239 0 00-1.42 2.682v2.963a2.56 2.56 0 01-2.557 2.558v1.123a3.69 3.69 0 003.68-3.696V14.4A2.12 2.12 0 0124 12.281l-.03-.562.03-.561a2.12 2.12 0 01-1.996-2.12V6.478a3.518 3.518 0 00-3.509-3.524zM6.253 7.478l3.478 3.259a3.393 3.393 0 004.553 0l3.478-3.26h-1.669l-2.65 2.464a2.133 2.133 0 01-2.886 0L7.922 7.478zm5.606 4.884a3.36 3.36 0 00-2.128.886l-3.493 3.274h1.668l2.667-2.495a2.133 2.133 0 012.885 0l2.65 2.495h1.67l-3.494-3.274a3.36 3.36 0 00-2.425-.886z"
        /> < title > { title } < / title > < / svg >
    }
}
