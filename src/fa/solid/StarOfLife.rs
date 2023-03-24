#[cfg(feature = "FaSolidStarOfLife")]
use leptos::*;
#[cfg(feature = "FaSolidStarOfLife")]
///This icon requires the feature `FaSolidStarOfLife` to be enabled.
#[component]
pub fn StarOfLife(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M186 32c0-17.7 14.3-32 32-32h32c17.7 0 32 14.3 32 32V172.9l122-70.4c15.3-8.8 34.9-3.6 43.7 11.7l16 27.7c8.8 15.3 3.6 34.9-11.7 43.7L330 256l122 70.4c15.3 8.8 20.5 28.4 11.7 43.7l-16 27.7c-8.8 15.3-28.4 20.6-43.7 11.7L282 339.1V480c0 17.7-14.3 32-32 32H218c-17.7 0-32-14.3-32-32V339.1L64 409.6c-15.3 8.8-34.9 3.6-43.7-11.7l-16-27.7C-4.5 354.8 .7 335.3 16 326.4L138 256 16 185.6C.7 176.7-4.5 157.2 4.3 141.9l16-27.7C29.1 98.8 48.7 93.6 64 102.4l122 70.4V32z"
        /> < title > { title } < / title > < / svg >
    }
}
