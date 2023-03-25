#[cfg(feature = "IoGameControllerOutline")]
use leptos::*;
#[cfg(feature = "IoGameControllerOutline")]
///This icon requires the feature `IoGameControllerOutline` to be enabled.
#[component]
pub fn GameControllerOutline(
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
        "M467.51,248.83c-18.4-83.18-45.69-136.24-89.43-149.17A91.5,91.5,0,0,0,352,96c-26.89,0-48.11,16-96,16s-69.15-16-96-16a99.09,99.09,0,0,0-27.2,3.66C89,112.59,61.94,165.7,43.33,248.83c-19,84.91-15.56,152,21.58,164.88,26,9,49.25-9.61,71.27-37,25-31.2,55.79-40.8,119.82-40.8s93.62,9.6,118.66,40.8c22,27.41,46.11,45.79,71.42,37.16C487.1,399.86,486.52,334.74,467.51,248.83Z"
        style = "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< circle
        xmlns = "http://www.w3.org/2000/svg" cx = "292" cy = "224" r = "20" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M336,288a20,20,0,1,1,20-19.95A20,20,0,0,1,336,288Z" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "336" cy = "180" r = "20" />< circle xmlns =
        "http://www.w3.org/2000/svg" cx = "380" cy = "224" r = "20" />< line xmlns =
        "http://www.w3.org/2000/svg" x1 = "160" y1 = "176" x2 = "160" y2 = "272" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "208" y1 = "224" x2 = "112" y2
        = "224" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
