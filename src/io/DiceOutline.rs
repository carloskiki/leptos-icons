#[cfg(feature = "IoDiceOutline")]
use leptos::*;
#[cfg(feature = "IoDiceOutline")]
///This icon requires the feature `IoDiceOutline` to be enabled.
#[component]
pub fn DiceOutline(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" fill = "none" stroke = "#000" stroke - linecap =
        "round" stroke - linejoin = "round" stroke - width = "32" d =
        "M448,341.37V170.61A32,32,0,0,0,432.11,143l-152-88.46a47.94,47.94,0,0,0-48.24,0L79.89,143A32,32,0,0,0,64,170.61V341.37A32,32,0,0,0,79.89,369l152,88.46a48,48,0,0,0,48.24,0l152-88.46A32,32,0,0,0,448,341.37Z"
        />< polyline xmlns = "http://www.w3.org/2000/svg" fill = "none" stroke = "#000"
        stroke - linecap = "round" stroke - linejoin = "round" stroke - width = "32"
        points = "69 153.99 256 263.99 443 153.99" />< line xmlns =
        "http://www.w3.org/2000/svg" fill = "none" stroke = "#000" stroke - linecap =
        "round" stroke - linejoin = "round" stroke - width = "32" x1 = "256" y1 =
        "463.99" x2 = "256" y2 = "263.99" />< ellipse xmlns =
        "http://www.w3.org/2000/svg" cx = "256" cy = "152" rx = "24" ry = "16" /><
        ellipse xmlns = "http://www.w3.org/2000/svg" cx = "208" cy = "296" rx = "16" ry =
        "24" />< ellipse xmlns = "http://www.w3.org/2000/svg" cx = "112" cy = "328" rx =
        "16" ry = "24" />< ellipse xmlns = "http://www.w3.org/2000/svg" cx = "304" cy =
        "296" rx = "16" ry = "24" />< ellipse xmlns = "http://www.w3.org/2000/svg" cx =
        "400" cy = "240" rx = "16" ry = "24" />< ellipse xmlns =
        "http://www.w3.org/2000/svg" cx = "304" cy = "384" rx = "16" ry = "24" /><
        ellipse xmlns = "http://www.w3.org/2000/svg" cx = "400" cy = "328" rx = "16" ry =
        "24" /> < title > { title } < / title > < / svg >
    }
}
