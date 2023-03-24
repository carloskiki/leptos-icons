#[cfg(feature = "BiRegularImages")]
use leptos::*;
#[cfg(feature = "BiRegularImages")]
///This icon requires the feature `BiRegularImages` to be enabled.
#[component]
pub fn Images(
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
        "M20 2H8c-1.103 0-2 .897-2 2v12c0 1.103.897 2 2 2h12c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zM8 16V4h12l.002 12H8z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4 8H2v12c0 1.103.897 2 2 2h12v-2H4V8z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "m12 12-1-1-2 3h10l-4-6z" /> < title > { title }
        < / title > < / svg >
    }
}
