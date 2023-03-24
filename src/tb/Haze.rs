#[cfg(feature = "TbHaze")]
use leptos::*;
#[cfg(feature = "TbHaze")]
///This icon requires the feature `TbHaze` to be enabled.
#[component]
pub fn Haze(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-haze"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 12h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 3v1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 12h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5.6 5.6l.7 .7" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18.4 5.6l-.7 .7" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 12a4 4 0 1 1 8 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 16h18" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 20h18" /> < title > { title } < / title > <
        / svg >
    }
}
