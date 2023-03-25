#[cfg(feature = "TbBoxAlignTopLeft")]
use leptos::*;
#[cfg(feature = "TbBoxAlignTopLeft")]
///This icon requires the feature `TbBoxAlignTopLeft` to be enabled.
#[component]
pub fn BoxAlignTopLeft(
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
        "icon icon-tabler icon-tabler-box-align-top-left" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11 5v5a1 1 0 0 1 -1 1h-5a1 1 0 0 1 -1 -1v-5a1 1 0 0 1 1 -1h5a1 1 0 0 1 1 1z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M15 4h-.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 4h-.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 9h-.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 15h-.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 15h-.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 20h-.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 20h-.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 20h-.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 20h-.01" /> < title > { title } < / title >
        < / svg >
    }
}
