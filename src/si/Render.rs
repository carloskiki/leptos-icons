#[cfg(feature = "SiRender")]
use leptos::*;
#[cfg(feature = "SiRender")]
///This icon requires the feature `SiRender` to be enabled.
#[component]
pub fn Render(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M13.586 0v3.172H7.523a4.528 4.528 0 0 0-3.018 1.305 4.498 4.498 0 0 0-1.333 2.99v11.326H0V7.509a7.472 7.472 0 0 1 2.204-5.305A7.471 7.471 0 0 1 4.588.589 7.432 7.432 0 0 1 7.51 0Zm5.207 0v3.158H15.62V0ZM24 0v3.158h-3.172V0Zm0 5.207v3.172h-3.172V5.207Zm0 5.207v3.172h-3.172v-3.172Zm0 5.207v3.172h-3.172V15.62Zm0 5.207V24h-3.172v-3.172Zm-5.207 0V24H15.62v-3.172Zm-5.207 0V24h-3.172v-3.172Zm-5.207 0V24H5.207v-3.172Zm-5.221 0V24H0v-3.172Z"
        /> < title > { title } < / title > < / svg >
    }
}
