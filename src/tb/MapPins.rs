#[cfg(feature = "TbMapPins")]
use leptos::*;
#[cfg(feature = "TbMapPins")]
///This icon requires the feature `TbMapPins` to be enabled.
#[component]
pub fn MapPins(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-map-pins"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.828 9.828a4 4 0 1 0 -5.656 0l2.828 2.829l2.828 -2.829z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 7l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M18.828 17.828a4 4 0 1 0 -5.656 0l2.828 2.829l2.828 -2.829z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 15l0 .01" /> < title > { title } < / title
        > < / svg >
    }
}
