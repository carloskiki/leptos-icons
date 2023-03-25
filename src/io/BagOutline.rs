#[cfg(feature = "IoBagOutline")]
use leptos::*;
#[cfg(feature = "IoBagOutline")]
///This icon requires the feature `IoBagOutline` to be enabled.
#[component]
pub fn BagOutline(
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
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" fill = "none" stroke = "#000" stroke - linecap =
        "round" stroke - linejoin = "round" stroke - width = "32" fill - rule = "evenodd"
        d =
        "M80,176a16,16,0,0,0-16,16V408c0,30.24,25.76,56,56,56H392c30.24,0,56-24.51,56-54.75V192a16,16,0,0,0-16-16Z"
        />< path xmlns = "http://www.w3.org/2000/svg" fill = "none" stroke = "#000"
        stroke - linecap = "round" stroke - linejoin = "round" stroke - width = "32" fill
        - rule = "evenodd" d = "M160,176V144a96,96,0,0,1,96-96h0a96,96,0,0,1,96,96v32" />
        < title > { title } < / title > < / svg >
    }
}
