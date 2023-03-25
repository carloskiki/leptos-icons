#[cfg(feature = "VsQuestion")]
use leptos::*;
#[cfg(feature = "VsQuestion")]
///This icon requires the feature `VsQuestion` to be enabled.
#[component]
pub fn Question(
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
        "M7.5 1a6.5 6.5 0 1 0 0 13 6.5 6.5 0 0 0 0-13zm0 12a5.5 5.5 0 1 1 0-11 5.5 5.5 0 0 1 0 11zm1.55-8.42a1.84 1.84 0 0 0-.61-.42A2.25 2.25 0 0 0 7.53 4a2.16 2.16 0 0 0-.88.17c-.239.1-.45.254-.62.45a1.89 1.89 0 0 0-.38.62 3 3 0 0 0-.15.72h1.23a.84.84 0 0 1 .506-.741.72.72 0 0 1 .304-.049.86.86 0 0 1 .27 0 .64.64 0 0 1 .22.14.6.6 0 0 1 .16.22.73.73 0 0 1 .06.3c0 .173-.037.343-.11.5a2.4 2.4 0 0 1-.27.46l-.35.42c-.12.13-.24.27-.35.41a2.33 2.33 0 0 0-.27.45 1.18 1.18 0 0 0-.1.5v.66H8v-.49a.94.94 0 0 1 .11-.42 3.09 3.09 0 0 1 .28-.41l.36-.44a4.29 4.29 0 0 0 .36-.48 2.59 2.59 0 0 0 .28-.55 1.91 1.91 0 0 0 .11-.64 2.18 2.18 0 0 0-.1-.67 1.52 1.52 0 0 0-.35-.55zM6.8 9.83h1.17V11H6.8V9.83z"
        /> < title > { title } < / title > < / svg >
    }
}
