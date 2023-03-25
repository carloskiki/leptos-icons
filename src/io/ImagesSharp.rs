#[cfg(feature = "IoImagesSharp")]
use leptos::*;
#[cfg(feature = "IoImagesSharp")]
///This icon requires the feature `IoImagesSharp` to be enabled.
#[component]
pub fn ImagesSharp(
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
        "http://www.w3.org/2000/svg" > < ellipse xmlns = "http://www.w3.org/2000/svg" cx
        = "373.14" cy = "219.33" rx = "46.29" ry = "46" style = "fill:none" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M80,132V460a20,20,0,0,0,20,20H492a20,20,0,0,0,20-20V132a20,20,0,0,0-20-20H100A20,20,0,0,0,80,132Zm293.14,41.33a46,46,0,1,1-46.28,46A46.19,46.19,0,0,1,373.14,173.33Zm-261.41,276V353.85l122.76-110.2L328.27,337l-113,112.33Zm368.27,0H259l144.58-144L480,370.59Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20,32A20,20,0,0,0,0,52V396a20,20,0,0,0,20,20H48V100A20,20,0,0,1,68,80H448V52a20,20,0,0,0-20-20Z"
        /> < title > { title } < / title > < / svg >
    }
}
