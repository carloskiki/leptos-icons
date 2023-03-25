#[cfg(feature = "IoAlbumsOutline")]
use leptos::*;
#[cfg(feature = "IoAlbumsOutline")]
///This icon requires the feature `IoAlbumsOutline` to be enabled.
#[component]
pub fn AlbumsOutline(
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
        "http://www.w3.org/2000/svg" > < rect xmlns = "http://www.w3.org/2000/svg" x =
        "64" y = "176" width = "384" height = "256" rx = "28.87" ry = "28.87" style =
        "fill:none;stroke:#000;stroke-linejoin:round;stroke-width:32px" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "144" y1 = "80" x2 = "368" y2 = "80" style =
        "stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px" /><
        line xmlns = "http://www.w3.org/2000/svg" x1 = "112" y1 = "128" x2 = "400" y2 =
        "128" style =
        "stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px" /> <
        title > { title } < / title > < / svg >
    }
}
