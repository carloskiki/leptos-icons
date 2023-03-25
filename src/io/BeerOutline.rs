#[cfg(feature = "IoBeerOutline")]
use leptos::*;
#[cfg(feature = "IoBeerOutline")]
///This icon requires the feature `IoBeerOutline` to be enabled.
#[component]
pub fn BeerOutline(
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
        "M352,200V440a40.12,40.12,0,0,1-40,40H136a40.12,40.12,0,0,1-40-40V224" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M352,224h40a56.16,56.16,0,0,1,56,56v80a56.16,56.16,0,0,1-56,56H352" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "224" y1 = "256" x2 = "224" y2
        = "416" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "288" y1 = "256" x2 = "288" y2
        = "416" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "160" y1 = "256" x2 = "160" y2
        = "416" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M320,112a48,48,0,0,1,0,96c-13.25,0-29.31-7.31-38-16H160c-8,22-27,32-48,32a48,48,0,0,1,0-96,47.91,47.91,0,0,1,26,9"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M91.86,132.43a40,40,0,1,1,60.46-52S160,91,160,96" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M145.83,64.71C163.22,44.89,187.57,32,216,32c52.38,0,94,42.84,94,95.21A95,95,0,0,1,308.33,145"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
