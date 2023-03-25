#[cfg(feature = "IoNuclearOutline")]
use leptos::*;
#[cfg(feature = "IoNuclearOutline")]
///This icon requires the feature `IoNuclearOutline` to be enabled.
#[component]
pub fn NuclearOutline(
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
        "http://www.w3.org/2000/svg" > < circle xmlns = "http://www.w3.org/2000/svg" cx =
        "256" cy = "256" r = "192" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "256" cy = "256" r = "64"
        style = "fill:none;stroke:#000;stroke-miterlimit:10;stroke-width:32px" />< line
        xmlns = "http://www.w3.org/2000/svg" x1 = "224" y1 = "192" x2 = "171" y2 = "85"
        style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "288" y1 = "192" x2 = "341" y2
        = "85" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "327.55" y1 = "255.81" x2 =
        "446.96" y2 = "255.94" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "299.09" y1 = "313.13" x2 =
        "371.34" y2 = "408.19" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "184.45" y1 = "255.81" x2 =
        "65.04" y2 = "255.94" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        />< line xmlns = "http://www.w3.org/2000/svg" x1 = "212.91" y1 = "313.13" x2 =
        "140.66" y2 = "408.19" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
