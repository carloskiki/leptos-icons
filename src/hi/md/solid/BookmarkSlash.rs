#[cfg(feature = "HiMdSolidBookmarkSlash")]
use leptos::*;
#[cfg(feature = "HiMdSolidBookmarkSlash")]
///This icon requires the feature `HiMdSolidBookmarkSlash` to be enabled.
#[component]
pub fn BookmarkSlash(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M17 4.51661V13.818L5.43349 2.25151C6.93301 2.0853 8.45668 2 10 2C11.7163 2 13.4084 2.10551 15.07 2.31046C16.1942 2.44913 17 3.41374 17 4.51661Z"
        fill = "black" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 17.25V6.18198L13.6537 16.8357L10 15.0819L4.07455 17.9261C3.84215 18.0377 3.56875 18.0221 3.35057 17.8848C3.13239 17.7475 3 17.5078 3 17.25Z"
        fill = "black" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.28033 2.21967C2.98744 1.92678 2.51256 1.92678 2.21967 2.21967C1.92678 2.51256 1.92678 2.98744 2.21967 3.28033L16.7197 17.7803C17.0126 18.0732 17.4874 18.0732 17.7803 17.7803C18.0732 17.4874 18.0732 17.0126 17.7803 16.7197L3.28033 2.21967Z"
        fill = "black" /> < title > { title } < / title > < / svg >
    }
}
