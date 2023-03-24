#[cfg(feature = "TbCoffeeOff")]
use leptos::*;
#[cfg(feature = "TbCoffeeOff")]
///This icon requires the feature `TbCoffeeOff` to be enabled.
#[component]
pub fn CoffeeOff(
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
        "icon icon-tabler icon-tabler-coffee-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 14c.83 .642 2.077 1.017 3.5 1c1.423 .017 2.67 -.358 3.5 -1c.73 -.565 1.783 -.923 3 -.99"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 3c-.194 .14 -.364 .305 -.506 .49" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 3a2.4 2.4 0 0 0 -1 2a2.4 2.4 0 0 0 1 2" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M14 10h3v3m-.257 3.743a6 6 0 0 1 -5.743 4.257h-2a6 6 0 0 1 -6 -6v-5h7" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M20.116 16.124a3 3 0 0 0 -3.118 -4.953"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > {
        title } < / title > < / svg >
    }
}
