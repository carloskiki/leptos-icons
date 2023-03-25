#[cfg(feature = "TbArrowsRandom")]
use leptos::*;
#[cfg(feature = "TbArrowsRandom")]
///This icon requires the feature `TbArrowsRandom` to be enabled.
#[component]
pub fn ArrowsRandom(
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
        "icon icon-tabler icon-tabler-arrows-random" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M20 21h-4v-4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 21l5 -5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6.5 9.504l-3.5 -2l2 -3.504" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 7.504l6.83 -1.87" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 16l4 -1l1 4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 15l-3.5 6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 5l-.5 4l-4 -.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20.5 9l-4.5 -5.5" /> < title > { title } < /
        title > < / svg >
    }
}
