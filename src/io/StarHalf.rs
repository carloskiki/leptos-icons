#[cfg(feature = "IoStarHalf")]
use leptos::*;
#[cfg(feature = "IoStarHalf")]
///This icon requires the feature `IoStarHalf` to be enabled.
#[component]
pub fn StarHalf(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M480,208H308L256,48,204,208H32l140,96L118,464,256,364,394,464,340,304Z" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< polygon xmlns
        = "http://www.w3.org/2000/svg" points =
        "256 48 256 364 118 464 172 304 32 208 204 208 256 48" /> < title > { title } < /
        title > < / svg >
    }
}
