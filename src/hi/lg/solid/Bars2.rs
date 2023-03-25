#[cfg(feature = "HiLgSolidBars2")]
use leptos::*;
#[cfg(feature = "HiLgSolidBars2")]
///This icon requires the feature `HiLgSolidBars2` to be enabled.
#[component]
pub fn Bars2(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M3 9C3 8.58579 3.33579 8.25 3.75 8.25H20.25C20.6642 8.25 21 8.58579 21 9C21 9.41421 20.6642 9.75 20.25 9.75H3.75C3.33579 9.75 3 9.41421 3 9ZM3 15.75C3 15.3358 3.33579 15 3.75 15H20.25C20.6642 15 21 15.3358 21 15.75C21 16.1642 20.6642 16.5 20.25 16.5H3.75C3.33579 16.5 3 16.1642 3 15.75Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
