#[cfg(feature = "TbLinkOff")]
use leptos::*;
#[cfg(feature = "TbLinkOff")]
///This icon requires the feature `TbLinkOff` to be enabled.
#[component]
pub fn LinkOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-link-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 14a3.5 3.5 0 0 0 4.47 .444m2.025 -1.94c.557 -.556 1.392 -1.39 2.505 -2.504a3.536 3.536 0 0 0 -5 -5l-.5 .5"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.548 9.544a3.5 3.5 0 0 0 -.548 .456l-4 4a3.536 3.536 0 0 0 5 5l.5 -.5" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
