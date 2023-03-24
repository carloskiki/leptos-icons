#[cfg(feature = "TbTiltShift")]
use leptos::*;
#[cfg(feature = "TbTiltShift")]
///This icon requires the feature `TbTiltShift` to be enabled.
#[component]
pub fn TiltShift(
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
        "icon icon-tabler icon-tabler-tilt-shift" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.56 3.69a9 9 0 0 0 -2.92 1.95" />< path xmlns = "http://www.w3.org/2000/svg" d
        = "M3.69 8.56a9 9 0 0 0 -.69 3.44" />< path xmlns = "http://www.w3.org/2000/svg"
        d = "M3.69 15.44a9 9 0 0 0 1.95 2.92" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8.56 20.31a9 9 0 0 0 3.44 .69" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M15.44 20.31a9 9 0 0 0 2.92 -1.95" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M20.31 15.44a9 9 0 0 0 .69 -3.44" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M20.31 8.56a9 9 0 0 0 -1.95 -2.92"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.44 3.69a9 9 0 0 0 -3.44 -.69" />< path xmlns = "http://www.w3.org/2000/svg"
        d = "M12 12m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> < title > { title } < / title >
        < / svg >
    }
}
