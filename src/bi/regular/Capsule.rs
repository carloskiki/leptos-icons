#[cfg(feature = "BiRegularCapsule")]
use leptos::*;
#[cfg(feature = "BiRegularCapsule")]
///This icon requires the feature `BiRegularCapsule` to be enabled.
#[component]
pub fn Capsule(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M8.999 20.133a4.969 4.969 0 0 0 3.536-1.465l7.134-7.133a5.007 5.007 0 0 0-.001-7.072C18.723 3.52 17.467 3 16.132 3s-2.591.52-3.534 1.464l-7.134 7.134a5.009 5.009 0 0 0 0 7.072 4.97 4.97 0 0 0 3.535 1.463zm5.013-14.255A2.979 2.979 0 0 1 16.132 5c.802 0 1.556.313 2.122.878a3.004 3.004 0 0 1 .001 4.243l-2.893 2.892L11.12 8.77l2.892-2.892zm-7.134 7.134 2.828-2.828 4.242 4.243-2.827 2.827c-1.133 1.133-3.11 1.132-4.243.001a3.005 3.005 0 0 1 0-4.243z"
        /> < title > { title } < / title > < / svg >
    }
}
