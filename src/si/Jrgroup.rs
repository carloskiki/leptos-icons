#[cfg(feature = "SiJrgroup")]
use leptos::*;
#[cfg(feature = "SiJrgroup")]
///This icon requires the feature `SiJrgroup` to be enabled.
#[component]
pub fn Jrgroup(
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
        "M19.955 13.653h1.089c2.684 0 2.684-4.123 2.684-4.123s0-4.162-2.684-4.162H9.18v8.869c0 1.556-3.112 1.478-3.112 1.478s-3.073.116-3.073-1.478v-3.423H0v4.395c0 3.19 5.68 3.384 6.107 3.423.428 0 6.107-.194 6.107-3.423V8.363h7.896c.661 0 .661 1.167.661 1.167s0 1.167-.66 1.167h-6.069l5.952 7.702H24Z"
        /> < title > { title } < / title > < / svg >
    }
}
