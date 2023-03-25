#[cfg(feature = "TbArrowLeftRhombus")]
use leptos::*;
#[cfg(feature = "TbArrowLeftRhombus")]
///This icon requires the feature `TbArrowLeftRhombus` to be enabled.
#[component]
pub fn ArrowLeftRhombus(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-arrow-left-rhombus" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M16 12h-13" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 9l-3 3l3 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18.5 9.5l2.5 2.5l-2.5 2.5l-2.5 -2.5z" /> <
        title > { title } < / title > < / svg >
    }
}
