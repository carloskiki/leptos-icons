#[cfg(feature = "VsStarEmpty")]
use leptos::*;
#[cfg(feature = "VsStarEmpty")]
///This icon requires the feature `VsStarEmpty` to be enabled.
#[component]
pub fn StarEmpty(
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
        "M9.595 6.252L8 1 6.405 6.252H1l4.373 3.4L3.75 15 8 11.695 12.25 15l-1.623-5.348L15 6.252H9.595zm-7.247.47H6.72L8 2.507 6.72 6.722H2.348zm3.537 2.75l-1.307 4.305 1.307-4.305zm7.767-2.75H9.28h4.372zm-8.75.9h2.366L8 5.214l.732 2.41h2.367l-1.915 1.49.731 2.409L8 10.032l-1.915 1.49.731-2.41-1.915-1.49z"
        /> < title > { title } < / title > < / svg >
    }
}
