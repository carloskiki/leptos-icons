#[cfg(feature = "IoAnalyticsSharp")]
use leptos::*;
#[cfg(feature = "IoAnalyticsSharp")]
///This icon requires the feature `IoAnalyticsSharp` to be enabled.
#[component]
pub fn AnalyticsSharp(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M450,128a46,46,0,0,0-44.11,59l-71.37,71.36a45.88,45.88,0,0,0-29,0l-52.91-52.91a46,46,0,1,0-89.12,0L75,293.88A46.08,46.08,0,1,0,106.11,325l87.37-87.36a45.85,45.85,0,0,0,29,0l52.92,52.92a46,46,0,1,0,89.12,0L437,218.12A46,46,0,1,0,450,128Z"
        /> < title > { title } < / title > < / svg >
    }
}
