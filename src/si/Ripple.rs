#[cfg(feature = "SiRipple")]
use leptos::*;
#[cfg(feature = "SiRipple")]
///This icon requires the feature `SiRipple` to be enabled.
#[component]
pub fn Ripple(
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
        "M20.55 14.65c-.846-.486-1.805-.632-2.752-.666-.79-.023-1.974-.541-1.974-1.985 0-1.072.868-1.94 1.985-1.985.947-.034 1.906-.18 2.752-.666A5.018 5.018 0 0022.4 2.502 5.04 5.04 0 0015.53.674a4.993 4.993 0 00-2.504 4.343c0 .97.35 1.861.79 2.696.372.699.553 1.996-.71 2.73-.948.54-2.132.202-2.719-.745-.496-.801-1.094-1.545-1.94-2.03C6.045 6.28 2.977 7.104 1.6 9.495A5.018 5.018 0 003.44 16.34a5.025 5.025 0 005.008 0c.846-.485 1.444-1.23 1.94-2.03.406-.654 1.433-1.489 2.718-.744.948.541 1.241 1.737.711 2.73-.44.823-.79 1.725-.79 2.695A5.011 5.011 0 0018.034 24a5.011 5.011 0 005.008-5.008 4.982 4.982 0 00-2.492-4.343z"
        /> < title > { title } < / title > < / svg >
    }
}
