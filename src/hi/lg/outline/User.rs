#[cfg(feature = "HiLgOutlineUser")]
use leptos::*;
#[cfg(feature = "HiLgOutlineUser")]
///This icon requires the feature `HiLgOutlineUser` to be enabled.
#[component]
pub fn User(
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
        "M15.75 6C15.75 8.07107 14.071 9.75 12 9.75C9.9289 9.75 8.24996 8.07107 8.24996 6C8.24996 3.92893 9.9289 2.25 12 2.25C14.071 2.25 15.75 3.92893 15.75 6Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4.5011 20.1182C4.5714 16.0369 7.90184 12.75 12 12.75C16.0982 12.75 19.4287 16.0371 19.4988 20.1185C17.216 21.166 14.6764 21.75 12.0003 21.75C9.32396 21.75 6.78406 21.1659 4.5011 20.1182Z"
        stroke = "#0F172A" stroke - width = "1.5" stroke - linecap = "round" stroke -
        linejoin = "round" /> < title > { title } < / title > < / svg >
    }
}
