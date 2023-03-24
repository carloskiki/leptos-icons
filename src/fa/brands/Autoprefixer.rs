#[cfg(feature = "FaBrandsAutoprefixer")]
use leptos::*;
#[cfg(feature = "FaBrandsAutoprefixer")]
///This icon requires the feature `FaBrandsAutoprefixer` to be enabled.
#[component]
pub fn Autoprefixer(
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
        stroke_witdh = "0" style = style viewBox = "0 0 640 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M318.4 16l-161 480h77.5l25.4-81.4h119.5L405 496h77.5L318.4 16zm-40.3 341.9l41.2-130.4h1.5l40.9 130.4h-83.6zM640 405l-10-31.4L462.1 358l19.4 56.5L640 405zm-462.1-47L10 373.7 0 405l158.5 9.4 19.4-56.4z"
        /> < title > { title } < / title > < / svg >
    }
}
