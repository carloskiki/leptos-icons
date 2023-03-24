#[cfg(feature = "TbLanguageOff")]
use leptos::*;
#[cfg(feature = "TbLanguageOff")]
///This icon requires the feature `TbLanguageOff` to be enabled.
#[component]
pub fn LanguageOff(
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
        "icon icon-tabler icon-tabler-language-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M4 5h1m4 0h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M9 3v2m-.508 3.517c-.814 2.655 -2.52 4.483 -4.492 4.483" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 9c0 2.144 2.952 3.908 6.7 4" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M12 20l2.463 -5.541m1.228 -2.764l.309 -.695l.8 1.8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 18h-5.1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
