#[cfg(feature = "TbSwitch")]
use leptos::*;
#[cfg(feature = "TbSwitch")]
///This icon requires the feature `TbSwitch` to be enabled.
#[component]
pub fn Switch(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-switch"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M15 4l4 0l0 4" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M14.75 9.25l4.25 -5.25" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 19l4 -4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 19l4 0l0 -4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 5l14 14" /> < title > { title } < / title >
        < / svg >
    }
}
