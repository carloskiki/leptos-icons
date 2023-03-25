#[cfg(feature = "IoCellularSharp")]
use leptos::*;
#[cfg(feature = "IoCellularSharp")]
///This icon requires the feature `IoCellularSharp` to be enabled.
#[component]
pub fn CellularSharp(
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
        "M496,432H400V80h96Z" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M368,432H272V160h96Z" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M240,432H144V224h96Z" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M112,432H16V288h96Z" /> < title > { title } < / title > < / svg >
    }
}
