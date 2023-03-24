#[cfg(feature = "HiLgSolidArrowUpLeft")]
use leptos::*;
#[cfg(feature = "HiLgSolidArrowUpLeft")]
///This icon requires the feature `HiLgSolidArrowUpLeft` to be enabled.
#[component]
pub fn ArrowUpLeft(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M5.25 6.31066L5.25 15.75C5.25 16.1642 4.91421 16.5 4.5 16.5C4.08579 16.5 3.75 16.1642 3.75 15.75L3.75 4.5C3.75 4.30109 3.82902 4.11032 3.96967 3.96967C4.11032 3.82902 4.30109 3.75 4.5 3.75L15.75 3.75C16.1642 3.75 16.5 4.08579 16.5 4.5C16.5 4.91421 16.1642 5.25 15.75 5.25L6.31066 5.25L20.0303 18.9697C20.3232 19.2626 20.3232 19.7374 20.0303 20.0303C19.7374 20.3232 19.2626 20.3232 18.9697 20.0303L5.25 6.31066Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
