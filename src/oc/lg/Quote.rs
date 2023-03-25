#[cfg(feature = "OcLgQuote")]
use leptos::*;
#[cfg(feature = "OcLgQuote")]
///This icon requires the feature `OcLgQuote` to be enabled.
#[component]
pub fn Quote(
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
        "M3 6.25a.75.75 0 0 1 .75-.75h13.5a.75.75 0 0 1 0 1.5H3.75A.75.75 0 0 1 3 6.25Zm5 6.063a.75.75 0 0 1 .75-.75h11.5a.75.75 0 0 1 0 1.5H8.75a.75.75 0 0 1-.75-.75Zm0 5.937a.75.75 0 0 1 .75-.75h11.5a.75.75 0 0 1 0 1.5H8.75a.75.75 0 0 1-.75-.75ZM3.75 11a.75.75 0 0 1 .75.75v7a.75.75 0 0 1-1.5 0v-7a.75.75 0 0 1 .75-.75Z"
        /> < title > { title } < / title > < / svg >
    }
}
