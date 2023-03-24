#[cfg(feature = "TbDiscGolf")]
use leptos::*;
#[cfg(feature = "TbDiscGolf")]
///This icon requires the feature `TbDiscGolf` to be enabled.
#[component]
pub fn DiscGolf(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-disc-golf"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d = "M5 5h14" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M6 5c.32 6.744 2.74 9.246 6 10"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 5c-.32 6.744 -2.74 9.246 -6 10" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 5c0 4.915 .552 7.082 2 10" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 5c0 4.915 -.552 7.082 -2 10" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M12 15v6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 3v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M7 16c.64 .64 1.509 1 2.414 1h5.172c.905 0 1.774 -.36 2.414 -1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 21h2" /> < title > { title } < / title > <
        / svg >
    }
}
