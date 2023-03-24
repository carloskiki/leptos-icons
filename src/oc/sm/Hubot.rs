#[cfg(feature = "OcSmHubot")]
use leptos::*;
#[cfg(feature = "OcSmHubot")]
///This icon requires the feature `OcSmHubot` to be enabled.
#[component]
pub fn Hubot(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M0 8a8 8 0 0 1 16 0v5.25a.75.75 0 0 1-1.5 0V8a6.5 6.5 0 1 0-13 0v5.25a.75.75 0 0 1-1.5 0Zm3-1.25C3 5.784 3.784 5 4.75 5h6.5c.966 0 1.75.784 1.75 1.75v1.5A1.75 1.75 0 0 1 11.25 10h-6.5A1.75 1.75 0 0 1 3 8.25Zm1.47-.53a.75.75 0 0 0 0 1.06l1.5 1.5a.75.75 0 0 0 1.06 0L8 7.81l.97.97a.75.75 0 0 0 1.06 0l1.5-1.5a.749.749 0 0 0-.326-1.275.749.749 0 0 0-.734.215l-.97.97-.97-.97a.75.75 0 0 0-1.06 0l-.97.97-.97-.97a.75.75 0 0 0-1.06 0Zm1.03 6.03a.75.75 0 0 1 .75-.75h3.5a.75.75 0 0 1 0 1.5h-3.5a.75.75 0 0 1-.75-.75Z"
        /> < title > { title } < / title > < / svg >
    }
}
