#[cfg(feature = "TbContainerOff")]
use leptos::*;
#[cfg(feature = "TbContainerOff")]
///This icon requires the feature `TbContainerOff` to be enabled.
#[component]
pub fn ContainerOff(
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
        "icon icon-tabler icon-tabler-container-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M20 4v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 20v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 16v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 12v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 8v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M8.297 4.289a1 1 0 0 1 .703 -.289h6a1 1 0 0 1 1 1v7m0 4v3a1 1 0 0 1 -1 1h-6a1 1 0 0 1 -1 -1v-11"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M4 4v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 20v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 16v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 12v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 8v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
