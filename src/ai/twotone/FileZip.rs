#[cfg(feature = "AiTwotoneFileZip")]
use leptos::*;
#[cfg(feature = "AiTwotoneFileZip")]
///This icon requires the feature `AiTwotoneFileZip` to be enabled.
#[component]
pub fn FileZip(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 1024 1024" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" fill = "#D9D9D9" d = "M344 630h32v2h-32z" />< path
        xmlns = "http://www.w3.org/2000/svg" fill = "#D9D9D9" d =
        "M534 352V136H360v64h64v64h-64v64h64v64h-64v64h64v64h-64v62h64v160H296V520h64v-64h-64v-64h64v-64h-64v-64h64v-64h-64v-64h-64v752h560V394H576a42 42 0 0 1-42-42z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M854.6 288.6L639.4 73.4c-6-6-14.1-9.4-22.6-9.4H192c-17.7 0-32 14.3-32 32v832c0 17.7 14.3 32 32 32h640c17.7 0 32-14.3 32-32V311.3c0-8.5-3.4-16.7-9.4-22.7zM602 137.8L790.2 326H602V137.8zM792 888H232V136h64v64h64v-64h174v216a42 42 0 0 0 42 42h216v494z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M296 392h64v64h-64zm0-128h64v64h-64zm0 318v160h128V582h-64v-62h-64v62zm48 50v-2h32v64h-32v-62zm16-432h64v64h-64zm0 256h64v64h-64zm0-128h64v64h-64z"
        /> < title > { title } < / title > < / svg >
    }
}
