#[cfg(feature = "TbNetworkOff")]
use leptos::*;
#[cfg(feature = "TbNetworkOff")]
///This icon requires the feature `TbNetworkOff` to be enabled.
#[component]
pub fn NetworkOff(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-network-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.537 6.516a6 6 0 0 0 7.932 7.954m2.246 -1.76a6 6 0 0 0 -8.415 -8.433" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M12 3c1.333 .333 2 2.333 2 6c0 .348 0 .681 -.018 1m-.545 3.43c-.332 .89 -.811 1.414 -1.437 1.57"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 3c-.938 .234 -1.547 1.295 -1.825 3.182m-.156 3.837c.117 3.02 .777 4.68 1.981 4.981"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M6 9h3m4 0h5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 19h7" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 19h5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 19m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M12 15v2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
