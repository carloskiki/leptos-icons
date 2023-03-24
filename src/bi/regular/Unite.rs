#[cfg(feature = "BiRegularUnite")]
use leptos::*;
#[cfg(feature = "BiRegularUnite")]
///This icon requires the feature `BiRegularUnite` to be enabled.
#[component]
pub fn Unite(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M15 16a1 1 0 0 0 1-1V9a1 1 0 0 0-1-1H9a1 1 0 0 0-1 1v6a1 1 0 0 0 1 1h6z" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 16h1V8a2 2 0 0 1 2-2h8V5a2 2 0 0 0-2-2H5a2 2 0 0 0-2 2v9a2 2 0 0 0 2 2zm3 3a2 2 0 0 0 2 2h9a2 2 0 0 0 2-2v-9a2 2 0 0 0-2-2h-1v8a2 2 0 0 1-2 2H8v1z"
        /> < title > { title } < / title > < / svg >
    }
}
