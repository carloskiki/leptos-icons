#[cfg(feature = "TbDropletFilled")]
use leptos::*;
#[cfg(feature = "TbDropletFilled")]
///This icon requires the feature `TbDropletFilled` to be enabled.
#[component]
pub fn DropletFilled(
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
        "icon icon-tabler icon-tabler-droplet-filled" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6.801 11.003a6 6 0 1 0 10.396 -.003l-5.197 -8l-5.199 8.003z" stroke = "#010202"
        stroke - width = "0" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 3v17" stroke - width = "0" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12l3.544 -3.544" stroke - width = "0" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 17.3l5.558 -5.558" stroke - width = "0"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
