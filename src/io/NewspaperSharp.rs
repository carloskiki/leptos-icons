#[cfg(feature = "IoNewspaperSharp")]
use leptos::*;
#[cfg(feature = "IoNewspaperSharp")]
///This icon requires the feature `IoNewspaperSharp` to be enabled.
#[component]
pub fn NewspaperSharp(
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
        "http://www.w3.org/2000/svg" > < rect xmlns = "http://www.w3.org/2000/svg" x =
        "96" y = "112" width = "96" height = "96" rx = "16" ry = "16" style = "fill:none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M468,112H416V416a32,32,0,0,0,32,32h0a32,32,0,0,0,32-32V124A12,12,0,0,0,468,112Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M431.15,477.75A64.11,64.11,0,0,1,384,416V44a12,12,0,0,0-12-12H44A12,12,0,0,0,32,44V424a56,56,0,0,0,56,56H430.85a1.14,1.14,0,0,0,.3-2.25ZM96,208V112h96v96ZM320,400H96V368H320Zm0-64H96V304H320Zm0-64H96V240H320Zm0-64H224V176h96Zm0-64H224V112h96Z"
        /> < title > { title } < / title > < / svg >
    }
}
