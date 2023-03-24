#[cfg(feature = "SiFraunhofergesellschaft")]
use leptos::*;
#[cfg(feature = "SiFraunhofergesellschaft")]
///This icon requires the feature `SiFraunhofergesellschaft` to be enabled.
#[component]
pub fn Fraunhofergesellschaft(
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
        "M.06 23.99c5.681-2.926 14-7.653 23.88-14.567v-2.32C15.247 12.792 6.406 17.359.06 20.38zm0-6.93c6.325-2.575 15.166-6.558 23.88-11.74V4.174C15.751 8.238 7.24 10.781.06 12.366zM23.94 24V12.332A201.394 201.393 0 0 1 8.596 24zM5.542 24a166.927 166.926 0 0 0 14.7-9.765 323.136 324.76 0 0 0 3.698-2.81V9.98C16.257 15.74 8.413 20.542 2.287 24zM.06 10.668C7.044 9.44 15.589 7.231 23.94 3.262v-1.3C15.526 5.737 7.102 7.338.06 7.91zM.06 0v6.686c.522-.033 1.054-.07 1.596-.111C7.464 6.126 15.387 5.1 23.94 1.402V0z"
        /> < title > { title } < / title > < / svg >
    }
}
