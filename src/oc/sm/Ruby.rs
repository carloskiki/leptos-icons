#[cfg(feature = "OcSmRuby")]
use leptos::*;
#[cfg(feature = "OcSmRuby")]
///This icon requires the feature `OcSmRuby` to be enabled.
#[component]
pub fn Ruby(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M3.637 2.291A.748.748 0 0 1 4.23 2h7.54c.232 0 .451.107.593.291l3.48 4.5a.75.75 0 0 1-.072.999l-7.25 7a.75.75 0 0 1-1.042 0l-7.25-7a.75.75 0 0 1-.072-.999ZM4.598 3.5 1.754 7.177 8 13.207l6.246-6.03L11.402 3.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
