#[cfg(feature = "SiClyp")]
use leptos::*;
#[cfg(feature = "SiClyp")]
///This icon requires the feature `SiClyp` to be enabled.
#[component]
pub fn Clyp(
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
        "M11.9995 17.9583a1.137 1.137 0 01-1.137-1.136V7.2347a1.138 1.138 0 012.275 0v9.5896c0 .626-.51 1.134-1.138 1.134m7.4397 2.4398a1.137 1.137 0 01-1.14-1.1379V4.7958a1.138 1.138 0 012.276 0v14.4654c0 .627-.51 1.136-1.138 1.136M15.7193 24a1.137 1.137 0 01-1.138-1.136V1.138a1.138 1.138 0 012.276 0v21.726c0 .627-.509 1.136-1.138 1.136m-7.4366-3.1599a1.137 1.137 0 01-1.138-1.136V4.2979a1.138 1.138 0 012.276 0v15.4064c0 .628-.51 1.137-1.138 1.137m-3.7199-4.9889a1.137 1.137 0 01-1.138-1.135V9.2857a1.138 1.138 0 012.276 0v5.4318c0 .626-.51 1.135-1.138 1.135z"
        /> < title > { title } < / title > < / svg >
    }
}
