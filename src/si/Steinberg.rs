#[cfg(feature = "SiSteinberg")]
use leptos::*;
#[cfg(feature = "SiSteinberg")]
///This icon requires the feature `SiSteinberg` to be enabled.
#[component]
pub fn Steinberg(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M19.4807 12.6291c.6422-.371.6422-1.0092 0-1.3792L14.711 8.4954c-.6422-.371-1.1952-.052-1.1952.6901v5.508c0 .741.553 1.0601 1.1952.69zm-7.4812-9.9036c5.1219 0 9.2745 4.1526 9.2745 9.2745s-4.1526 9.2745-9.2745 9.2745S2.726 17.122 2.726 12s4.1516-9.2745 9.2735-9.2745m0-2.7255C5.3834 0 .0005 5.3829.0005 12s5.3829 12 12 12 11.999-5.3829 11.999-12-5.3829-12-12-12z"
        /> < title > { title } < / title > < / svg >
    }
}
