#[cfg(feature = "IoReorderTwoSharp")]
use leptos::*;
#[cfg(feature = "IoReorderTwoSharp")]
///This icon requires the feature `IoReorderTwoSharp` to be enabled.
#[component]
pub fn ReorderTwoSharp(
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
        "http://www.w3.org/2000/svg" > < line xmlns = "http://www.w3.org/2000/svg" x1 =
        "118" y1 = "304" x2 = "394" y2 = "304" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:44px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "118" y1 = "208" x2 = "394" y2
        = "208" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-linejoin:round;stroke-width:44px"
        /> < title > { title } < / title > < / svg >
    }
}
