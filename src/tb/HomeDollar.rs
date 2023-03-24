#[cfg(feature = "TbHomeDollar")]
use leptos::*;
#[cfg(feature = "TbHomeDollar")]
///This icon requires the feature `TbHomeDollar` to be enabled.
#[component]
pub fn HomeDollar(
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
        "icon icon-tabler icon-tabler-home-dollar" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 10l-7 -7l-9 9h2v7a2 2 0 0 0 2 2h6" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M9 21v-6a2 2 0 0 1 2 -2h2c.387 0 .748 .11 1.054 .3" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M21 15h-2.5a1.5 1.5 0 0 0 0 3h1a1.5 1.5 0 0 1 0 3h-2.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19 21v1m0 -8v1" /> < title > { title } < /
        title > < / svg >
    }
}
