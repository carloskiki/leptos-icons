#[cfg(feature = "TbZoomInAreaFilled")]
use leptos::*;
#[cfg(feature = "TbZoomInAreaFilled")]
///This icon requires the feature `TbZoomInAreaFilled` to be enabled.
#[component]
pub fn ZoomInAreaFilled(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-zoom-in-area-filled" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 9a6 6 0 0 1 4.891 9.476l2.816 2.817a1 1 0 0 1 -1.32 1.497l-.094 -.083l-2.817 -2.816a6 6 0 0 1 -9.472 -4.666l-.004 -.225l.004 -.225a6 6 0 0 1 5.996 -5.775zm0 3a1 1 0 0 0 -.993 .883l-.007 .117v1h-1l-.117 .007a1 1 0 0 0 0 1.986l.117 .007h1v1l.007 .117a1 1 0 0 0 1.986 0l.007 -.117v-1h1l.117 -.007a1 1 0 0 0 0 -1.986l-.117 -.007h-1v-1l-.007 -.117a1 1 0 0 0 -.993 -.883z"
        stroke - width = "0" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M3 14a1 1 0 0 1 .993 .883l.007 .117v1a1 1 0 0 0 .883 .993l.117 .007h1a1 1 0 0 1 .117 1.993l-.117 .007h-1a3 3 0 0 1 -2.995 -2.824l-.005 -.176v-1a1 1 0 0 1 1 -1z"
        stroke - width = "0" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M3 9a1 1 0 0 1 .993 .883l.007 .117v1a1 1 0 0 1 -1.993 .117l-.007 -.117v-1a1 1 0 0 1 1 -1z"
        stroke - width = "0" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M6 2a1 1 0 0 1 .117 1.993l-.117 .007h-1a1 1 0 0 0 -.993 .883l-.007 .117v1a1 1 0 0 1 -1.993 .117l-.007 -.117v-1a3 3 0 0 1 2.824 -2.995l.176 -.005h1z"
        stroke - width = "0" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M11 2a1 1 0 0 1 .117 1.993l-.117 .007h-1a1 1 0 0 1 -.117 -1.993l.117 -.007h1z"
        stroke - width = "0" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M16 2a3 3 0 0 1 2.995 2.824l.005 .176v1a1 1 0 0 1 -1.993 .117l-.007 -.117v-1a1 1 0 0 0 -.883 -.993l-.117 -.007h-1a1 1 0 0 1 -.117 -1.993l.117 -.007h1z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
