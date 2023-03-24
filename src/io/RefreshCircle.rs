#[cfg(feature = "IoRefreshCircle")]
use leptos::*;
#[cfg(feature = "IoRefreshCircle")]
///This icon requires the feature `IoRefreshCircle` to be enabled.
#[component]
pub fn RefreshCircle(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M256,48C141.31,48,48,141.32,48,256c0,114.86,93.14,208,208,208,114.69,0,208-93.31,208-208C464,141.13,370.87,48,256,48Zm0,313a94,94,0,0,1,0-188h4.21L246.1,158.9a14,14,0,0,1,19.8-19.8l40,40a14,14,0,0,1,0,19.8l-40,40a14,14,0,0,1-19.8-19.8l18-18C261.72,201,259,201,256,201a66,66,0,1,0,66,66,14,14,0,0,1,28,0A94.11,94.11,0,0,1,256,361Z"
        /> < title > { title } < / title > < / svg >
    }
}
