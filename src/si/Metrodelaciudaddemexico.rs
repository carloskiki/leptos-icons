#[cfg(feature = "SiMetrodelaciudaddemexico")]
use leptos::*;
#[cfg(feature = "SiMetrodelaciudaddemexico")]
///This icon requires the feature `SiMetrodelaciudaddemexico` to be enabled.
#[component]
pub fn Metrodelaciudaddemexico(
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
        "M3.965 8.704V24H.008V6.864h2.097c1.066 0 1.86.774 1.86 1.84m2.366-1.84c.268.521.521 1.315.521 1.84V24h3.685V8.704a1.784 1.784 0 0 0-1.84-1.84M17.4 24V8.704a1.795 1.795 0 0 0-1.844-1.84h-2.382c.269.521.269 1.315.269 1.84V24M.008 3.953V0h15.549c4.75 0 8.435 3.953 8.435 8.704V24h-3.685V8.704a4.735 4.735 0 0 0-4.75-4.75z"
        /> < title > { title } < / title > < / svg >
    }
}
