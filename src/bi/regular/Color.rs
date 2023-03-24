#[cfg(feature = "BiRegularColor")]
use leptos::*;
#[cfg(feature = "BiRegularColor")]
///This icon requires the feature `BiRegularColor` to be enabled.
#[component]
pub fn Color(
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
        "M12 2a10 10 0 1 0 10 10A10 10 0 0 0 12 2zm0 13a3 3 0 1 1 3-3 3 3 0 0 1-3 3zm2.75-7.17A5 5 0 0 0 13 7.1v-3a7.94 7.94 0 0 1 3.9 1.62zM11 7.1a5 5 0 0 0-1.75.73L7.1 5.69A7.94 7.94 0 0 1 11 4.07zM7.83 9.25A5 5 0 0 0 7.1 11h-3a7.94 7.94 0 0 1 1.59-3.9zM7.1 13a5 5 0 0 0 .73 1.75L5.69 16.9A7.94 7.94 0 0 1 4.07 13zm2.15 3.17a5 5 0 0 0 1.75.73v3a7.94 7.94 0 0 1-3.9-1.62zm3.75.73a5 5 0 0 0 1.75-.73l2.15 2.14a7.94 7.94 0 0 1-3.9 1.62zm3.17-2.15A5 5 0 0 0 16.9 13h3a7.94 7.94 0 0 1-1.62 3.9zM16.9 11a5 5 0 0 0-.73-1.75l2.14-2.15a7.94 7.94 0 0 1 1.62 3.9z"
        /> < title > { title } < / title > < / svg >
    }
}
