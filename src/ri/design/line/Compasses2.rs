#[cfg(feature = "RiDesignLineCompasses2")]
use leptos::*;
#[cfg(feature = "RiDesignLineCompasses2")]
///This icon requires the feature `RiDesignLineCompasses2` to be enabled.
#[component]
pub fn Compasses2(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path d =
        "M16.33 13.5A6.988 6.988 0 0 0 19 8h2a8.987 8.987 0 0 1-3.662 7.246l2.528 4.378a2 2 0 0 1-.732 2.732l-3.527-6.108A8.97 8.97 0 0 1 12 17a8.97 8.97 0 0 1-3.607-.752l-3.527 6.108a2 2 0 0 1-.732-2.732l5.063-8.77A4.002 4.002 0 0 1 11 4.126V2h2v2.126a4.002 4.002 0 0 1 1.803 6.728L16.33 13.5zM14.6 14.502l-1.528-2.647a4.004 4.004 0 0 1-2.142 0l-1.528 2.647c.804.321 1.68.498 2.599.498.918 0 1.795-.177 2.599-.498zM12 10a2 2 0 1 0 0-4 2 2 0 0 0 0 4z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
