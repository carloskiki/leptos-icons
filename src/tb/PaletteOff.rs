#[cfg(feature = "TbPaletteOff")]
use leptos::*;
#[cfg(feature = "TbPaletteOff")]
///This icon requires the feature `TbPaletteOff` to be enabled.
#[component]
pub fn PaletteOff(
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
        "icon icon-tabler icon-tabler-palette-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 15h-1a2 2 0 0 0 -1 3.75a1.3 1.3 0 0 1 -1 2.25a9 9 0 0 1 -6.372 -15.356" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 4c1.236 -.623 2.569 -1 4 -1c4.97 0 9 3.582 9 8c0 1.06 -.474 2.078 -1.318 2.828a4.516 4.516 0 0 1 -1.127 .73"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.5 10.5m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12.5 7.5m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.5 10.5m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
