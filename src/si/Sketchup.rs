#[cfg(feature = "SiSketchup")]
use leptos::*;
#[cfg(feature = "SiSketchup")]
///This icon requires the feature `SiSketchup` to be enabled.
#[component]
pub fn Sketchup(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M.968 9.027l7.717 4.428-.006 1.32-4.39-2.518-2.763 1.57 7.148 4.12.005 1.27-7.658-4.405c.02.516.488 2.106 1.383 3.337.91 1.247 1.946 1.776 1.946 1.776L11.428 24V11.849L.975 5.846zm22.064-3.8L15.22.723S13.982 0 12.008 0C9.952 0 8.76.746 8.76.746l-7.236 4.14 11.009 6.328V24l7.245-4.136s1.295-.715 2.279-2.414c.867-1.496.975-2.943.975-2.943zM11.251 7.308s1.615-.298 2.98.49l2.171 1.25s.003 1.097.003 2.736c0 1.313-1.112 2.674-1.112 2.674l.002-4.816zm6.402 10.562l-2.358 1.353v-1.269l1.835-1.05c1.748-1.26 2.037-3.117 2.037-3.761l-.007-5.705-5.006-2.881s-.76-.499-2.129-.499c-1.367 0-2.113.461-2.113.461L8.154 5.53l-1.11-.641L9.473 3.5s.95-.527 2.544-.527c1.462 0 2.6.571 2.6.571L20.27 6.81l-.007 6.226c.04.957-.406 3.296-2.61 4.835z"
        /> < title > { title } < / title > < / svg >
    }
}
