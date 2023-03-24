#[cfg(feature = "BiRegularToggleRight")]
use leptos::*;
#[cfg(feature = "BiRegularToggleRight")]
///This icon requires the feature `BiRegularToggleRight` to be enabled.
#[component]
pub fn ToggleRight(
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
        "M16 9c-1.628 0-3 1.372-3 3s1.372 3 3 3 3-1.372 3-3-1.372-3-3-3z" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M16 6H8c-3.296 0-5.982 2.682-6 5.986v.042A6.01 6.01 0 0 0 8 18h8c3.309 0 6-2.691 6-6s-2.691-6-6-6zm0 10H8a4.006 4.006 0 0 1-4-3.99C4.004 9.799 5.798 8 8 8h8c2.206 0 4 1.794 4 4s-1.794 4-4 4z"
        /> < title > { title } < / title > < / svg >
    }
}
