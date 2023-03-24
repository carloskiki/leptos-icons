#[cfg(feature = "TbLivePhoto")]
use leptos::*;
#[cfg(feature = "TbLivePhoto")]
///This icon requires the feature `TbLivePhoto` to be enabled.
#[component]
pub fn LivePhoto(
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
        "icon icon-tabler icon-tabler-live-photo" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 12m-5 0a5 5 0 1 0 10 0a5 5 0 1 0 -10 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M15.9 20.11l0 .01" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M19.04 17.61l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20.77 14l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20.77 10l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19.04 6.39l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15.9 3.89l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 3l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8.1 3.89l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4.96 6.39l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3.23 10l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3.23 14l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4.96 17.61l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8.1 20.11l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 21l0 .01" /> < title > { title } < / title
        > < / svg >
    }
}
