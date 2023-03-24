#[cfg(feature = "HiLgSolidArrowUturnUp")]
use leptos::*;
#[cfg(feature = "HiLgSolidArrowUturnUp")]
///This icon requires the feature `HiLgSolidArrowUturnUp` to be enabled.
#[component]
pub fn ArrowUturnUp(
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
        "M21.5303 9.53033C21.2374 9.82322 20.7626 9.82322 20.4697 9.53033L15.75 4.81066V15C15.75 18.7279 12.7279 21.75 9 21.75C5.27208 21.75 2.25 18.7279 2.25 15L2.25 12C2.25 11.5858 2.58579 11.25 3 11.25C3.41421 11.25 3.75 11.5858 3.75 12L3.75 15C3.75 17.8995 6.10051 20.25 9 20.25C11.8995 20.25 14.25 17.8995 14.25 15V4.81066L9.53033 9.53033C9.23744 9.82322 8.76256 9.82322 8.46967 9.53033C8.17678 9.23744 8.17678 8.76256 8.46967 8.46967L14.4697 2.46967C14.7626 2.17678 15.2374 2.17678 15.5303 2.46967L21.5303 8.46967C21.8232 8.76256 21.8232 9.23744 21.5303 9.53033Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
