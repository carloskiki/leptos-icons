#[cfg(feature = "TbSatellite")]
use leptos::*;
#[cfg(feature = "TbSatellite")]
///This icon requires the feature `TbSatellite` to be enabled.
#[component]
pub fn Satellite(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-satellite"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3.707 6.293l2.586 -2.586a1 1 0 0 1 1.414 0l5.586 5.586a1 1 0 0 1 0 1.414l-2.586 2.586a1 1 0 0 1 -1.414 0l-5.586 -5.586a1 1 0 0 1 0 -1.414z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M6 10l-3 3l3 3l3 -3" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M10 6l3 -3l3 3l-3 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 12l1.5 1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14.5 17a2.5 2.5 0 0 0 2.5 -2.5" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M15 21a6 6 0 0 0 6 -6" /> < title > { title }
        < / title > < / svg >
    }
}
