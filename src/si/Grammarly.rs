#[cfg(feature = "SiGrammarly")]
use leptos::*;
#[cfg(feature = "SiGrammarly")]
///This icon requires the feature `SiGrammarly` to be enabled.
#[component]
pub fn Grammarly(
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
        "M24 12c0 6.627-5.373 12-12 12S0 18.627 0 12C0 5.372 5.373 0 12 0c6.628 0 12 5.372 12 12m-9.633 1.626a.81.815 0 00-.799.965c.071.393.44.662.84.662h1.257l.729-.102c-1.166 1.71-3.19 2.498-5.405 2.15-1.802-.282-3.35-1.502-4.003-3.205-1.483-3.865 1.34-7.556 5.02-7.556 1.916 0 3.598 1.122 4.562 2.478.277.39.763.504 1.133.248a.795.8 0 00.236-1.069h.006a7.04 7.04 0 00-6.425-3.233c-3.508.236-6.347 3.107-6.55 6.617-.233 4.086 3.007 7.421 7.037 7.421a6.976 6.976 0 005.304-2.413l-.153.855v.773c0 .4.269.77.662.84a.814.814 0 00.964-.8v-4.63h-4.415"
        /> < title > { title } < / title > < / svg >
    }
}
