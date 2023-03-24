#[cfg(feature = "BiLogosDiscourse")]
use leptos::*;
#[cfg(feature = "BiLogosDiscourse")]
///This icon requires the feature `BiLogosDiscourse` to be enabled.
#[component]
pub fn Discourse(
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
        "M12.077 3C7.149 3 3 6.96 3 11.843V21l9.075-.01c4.928 0 8.925-4.11 8.925-8.993C21 7.113 17 3 12.077 3zm3.92 12.859a5.568 5.568 0 0 1-6.102 1.043l-3.595.805 1.001-3.192a5.435 5.435 0 0 1 .11-5.415 5.55 5.55 0 0 1 4.753-2.678v.001h.006a5.533 5.533 0 0 1 5.131 3.438 5.442 5.442 0 0 1-1.304 5.998z"
        /> < title > { title } < / title > < / svg >
    }
}
