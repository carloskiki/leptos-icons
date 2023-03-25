#[cfg(feature = "CgChevronDoubleDown")]
use leptos::*;
#[cfg(feature = "CgChevronDoubleDown")]
///This icon requires the feature `CgChevronDoubleDown` to be enabled.
#[component]
pub fn ChevronDoubleDown(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.75735 5.63605L6.34314 7.05026L12 12.7071L17.6569 7.05029L16.2427 5.63608L12 9.87872L7.75735 5.63605Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.34314 12.7071L7.75735 11.2929L12 15.5356L16.2427 11.2929L17.6569 12.7071L12 18.364L6.34314 12.7071Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
