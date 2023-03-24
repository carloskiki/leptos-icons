#[cfg(feature = "IoBagAddOutline")]
use leptos::*;
#[cfg(feature = "IoBagAddOutline")]
///This icon requires the feature `IoBagAddOutline` to be enabled.
#[component]
pub fn BagAddOutline(
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
        stroke_witdh = "0" style = style id = "icons" viewBox = "0 0 512 512" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < line xmlns =
        "http://www.w3.org/2000/svg" x1 = "256" y1 = "256" x2 = "256" y2 = "384" fill =
        "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin = "round"
        stroke - width = "32" />< line xmlns = "http://www.w3.org/2000/svg" x1 = "320" y1
        = "320" x2 = "192" y2 = "320" fill = "none" stroke = "#000" stroke - linecap =
        "round" stroke - linejoin = "round" stroke - width = "32" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M80,176a16,16,0,0,0-16,16V408c0,30.24,25.76,56,56,56H392c30.24,0,56-24.51,56-54.75V192a16,16,0,0,0-16-16Z"
        fill = "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin =
        "round" stroke - width = "32" fill - rule = "evenodd" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M160,176V144a96,96,0,0,1,96-96h0a96,96,0,0,1,96,96v32" fill = "none" stroke =
        "#000" stroke - linecap = "round" stroke - linejoin = "round" stroke - width =
        "32" fill - rule = "evenodd" /> < title > { title } < / title > < / svg >
    }
}
