#[cfg(feature = "SiPaloaltosoftware")]
use leptos::*;
#[cfg(feature = "SiPaloaltosoftware")]
///This icon requires the feature `SiPaloaltosoftware` to be enabled.
#[component]
pub fn Paloaltosoftware(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M11.995.005c-.58 0-1.158.228-1.615.685L.685 10.385c-.913.913-.913 2.318 0 3.231l.842.843 8.01-8.15a3.435 3.435 0 0 1 4.847 0l8.079 8.08.842-.843c.914-.843.915-2.248.072-3.161L13.612.69a2.279 2.279 0 0 0-1.617-.685zm0 6.463c-.58 0-1.158.228-1.615.684L.685 16.848c-.913.913-.913 2.318 0 3.23l3.231 3.232c.914.913 2.318.913 3.232 0l4.847-4.846 4.848 4.846c.913.913 2.318.913 3.231 0l3.231-3.231c.914-.843.915-2.318.072-3.231l-9.765-9.696a2.279 2.279 0 0 0-1.617-.684z"
        /> < title > { title } < / title > < / svg >
    }
}
