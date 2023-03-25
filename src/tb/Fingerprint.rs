#[cfg(feature = "TbFingerprint")]
use leptos::*;
#[cfg(feature = "TbFingerprint")]
///This icon requires the feature `TbFingerprint` to be enabled.
#[component]
pub fn Fingerprint(
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
        "icon icon-tabler icon-tabler-fingerprint" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.9 7a8 8 0 0 1 1.1 5v1a6 6 0 0 0 .8 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 11a4 4 0 0 1 8 0v1a10 10 0 0 0 2 6" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M12 11v2a14 14 0 0 0 2.5 8" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M8 15a18 18 0 0 0 1.8 6" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M4.9 19a22 22 0 0 1 -.9 -7v-1a8 8 0 0 1 12 -6.95" /> < title > { title } < /
        title > < / svg >
    }
}
