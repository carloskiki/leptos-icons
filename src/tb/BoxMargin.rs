#[cfg(feature = "TbBoxMargin")]
use leptos::*;
#[cfg(feature = "TbBoxMargin")]
///This icon requires the feature `TbBoxMargin` to be enabled.
#[component]
pub fn BoxMargin(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-box-margin" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 8h8v8h-8z" />< path xmlns = "http://www.w3.org/2000/svg" d = "M4 4v.01" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M8 4v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 4v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 4v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 4v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 20v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 20v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 20v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 20v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 20v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 16v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 12v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 8v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 16v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 12v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 8v.01" /> < title > { title } < / title > <
        / svg >
    }
}
