#[cfg(feature = "IoBandageOutline")]
use leptos::*;
#[cfg(feature = "IoBandageOutline")]
///This icon requires the feature `IoBandageOutline` to be enabled.
#[component]
pub fn BandageOutline(
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
        "-24.43" y = "167.88" width = "560.87" height = "176.25" rx = "88.12" ry =
        "88.12" transform = "translate(-106.04 256) rotate(-45)" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< rect xmlns = "http://www.w3.org/2000/svg" x = "169.41" y = "156.59" width =
        "176" height = "196" rx = "32" ry = "32" transform =
        "translate(255.41 -107.45) rotate(45)" style =
        "fill:none;stroke:#000;stroke-linecap:round;stroke-linejoin:round;stroke-width:32px"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "256" cy = "208" r = "16"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "304" cy = "256" r = "16"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "208" cy = "256" r = "16"
        />< circle xmlns = "http://www.w3.org/2000/svg" cx = "256" cy = "304" r = "16" />
        < title > { title } < / title > < / svg >
    }
}
