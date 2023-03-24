#[cfg(feature = "CgSortAz")]
use leptos::*;
#[cfg(feature = "CgSortAz")]
///This icon requires the feature `CgSortAz` to be enabled.
#[component]
pub fn SortAz(
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
        "M6 8C6 7.44772 6.44772 7 7 7H17C17.5523 7 18 7.44772 18 8C18 8.55228 17.5523 9 17 9H7C6.44772 9 6 8.55228 6 8Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 12C8 11.4477 8.44772 11 9 11H15C15.5523 11 16 11.4477 16 12C16 12.5523 15.5523 13 15 13H9C8.44772 13 8 12.5523 8 12Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11 15C10.4477 15 10 15.4477 10 16C10 16.5523 10.4477 17 11 17H13C13.5523 17 14 16.5523 14 16C14 15.4477 13.5523 15 13 15H11Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
