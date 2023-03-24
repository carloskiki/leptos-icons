#[cfg(feature = "HiLgOutlineViewfinderCircle")]
use leptos::*;
#[cfg(feature = "HiLgOutlineViewfinderCircle")]
///This icon requires the feature `HiLgOutlineViewfinderCircle` to be enabled.
#[component]
pub fn ViewfinderCircle(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.5 3.75H6C4.75736 3.75 3.75 4.75736 3.75 6V7.5M16.5 3.75H18C19.2426 3.75 20.25 4.75736 20.25 6V7.5M20.25 16.5V18C20.25 19.2426 19.2426 20.25 18 20.25H16.5M7.5 20.25H6C4.75736 20.25 3.75 19.2426 3.75 18V16.5M15 12C15 13.6569 13.6569 15 12 15C10.3431 15 9 13.6569 9 12C9 10.3431 10.3431 9 12 9C13.6569 9 15 10.3431 15 12Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
