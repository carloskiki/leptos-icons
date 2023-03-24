#[cfg(feature = "TbMathSymbols")]
use leptos::*;
#[cfg(feature = "TbMathSymbols")]
///This icon requires the feature `TbMathSymbols` to be enabled.
#[component]
pub fn MathSymbols(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-math-symbols" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 12l18 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 3l0 18" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16.5 4.5l3 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19.5 4.5l-3 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6 4l0 4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 6l4 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 16l.01 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 20l.01 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 18l4 0" /> < title > { title } < / title > <
        / svg >
    }
}
