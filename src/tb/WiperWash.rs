#[cfg(feature = "TbWiperWash")]
use leptos::*;
#[cfg(feature = "TbWiperWash")]
///This icon requires the feature `TbWiperWash` to be enabled.
#[component]
pub fn WiperWash(
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
        "icon icon-tabler icon-tabler-wiper-wash" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 20m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M3 11l5.5 5.5a5 5 0 0 1 7 0l5.5 -5.5a12 12 0 0 0 -18 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 20l0 -14" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 6a4 4 0 0 1 .4 -1.8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 2.1a4 4 0 0 1 2 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 6a4 4 0 0 0 -.4 -1.8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 6a4 4 0 0 1 .4 -1.8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 2.1a4 4 0 0 1 2 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 6a4 4 0 0 0 -.4 -1.8" /> < title > { title
        } < / title > < / svg >
    }
}
