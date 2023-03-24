#[cfg(feature = "RiDesignLineEditCircle")]
use leptos::*;
#[cfg(feature = "RiDesignLineEditCircle")]
///This icon requires the feature `RiDesignLineEditCircle` to be enabled.
#[component]
pub fn EditCircle(
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
        "M12.684 4.029a8 8 0 1 0 7.287 7.287 7.936 7.936 0 0 0-.603-2.44l1.5-1.502A9.933 9.933 0 0 1 22 12c0 5.523-4.477 10-10 10S2 17.523 2 12 6.477 2 12 2a9.982 9.982 0 0 1 4.626 1.132l-1.501 1.5a7.941 7.941 0 0 0-2.44-.603zM20.485 2.1L21.9 3.515l-9.192 9.192-1.412.003-.002-1.417L20.485 2.1z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
