#[cfg(feature = "CgPentagonTopLeft")]
use leptos::*;
#[cfg(feature = "CgPentagonTopLeft")]
///This icon requires the feature `CgPentagonTopLeft` to be enabled.
#[component]
pub fn PentagonTopLeft(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M14.3301 15.1601L11 19.3922L6 10.7319L9.33013 6.49988L14.6603 5.73193L19.6603 14.3922L14.3301 15.1601ZM16.4512 12.8339L13.2531 13.2947L11.255 15.8339L8.40908 10.9046L10.4072 8.36536L13.6052 7.90459L16.4512 12.8339Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
