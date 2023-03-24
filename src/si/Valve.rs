#[cfg(feature = "SiValve")]
use leptos::*;
#[cfg(feature = "SiValve")]
///This icon requires the feature `SiValve` to be enabled.
#[component]
pub fn Valve(
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
        "M0 8.579v6.842h24V8.58zm1.8 1.415h.793l.776 3.044.76-3.044h.836l-1.227 4.029H3zm5.488 0h1.084l1.145 4.034h-.814l-.27-1.007H7.228s-.21.81-.254.99c-.242.017-.83 0-.83 0zm4.184 0h.792v3.352h1.69v.677h-2.482zm3.45 0h.816l.776 3.005.754-3.005h.815l-1.222 4.034h-.716zm4.828 0h1.69v.522h-1.084v.584h.99v.523h-.99v.6h1.084v.523h-1.69zm-11.902.68l-.426 1.702h.89z"
        /> < title > { title } < / title > < / svg >
    }
}
