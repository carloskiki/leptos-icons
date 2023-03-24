#[cfg(feature = "TbGlobeOff")]
use leptos::*;
#[cfg(feature = "TbGlobeOff")]
///This icon requires the feature `TbGlobeOff` to be enabled.
#[component]
pub fn GlobeOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-globe-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.36 8.339a4 4 0 0 0 5.281 5.31m2 -1.98a4 4 0 0 0 -5.262 -5.325" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M6.75 16a8.015 8.015 0 0 0 9.799 .553m2.016 -2a8.015 8.015 0 0 0 -2.565 -11.555"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 18v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 22h8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
