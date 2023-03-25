#[cfg(feature = "SiElasticstack")]
use leptos::*;
#[cfg(feature = "SiElasticstack")]
///This icon requires the feature `SiElasticstack` to be enabled.
#[component]
pub fn Elasticstack(
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
        "M1.875 0C.839 0 0 .84 0 1.875v4.792h24V1.875C24 .839 23.16 0 22.125 0zM0 8.889v6.222h24V8.89zm0 8.444v4.792C0 23.161.84 24 1.875 24h9v-6.667zm13.125 0V24h9C23.161 24 24 23.16 24 22.125v-4.792z"
        /> < title > { title } < / title > < / svg >
    }
}
