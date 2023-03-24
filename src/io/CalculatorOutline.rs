#[cfg(feature = "IoCalculatorOutline")]
use leptos::*;
#[cfg(feature = "IoCalculatorOutline")]
///This icon requires the feature `IoCalculatorOutline` to be enabled.
#[component]
pub fn CalculatorOutline(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < rect xmlns = "http://www.w3.org/2000/svg" x =
        "112" y = "48" width = "288" height = "416" rx = "32" ry = "32" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "160.01" y = "112" width =
        "191.99" height = "64" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "168" cy = "248" r = "24"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "256" cy = "248" r = "24"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "344" cy = "248" r = "24"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "168" cy = "328" r = "24"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "256" cy = "328" r = "24"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "168" cy = "408" r = "24"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "256" cy = "408" r = "24"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "320" y = "304" width = "48"
        height = "128" rx = "24" ry = "24" /> < title > { title } < / title > < / svg >
    }
}
