#[cfg(feature = "VsIndent")]
use leptos::*;
#[cfg(feature = "VsIndent")]
///This icon requires the feature `VsIndent` to be enabled.
#[component]
pub fn Indent(
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
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M4 5V6.984C4 7.11661 4.0527 7.24379 4.1465 7.33755C4.2402 7.43132 4.3674 7.484 4.5 7.484H11.382L9.749 5.851L10.456 5.144L12.577 7.265L13 7.688V8.256L10.456 10.8L9.749 10.093L11.359 8.484H4.5C4.1022 8.484 3.7207 8.32597 3.4393 8.04466C3.158 7.76336 3 7.38182 3 6.984V5H4Z"
        /> < title > { title } < / title > < / svg >
    }
}
