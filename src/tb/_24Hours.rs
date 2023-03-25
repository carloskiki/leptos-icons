#[cfg(feature = "Tb24Hours")]
use leptos::*;
#[cfg(feature = "Tb24Hours")]
///This icon requires the feature `Tb24Hours` to be enabled.
#[component]
pub fn _24Hours(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-24-hours"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 11a8.1 8.1 0 0 0 -15.5 -2m-.5 -4v4h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 13a8.094 8.094 0 0 0 3 5.24" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M11 15h2a1 1 0 0 1 1 1v1a1 1 0 0 1 -1 1h-1a1 1 0 0 0 -1 1v1a1 1 0 0 0 1 1h2" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M17 15v2a1 1 0 0 0 1 1h1" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M20 15v6" /> < title > { title } < /
        title > < / svg >
    }
}
