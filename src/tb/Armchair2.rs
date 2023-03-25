#[cfg(feature = "TbArmchair2")]
use leptos::*;
#[cfg(feature = "TbArmchair2")]
///This icon requires the feature `TbArmchair2` to be enabled.
#[component]
pub fn Armchair2(
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
        "icon icon-tabler icon-tabler-armchair-2" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 10v-4a3 3 0 0 1 3 -3h8a3 3 0 0 1 3 3v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M16 15v-2a3 3 0 1 1 3 3v3h-14v-3a3 3 0 1 1 3 -3v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 12h8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 19v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 19v2" /> < title > { title } < / title > <
        / svg >
    }
}
