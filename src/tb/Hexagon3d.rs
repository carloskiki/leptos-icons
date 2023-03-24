#[cfg(feature = "TbHexagon3d")]
use leptos::*;
#[cfg(feature = "TbHexagon3d")]
///This icon requires the feature `TbHexagon3d` to be enabled.
#[component]
pub fn Hexagon3d(
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
        "icon icon-tabler icon-tabler-hexagon-3d" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 6.844a2.007 2.007 0 0 1 1 1.752v6.555c0 .728 -.394 1.399 -1.03 1.753l-6 3.844a2 2 0 0 1 -1.942 0l-6 -3.844a2.007 2.007 0 0 1 -1.029 -1.752v-6.556c0 -.729 .394 -1.4 1.029 -1.753l6 -3.583a2.05 2.05 0 0 1 2 0l6 3.584h-.03z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 16.5v4.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4.5 7.5l3.5 2.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 10l4 -2.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 7.5v4.5l-4 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 12l4 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 16.5l4 -2.5v-4l-4 -2.5l-4 2.5v4z" /> <
        title > { title } < / title > < / svg >
    }
}
