#[cfg(feature = "ImShuffle")]
use leptos::*;
#[cfg(feature = "ImShuffle")]
///This icon requires the feature `ImShuffle` to be enabled.
#[component]
pub fn Shuffle(
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
        "M12 11h-1.586l-2.5-2.5 2.5-2.5h1.586v2.5l3.5-3.5-3.5-3.5v2.5h-2c-0.265 0-0.52 0.105-0.707 0.293l-2.793 2.793-2.793-2.793c-0.188-0.188-0.442-0.293-0.707-0.293h-3v2h2.586l2.5 2.5-2.5 2.5h-2.586v2h3c0.265 0 0.52-0.105 0.707-0.293l2.793-2.793 2.793 2.793c0.188 0.188 0.442 0.293 0.707 0.293h2v2.5l3.5-3.5-3.5-3.5v2.5z"
        /> < title > { title } < / title > < / svg >
    }
}
