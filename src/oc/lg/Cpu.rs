#[cfg(feature = "OcLgCpu")]
use leptos::*;
#[cfg(feature = "OcLgCpu")]
///This icon requires the feature `OcLgCpu` to be enabled.
#[component]
pub fn Cpu(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M8.75 8h6.5a.75.75 0 0 1 .75.75v6.5a.75.75 0 0 1-.75.75h-6.5a.75.75 0 0 1-.75-.75v-6.5A.75.75 0 0 1 8.75 8Zm.75 6.5h5v-5h-5Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.25 1a.75.75 0 0 1 .75.75V4h2.25c.966 0 1.75.784 1.75 1.75V8h2.25a.75.75 0 0 1 0 1.5H20v5h2.25a.75.75 0 0 1 0 1.5H20v2.25A1.75 1.75 0 0 1 18.25 20H16v2.25a.75.75 0 0 1-1.5 0V20h-5v2.25a.75.75 0 0 1-1.5 0V20H5.75A1.75 1.75 0 0 1 4 18.25V16H1.75a.75.75 0 0 1 0-1.5H4v-5H1.75a.75.75 0 0 1 0-1.5H4V5.75C4 4.784 4.784 4 5.75 4H8V1.75a.75.75 0 0 1 1.5 0V4h5V1.75a.75.75 0 0 1 .75-.75Zm3 17.5a.25.25 0 0 0 .25-.25V5.75a.25.25 0 0 0-.25-.25H5.75a.25.25 0 0 0-.25.25v12.5c0 .138.112.25.25.25Z"
        /> < title > { title } < / title > < / svg >
    }
}
