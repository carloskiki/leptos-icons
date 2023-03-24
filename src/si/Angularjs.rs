#[cfg(feature = "SiAngularjs")]
use leptos::*;
#[cfg(feature = "SiAngularjs")]
///This icon requires the feature `SiAngularjs` to be enabled.
#[component]
pub fn Angularjs(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M11.964 0L.672 3.974l1.784 14.794L11.976 24l9.568-5.303 1.784-14.794zm-.027 1.258l10.265 3.5-1.663 13.232-8.602 4.76-8.469-4.697L1.939 4.822zm0 .78L4.957 17.57l2.604-.048 1.4-3.501h6.257l1.532 3.55 2.492.046zm.02 4.98l2.355 4.93H9.878Z"
        /> < title > { title } < / title > < / svg >
    }
}
