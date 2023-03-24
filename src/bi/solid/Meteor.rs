#[cfg(feature = "BiSolidMeteor")]
use leptos::*;
#[cfg(feature = "BiSolidMeteor")]
///This icon requires the feature `BiSolidMeteor` to be enabled.
#[component]
pub fn Meteor(
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
        "M13.507 2.138a1 1 0 0 0-1.155.102L4.196 9.197c-2.924 2.924-2.924 7.682 0 10.606a7.472 7.472 0 0 0 5.3 2.192c1.924 0 3.85-.734 5.317-2.202l6.903-7.096A1 1 0 0 0 21 11h-3.301l4.175-7.514a1.001 1.001 0 0 0-1.359-1.36l-7.11 3.95.576-2.879a1.002 1.002 0 0 0-.474-1.059zM14 14.5a4.5 4.5 0 0 1-9 0c0-1.57.807-2.949 2.025-3.754-.01.084-.025.167-.025.254a2 2 0 1 0 3.845-.772C12.669 10.802 14 12.486 14 14.5z"
        /> < title > { title } < / title > < / svg >
    }
}
