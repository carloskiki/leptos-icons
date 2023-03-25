#[cfg(feature = "TbHomeSearch")]
use leptos::*;
#[cfg(feature = "TbHomeSearch")]
///This icon requires the feature `TbHomeSearch` to be enabled.
#[component]
pub fn HomeSearch(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-home-search" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 12l-9 -9l-9 9h2v7a2 2 0 0 0 2 2h4.7" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 21v-6a2 2 0 0 1 2 -2h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 18m-3 0a3 3 0 1 0 6 0a3 3 0 1 0 -6 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M20.2 20.2l1.8 1.8" /> < title > {
        title } < / title > < / svg >
    }
}
