#[cfg(feature = "TbCeOff")]
use leptos::*;
#[cfg(feature = "TbCeOff")]
///This icon requires the feature `TbCeOff` to be enabled.
#[component]
pub fn CeOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-ce-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11 4a7.99 7.99 0 0 0 -2.581 .426" />< path xmlns = "http://www.w3.org/2000/svg"
        d = "M5.867 5.864a8 8 0 0 0 5.133 14.136" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 4a8 8 0 0 0 -7.29 4.7" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 12a8 8 0 0 0 8 8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 12h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
