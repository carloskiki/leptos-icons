#[cfg(feature = "IoBatteryHalfSharp")]
use leptos::*;
#[cfg(feature = "IoBatteryHalfSharp")]
///This icon requires the feature `IoBatteryHalfSharp` to be enabled.
#[component]
pub fn BatteryHalfSharp(
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
        "M17,384H449V128H17ZM49,160H417V352H49Z" />< rect xmlns =
        "http://www.w3.org/2000/svg" x = "70.69" y = "182.92" width = "185.31" height =
        "146.16" />< rect xmlns = "http://www.w3.org/2000/svg" x = "465" y = "202.67"
        width = "32" height = "106.67" /> < title > { title } < / title > < / svg >
    }
}
