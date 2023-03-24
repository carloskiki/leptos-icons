#[cfg(feature = "ImDeviantart")]
use leptos::*;
#[cfg(feature = "ImDeviantart")]
///This icon requires the feature `ImDeviantart` to be enabled.
#[component]
pub fn Deviantart(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M12.953 2.909v-2.909h-2.909l-0.291 0.294-1.375 2.616-0.431 0.291h-4.9v3.994h2.694l0.241 0.291-2.934 5.606v2.909h2.909l0.291-0.294 1.375-2.616 0.431-0.291h4.9v-3.994h-2.694l-0.241-0.294z"
        /> < title > { title } < / title > < / svg >
    }
}
