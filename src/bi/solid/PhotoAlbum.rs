#[cfg(feature = "BiSolidPhotoAlbum")]
use leptos::*;
#[cfg(feature = "BiSolidPhotoAlbum")]
///This icon requires the feature `BiSolidPhotoAlbum` to be enabled.
#[component]
pub fn PhotoAlbum(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M20 2H6c-1.206 0-3 .799-3 3v14c0 2.201 1.794 3 3 3h15v-2H6.012C5.55 19.988 5 19.806 5 19s.55-.988 1.012-1H21V3a1 1 0 0 0-1-1zM9.503 5a1.503 1.503 0 1 1 0 3.006 1.503 1.503 0 0 1 0-3.006zM12 13H7l3-3 1.5 1.399L14.5 8l3.5 5h-6z"
        /> < title > { title } < / title > < / svg >
    }
}
