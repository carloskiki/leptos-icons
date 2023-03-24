#[cfg(feature = "ImOpt")]
use leptos::*;
#[cfg(feature = "ImOpt")]
///This icon requires the feature `ImOpt` to be enabled.
#[component]
pub fn Opt(
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
        "M14.5 13h-4c-0.198 0-0.377-0.116-0.457-0.297l-3.868-8.703h-4.675c-0.276 0-0.5-0.224-0.5-0.5s0.224-0.5 0.5-0.5h5c0.198 0 0.377 0.116 0.457 0.297l3.868 8.703h3.675c0.276 0 0.5 0.224 0.5 0.5s-0.224 0.5-0.5 0.5z"
        />< path xmlns = "http://www.w3.org/2000/svg" xmlns : xlink =
        "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M14.5 4h-5c-0.276 0-0.5-0.224-0.5-0.5s0.224-0.5 0.5-0.5h5c0.276 0 0.5 0.224 0.5 0.5s-0.224 0.5-0.5 0.5z"
        /> < title > { title } < / title > < / svg >
    }
}
