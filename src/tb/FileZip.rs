#[cfg(feature = "TbFileZip")]
use leptos::*;
#[cfg(feature = "TbFileZip")]
///This icon requires the feature `TbFileZip` to be enabled.
#[component]
pub fn FileZip(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-file-zip"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 20.735a2 2 0 0 1 -1 -1.735v-14a2 2 0 0 1 2 -2h7l5 5v11a2 2 0 0 1 -2 2h-1" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M11 17a2 2 0 0 1 2 2v2a1 1 0 0 1 -1 1h-2a1 1 0 0 1 -1 -1v-2a2 2 0 0 1 2 -2z" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M11 5l-1 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 7l-1 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 9l-1 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 11l-1 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 13l-1 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 15l-1 0" /> < title > { title } < / title >
        < / svg >
    }
}
