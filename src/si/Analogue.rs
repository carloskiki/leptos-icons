#[cfg(feature = "SiAnalogue")]
use leptos::*;
#[cfg(feature = "SiAnalogue")]
///This icon requires the feature `SiAnalogue` to be enabled.
#[component]
pub fn Analogue(
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
        "M5.468 12.804a5.145 5.145 0 10-.644 10.27 5.145 5.145 0 00.644-10.27zm17.841 2.562L16.45 3.484a5.146 5.146 0 00-8.912 5.15l6.86 11.878a5.148 5.148 0 007.031 1.885 5.146 5.146 0 001.881-7.031z"
        /> < title > { title } < / title > < / svg >
    }
}
