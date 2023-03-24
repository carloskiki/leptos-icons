#[cfg(feature = "TbQrcode")]
use leptos::*;
#[cfg(feature = "TbQrcode")]
///This icon requires the feature `TbQrcode` to be enabled.
#[component]
pub fn Qrcode(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-qrcode"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4 4m0 1a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M7 17l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M14 4m0 1a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M7 7l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M4 14m0 1a1 1 0 0 1 1 -1h4a1 1 0 0 1 1 1v4a1 1 0 0 1 -1 1h-4a1 1 0 0 1 -1 -1z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M17 7l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 14l3 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 14l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 14l0 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 20l3 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 17l3 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 17l0 3" /> < title > { title } < / title >
        < / svg >
    }
}
