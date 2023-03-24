#[cfg(feature = "TbMan")]
use leptos::*;
#[cfg(feature = "TbMan")]
///This icon requires the feature `TbMan` to be enabled.
#[component]
pub fn Man(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-man" width
        = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M10 16v5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 16v5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 9h6l-1 7h-4z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 11c1.333 -1.333 2.667 -2 4 -2" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M19 11c-1.333 -1.333 -2.667 -2 -4 -2"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 4m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /> < title > { title } < / title > < /
        svg >
    }
}
