#[cfg(feature = "VsStarHalf")]
use leptos::*;
#[cfg(feature = "VsStarHalf")]
///This icon requires the feature `VsStarHalf` to be enabled.
#[component]
pub fn StarHalf(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M6.405 6.252L8 1l1.595 5.252H15l-4.373 3.4L12.25 15 8 11.695 3.75 15l1.623-5.348L1 6.252h5.405zM8 10.032l1.915 1.49-.731-2.41 1.915-1.49H8.732L8 5.214v4.82zm0-7.525zm5.652 4.215H9.28h4.372z"
        /> < title > { title } < / title > < / svg >
    }
}
