#[cfg(feature = "TbRecharging")]
use leptos::*;
#[cfg(feature = "TbRecharging")]
///This icon requires the feature `TbRecharging` to be enabled.
#[component]
pub fn Recharging(
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
        "icon icon-tabler icon-tabler-recharging" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.038 4.5a9 9 0 0 0 -2.495 2.47" />< path xmlns = "http://www.w3.org/2000/svg"
        d = "M3.186 10.209a9 9 0 0 0 0 3.508" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4.5 16.962a9 9 0 0 0 2.47 2.495" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M10.209 20.814a9 9 0 0 0 3.5 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.962 19.5a9 9 0 0 0 2.495 -2.47" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20.814 13.791a9 9 0 0 0 0 -3.508" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M19.5 7.038a9 9 0 0 0 -2.47 -2.495" /><
        path xmlns = "http://www.w3.org/2000/svg" d =
        "M13.791 3.186a9 9 0 0 0 -3.508 -.02" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 8l-2 4h4l-2 4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 21a9 9 0 0 0 0 -18" /> < title > { title }
        < / title > < / svg >
    }
}
