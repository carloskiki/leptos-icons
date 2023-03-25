#[cfg(feature = "TbPlanetOff")]
use leptos::*;
#[cfg(feature = "TbPlanetOff")]
///This icon requires the feature `TbPlanetOff` to be enabled.
#[component]
pub fn PlanetOff(
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
        "icon icon-tabler icon-tabler-planet-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.816 13.58c1.956 1.825 3.157 3.449 3.184 4.445m-3.428 .593c-2.098 -.634 -4.944 -2.03 -7.919 -3.976c-5.47 -3.579 -9.304 -7.664 -8.56 -9.123c.32 -.628 1.591 -.6 3.294 -.113"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.042 7.059a7 7 0 0 0 9.908 9.89m1.581 -2.425a7 7 0 0 0 -9.057 -9.054" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < /
        title > < / svg >
    }
}
