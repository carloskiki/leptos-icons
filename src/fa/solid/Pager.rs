#[cfg(feature = "FaSolidPager")]
use leptos::*;
#[cfg(feature = "FaSolidPager")]
///This icon requires the feature `FaSolidPager` to be enabled.
#[component]
pub fn Pager(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M0 128C0 92.7 28.7 64 64 64H448c35.3 0 64 28.7 64 64V384c0 35.3-28.7 64-64 64H64c-35.3 0-64-28.7-64-64V128zm64 32v64c0 17.7 14.3 32 32 32H416c17.7 0 32-14.3 32-32V160c0-17.7-14.3-32-32-32H96c-17.7 0-32 14.3-32 32zM80 320c-13.3 0-24 10.7-24 24s10.7 24 24 24h56c13.3 0 24-10.7 24-24s-10.7-24-24-24H80zm136 0c-13.3 0-24 10.7-24 24s10.7 24 24 24h48c13.3 0 24-10.7 24-24s-10.7-24-24-24H216z"
        /> < title > { title } < / title > < / svg >
    }
}
