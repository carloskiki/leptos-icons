#[cfg(feature = "SiApachecordova")]
use leptos::*;
#[cfg(feature = "SiApachecordova")]
///This icon requires the feature `SiApachecordova` to be enabled.
#[component]
pub fn Apachecordova(
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
        "M18.545,0.545H5.455L0,9.273l2.182,14.182h3.886l-0.273-3.273h1.909l0.273,3.273 h8.045l0.273-3.273h1.909l-0.273,3.273h3.886L24,9.273L18.545,0.545z M18.545,18H5.455L4.364,9.273l2.182-4.364h3.506L9.818,6.545 h4.364l-0.234-1.636h3.506l2.182,4.364L18.545,18z M15.545,11.045c0.301,0,0.545,0.908,0.545,2.029 c0,1.121-0.244,2.029-0.545,2.029c-0.301,0-0.545-0.908-0.545-2.029C15,11.954,15.244,11.045,15.545,11.045z M8.659,11.215 c0.301,0,0.545,0.908,0.545,2.029c0,1.121-0.244,2.029-0.545,2.029c-0.301,0-0.545-0.908-0.545-2.029 C8.114,12.123,8.358,11.215,8.659,11.215z"
        /> < title > { title } < / title > < / svg >
    }
}
