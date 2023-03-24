#[cfg(feature = "FaSolidBurger")]
use leptos::*;
#[cfg(feature = "FaSolidBurger")]
///This icon requires the feature `FaSolidBurger` to be enabled.
#[component]
pub fn Burger(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M45.1 224C29 224 16 211 16 194.9c0-1.9 .2-3.7 .6-5.6C21.9 168.3 62.8 32 240 32s218.1 136.3 223.4 157.3c.5 1.9 .6 3.7 .6 5.6c0 16.1-13 29.1-29.1 29.1H45.1zM128 128a16 16 0 1 0 -32 0 16 16 0 1 0 32 0zm240 16a16 16 0 1 0 0-32 16 16 0 1 0 0 32zM256 96a16 16 0 1 0 -32 0 16 16 0 1 0 32 0zM0 304c0-26.5 21.5-48 48-48H432c26.5 0 48 21.5 48 48s-21.5 48-48 48H48c-26.5 0-48-21.5-48-48zm16 96c0-8.8 7.2-16 16-16H448c8.8 0 16 7.2 16 16v16c0 35.3-28.7 64-64 64H80c-35.3 0-64-28.7-64-64V400z"
        /> < title > { title } < / title > < / svg >
    }
}
