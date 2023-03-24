#[cfg(feature = "TbGrill")]
use leptos::*;
#[cfg(feature = "TbGrill")]
///This icon requires the feature `TbGrill` to be enabled.
#[component]
pub fn Grill(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-grill"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 8h-14a6 6 0 0 0 6 6h2a6 6 0 0 0 6 -5.775l0 -.225z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 20a2 2 0 1 1 0 -4a2 2 0 0 1 0 4z" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M15 14l1 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 14l-3 6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 18h-8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 5v-1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 5v-1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 5v-1" /> < title > { title } < / title > < /
        svg >
    }
}
