#[cfg(feature = "IoBookmarksSharp")]
use leptos::*;
#[cfg(feature = "IoBookmarksSharp")]
///This icon requires the feature `IoBookmarksSharp` to be enabled.
#[component]
pub fn BookmarksSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < polygon xmlns = "http://www.w3.org/2000/svg"
        points = "112 0 112 48 416 48 416 416 464 448 464 0 112 0" />< polygon xmlns =
        "http://www.w3.org/2000/svg" points = "48 80 48 512 216 388 384 512 384 80 48 80"
        /> < title > { title } < / title > < / svg >
    }
}
