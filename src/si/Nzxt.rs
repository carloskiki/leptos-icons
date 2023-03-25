#[cfg(feature = "SiNzxt")]
use leptos::*;
#[cfg(feature = "SiNzxt")]
///This icon requires the feature `SiNzxt` to be enabled.
#[component]
pub fn Nzxt(
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
        "M1.763 8.936l2.101 3.04-.002-3.04h1.773v6.128H3.99l-2.218-3.227v3.227H0V8.936zm22.237 0v1.614h-1.612v4.514h-1.883V10.55h-1.611V8.936H24zm-9.598 0l.996 1.573 1.061-1.573h1.947l-1.98 3.034 2.013 3.094h-2.063l-1.005-1.558-.99 1.558h-2.015l1.975-3.038-2.004-3.09h2.065zm-2.652 0L9.327 13.51h2.372v1.554H6.573l2.379-4.584H6.704V8.936h5.046z"
        /> < title > { title } < / title > < / svg >
    }
}
