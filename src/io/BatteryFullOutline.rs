#[cfg(feature = "IoBatteryFullOutline")]
use leptos::*;
#[cfg(feature = "IoBatteryFullOutline")]
///This icon requires the feature `IoBatteryFullOutline` to be enabled.
#[component]
pub fn BatteryFullOutline(
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
        "32" y = "144" width = "400" height = "224" rx = "45.7" ry = "45.7" style =
        "fill:none;stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "85.69" y = "198.93" width =
        "292.63" height = "114.14" rx = "4" ry = "4" style =
        "stroke:#000;stroke-linecap:square;stroke-miterlimit:10;stroke-width:32px" /><
        line xmlns = "http://www.w3.org/2000/svg" x1 = "480" y1 = "218.67" x2 = "480" y2
        = "293.33" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-miterlimit:10;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
