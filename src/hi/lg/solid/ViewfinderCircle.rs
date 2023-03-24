#[cfg(feature = "HiLgSolidViewfinderCircle")]
use leptos::*;
#[cfg(feature = "HiLgSolidViewfinderCircle")]
///This icon requires the feature `HiLgSolidViewfinderCircle` to be enabled.
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
        "M6 3C4.34315 3 3 4.34315 3 6V7.5C3 7.91421 3.33579 8.25 3.75 8.25C4.16421 8.25 4.5 7.91421 4.5 7.5V6C4.5 5.17157 5.17157 4.5 6 4.5H7.5C7.91421 4.5 8.25 4.16421 8.25 3.75C8.25 3.33579 7.91421 3 7.5 3H6Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.5 3C16.0858 3 15.75 3.33579 15.75 3.75C15.75 4.16421 16.0858 4.5 16.5 4.5H18C18.8284 4.5 19.5 5.17157 19.5 6V7.5C19.5 7.91421 19.8358 8.25 20.25 8.25C20.6642 8.25 21 7.91421 21 7.5V6C21 4.34315 19.6569 3 18 3H16.5Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 8.25C9.92893 8.25 8.25 9.92893 8.25 12C8.25 14.0711 9.92893 15.75 12 15.75C14.0711 15.75 15.75 14.0711 15.75 12C15.75 9.92893 14.0711 8.25 12 8.25Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4.5 16.5C4.5 16.0858 4.16421 15.75 3.75 15.75C3.33579 15.75 3 16.0858 3 16.5V18C3 19.6569 4.34315 21 6 21H7.5C7.91421 21 8.25 20.6642 8.25 20.25C8.25 19.8358 7.91421 19.5 7.5 19.5H6C5.17157 19.5 4.5 18.8284 4.5 18V16.5Z"
        fill = "#0F172A" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 16.5C21 16.0858 20.6642 15.75 20.25 15.75C19.8358 15.75 19.5 16.0858 19.5 16.5V18C19.5 18.8284 18.8284 19.5 18 19.5H16.5C16.0858 19.5 15.75 19.8358 15.75 20.25C15.75 20.6642 16.0858 21 16.5 21H18C19.6569 21 21 19.6569 21 18V16.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
