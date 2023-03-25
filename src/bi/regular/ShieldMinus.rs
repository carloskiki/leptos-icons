#[cfg(feature = "BiRegularShieldMinus")]
use leptos::*;
#[cfg(feature = "BiRegularShieldMinus")]
///This icon requires the feature `BiRegularShieldMinus` to be enabled.
#[component]
pub fn ShieldMinus(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "m20.41 6.11-8-4a.94.94 0 0 0-.89 0l-8 4A1 1 0 0 0 3 6.9c0 .11-1 10.77 8.59 15a1 1 0 0 0 .41.1.93.93 0 0 0 .4-.09C21.92 17.67 21 7 21 6.9a1 1 0 0 0-.59-.79zM12 19.9C5.2 16.63 4.88 9.64 4.93 7.63l7-3.51 7 3.51c.13 2.01-.19 9-6.93 12.27z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M8 11h8v2H8z" /> < title > {
        title } < / title > < / svg >
    }
}
