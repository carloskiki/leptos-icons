#[cfg(feature = "SiWeightsandbiases")]
use leptos::*;
#[cfg(feature = "SiWeightsandbiases")]
///This icon requires the feature `SiWeightsandbiases` to be enabled.
#[component]
pub fn Weightsandbiases(
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
        "M2.48 0a1.55 1.55 0 1 0 0 3.1 1.55 1.55 0 0 0 0-3.1zm19.04 0a1.55 1.55 0 1 0 0 3.101 1.55 1.55 0 0 0 0-3.101zM12 2.295a1.55 1.55 0 1 0 0 3.1 1.55 1.55 0 0 0 0-3.1zM2.48 5.272a2.48 2.48 0 1 0 0 4.96 2.48 2.48 0 0 0 0-4.96zm19.04 0a2.48 2.48 0 1 0 0 4.96 2.48 2.48 0 0 0 0-4.96zM12 8.496a1.55 1.55 0 1 0 0 3.1 1.55 1.55 0 0 0 0-3.1zm-9.52 3.907a1.55 1.55 0 1 0 0 3.1 1.55 1.55 0 0 0 0-3.1zm19.04 0a1.55 1.55 0 1 0 0 3.102 1.55 1.55 0 0 0 0-3.102zM12 13.767a2.48 2.48 0 1 0 0 4.962 2.48 2.48 0 0 0 0-4.962zm-9.52 3.907a2.48 2.48 0 1 0 .001 4.962 2.48 2.48 0 0 0 0-4.962zm19.04.93a1.55 1.55 0 1 0 0 3.102 1.55 1.55 0 0 0 0-3.101zM12 20.9a1.55 1.55 0 1 0 0 3.1 1.55 1.55 0 0 0 0-3.1Z"
        /> < title > { title } < / title > < / svg >
    }
}
