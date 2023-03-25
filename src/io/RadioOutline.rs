#[cfg(feature = "IoRadioOutline")]
use leptos::*;
#[cfg(feature = "IoRadioOutline")]
///This icon requires the feature `IoRadioOutline` to be enabled.
#[component]
pub fn RadioOutline(
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
        "256" cy = "256.02" r = "32" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M184.25,192.25a96,96,0,0,0,0,127.52" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M327.77,319.77a96,96,0,0,0,0-127.52" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M133.28,141.28a168,168,0,0,0,0,229.44" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M378.72,370.72a168,168,0,0,0,0-229.44" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M435,416a240.34,240.34,0,0,0,0-320" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M77,96a240.34,240.34,0,0,0,0,320" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        /> < title > { title } < / title > < / svg >
    }
}
