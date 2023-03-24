#[cfg(feature = "TbIroningSteamOff")]
use leptos::*;
#[cfg(feature = "TbIroningSteamOff")]
///This icon requires the feature `TbIroningSteamOff` to be enabled.
#[component]
pub fn IroningSteamOff(
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
        "icon icon-tabler icon-tabler-ironing-steam-off" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 4h7.459a3 3 0 0 1 2.959 2.507l.577 3.464l.81 4.865a1 1 0 0 1 -.821 1.15" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M16 16h-13a7 7 0 0 1 6.056 -6.937"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M13 9h6.8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 19v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 19l-1 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 19l1 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
