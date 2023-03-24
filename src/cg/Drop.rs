#[cfg(feature = "CgDrop")]
use leptos::*;
#[cfg(feature = "CgDrop")]
///This icon requires the feature `CgDrop` to be enabled.
#[component]
pub fn Drop(
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
        "M6.34315 19.5208C3.21895 16.3966 3.21895 11.3312 6.34315 8.20705L12 2.5502L17.6569 8.20705C20.781 11.3312 20.781 16.3966 17.6569 19.5208C14.5327 22.645 9.46734 22.645 6.34315 19.5208Z"
        stroke = "currentColor" stroke - width = "2" /> < title > { title } < / title > <
        / svg >
    }
}
