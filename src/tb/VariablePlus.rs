#[cfg(feature = "TbVariablePlus")]
use leptos::*;
#[cfg(feature = "TbVariablePlus")]
///This icon requires the feature `TbVariablePlus` to be enabled.
#[component]
pub fn VariablePlus(
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
        "icon icon-tabler icon-tabler-variable-plus" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 4c-2.5 5 -2.5 10 0 16m14 -16c1.38 2.76 2 5.52 1.855 8.448m-11.855 -3.448h1c1 0 1 1 2.016 3.527c.785 1.972 .944 3.008 1.483 3.346"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 16c1.5 0 3 -2 4 -3.5s2.5 -3.5 4 -3.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 19h6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 16v6" /> < title > { title } < / title > <
        / svg >
    }
}
