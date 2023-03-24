#[cfg(feature = "ImHeartBroken")]
use leptos::*;
#[cfg(feature = "ImHeartBroken")]
///This icon requires the feature `ImHeartBroken` to be enabled.
#[component]
pub fn HeartBroken(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M11.8 1c2.318 0 4.2 1.882 4.2 4.2 0 4.566-4.935 5.982-8 10.616-3.243-4.663-8-5.9-8-10.616 0-2.319 1.882-4.2 4.2-4.2 0.943 0 1.812 0.43 2.512 1.060l-1.213 1.94 3.5 2-2 5 5.5-6-3.5-2 0.967-1.451c0.553-0.34 1.175-0.549 1.833-0.549z"
        /> < title > { title } < / title > < / svg >
    }
}
