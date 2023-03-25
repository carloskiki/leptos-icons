#[cfg(feature = "TbBellPlus")]
use leptos::*;
#[cfg(feature = "TbBellPlus")]
///This icon requires the feature `TbBellPlus` to be enabled.
#[component]
pub fn BellPlus(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-bell-plus"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 5a2 2 0 1 1 4 0a7 7 0 0 1 4 6v3a4 4 0 0 0 2 3h-16a4 4 0 0 0 2 -3v-3a7 7 0 0 1 4 -6"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M9 17v1a3 3 0 0 0 6 0v-1" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M10 11l4 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 9l0 4" /> < title > { title } < / title > <
        / svg >
    }
}
