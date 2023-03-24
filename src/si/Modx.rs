#[cfg(feature = "SiModx")]
use leptos::*;
#[cfg(feature = "SiModx")]
///This icon requires the feature `SiModx` to be enabled.
#[component]
pub fn Modx(
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
        "M19.145 9.49l4.47-7.19H12.118l-1.24 2.023zM2.92 0v11.497l2.48 1.55 13.435-3.1zm18.16 24V12.503l-1.984-1.263-5.168 8.267zM5.165 14.053l-4.78 7.648h11.497L18.6 10.953Z"
        /> < title > { title } < / title > < / svg >
    }
}
