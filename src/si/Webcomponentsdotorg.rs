#[cfg(feature = "SiWebcomponentsdotorg")]
use leptos::*;
#[cfg(feature = "SiWebcomponentsdotorg")]
///This icon requires the feature `SiWebcomponentsdotorg` to be enabled.
#[component]
pub fn Webcomponentsdotorg(
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
        "M11.731 2.225l-.01.016H5.618L0 11.979l5.618 9.736h12.8l.04.06 2.134-3.735.518-.893h-.008l.008-.014-.626-.75h.895l.006-.01.008.01L24 11.994l-2.607-4.39-.003.005-.011-.02h-.945l.63-.763-2.606-4.57-.006.01-.024-.04H11.73zM9.107 6.824h6.19l-.53.764h-.023l2.398 4.015h.875l-.277.328.357.435h-.956l-2.398 4.015h.027l.523.764H9.074l-2.99-5.168 3.022-5.155z"
        /> < title > { title } < / title > < / svg >
    }
}
