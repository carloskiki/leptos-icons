#[cfg(feature = "BiRegularLira")]
use leptos::*;
#[cfg(feature = "BiRegularLira")]
///This icon requires the feature `BiRegularLira` to be enabled.
#[component]
pub fn Lira(
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
        "M9 21h2c4.411 0 8-4.038 8-9h-2c0 3.86-2.691 7-6 7v-7.358l6-1.385V8.204l-6 1.385V7.642l6-1.385V4.204l-6 1.385V3H9v3.05l-3 .693v2.053l3-.692v1.947l-3 .692v2.053l3-.692V21z"
        /> < title > { title } < / title > < / svg >
    }
}
