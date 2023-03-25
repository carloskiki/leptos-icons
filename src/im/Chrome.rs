#[cfg(feature = "ImChrome")]
use leptos::*;
#[cfg(feature = "ImChrome")]
///This icon requires the feature `ImChrome` to be enabled.
#[component]
pub fn Chrome(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" xmlns
        : xlink = "http://www.w3.org/1999/xlink" fill = "#000000" d =
        "M4.036 6.977l-2.29-3.966c1.466-1.835 3.722-3.012 6.254-3.012 2.929 0 5.489 1.574 6.883 3.922h-6.528c-0.117-0.010-0.236-0.016-0.356-0.016-1.904 0-3.509 1.307-3.964 3.071zM10.864 5.078h4.585c0.355 0.905 0.551 1.891 0.551 2.922 0 4.388-3.533 7.95-7.909 7.999l3.272-5.667c0.461-0.662 0.731-1.466 0.731-2.332 0-1.143-0.471-2.178-1.23-2.922zM5.094 8c0-1.603 1.304-2.906 2.906-2.906s2.906 1.304 2.906 2.906c0 1.602-1.304 2.906-2.906 2.906s-2.906-1.304-2.906-2.906zM9.097 11.944l-2.29 3.967c-3.852-0.576-6.806-3.899-6.806-7.911 0-1.425 0.373-2.763 1.026-3.922l3.266 5.657c0.654 1.392 2.070 2.359 3.707 2.359 0.38 0 0.747-0.052 1.097-0.149z"
        /> < title > { title } < / title > < / svg >
    }
}
