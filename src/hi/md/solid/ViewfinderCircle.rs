#[cfg(feature = "HiMdSolidViewfinderCircle")]
use leptos::*;
#[cfg(feature = "HiMdSolidViewfinderCircle")]
///This icon requires the feature `HiMdSolidViewfinderCircle` to be enabled.
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M4.25 2C3.00736 2 2 3.00736 2 4.25V6.25C2 6.66421 2.33579 7 2.75 7C3.16421 7 3.5 6.66421 3.5 6.25V4.25C3.5 3.83579 3.83579 3.5 4.25 3.5H6.25C6.66421 3.5 7 3.16421 7 2.75C7 2.33579 6.66421 2 6.25 2H4.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13.75 2C13.3358 2 13 2.33579 13 2.75C13 3.16421 13.3358 3.5 13.75 3.5H15.75C16.1642 3.5 16.5 3.83579 16.5 4.25V6.25C16.5 6.66421 16.8358 7 17.25 7C17.6642 7 18 6.66421 18 6.25V4.25C18 3.00736 16.9926 2 15.75 2H13.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.5 13.75C3.5 13.3358 3.16421 13 2.75 13C2.33579 13 2 13.3358 2 13.75V15.75C2 16.9926 3.00736 18 4.25 18H6.25C6.66421 18 7 17.6642 7 17.25C7 16.8358 6.66421 16.5 6.25 16.5H4.25C3.83579 16.5 3.5 16.1642 3.5 15.75V13.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 13.75C18 13.3358 17.6642 13 17.25 13C16.8358 13 16.5 13.3358 16.5 13.75V15.75C16.5 16.1642 16.1642 16.5 15.75 16.5H13.75C13.3358 16.5 13 16.8358 13 17.25C13 17.6642 13.3358 18 13.75 18H15.75C16.9926 18 18 16.9926 18 15.75V13.75Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 10C7 8.34315 8.34315 7 10 7C11.6569 7 13 8.34315 13 10C13 11.6569 11.6569 13 10 13C8.34315 13 7 11.6569 7 10Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
