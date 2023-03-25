#[cfg(feature = "RiBusinessFillAdvertisement")]
use leptos::*;
#[cfg(feature = "RiBusinessFillAdvertisement")]
///This icon requires the feature `RiBusinessFillAdvertisement` to be enabled.
#[component]
pub fn Advertisement(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M21 3a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h18zM9.399 8h-2l-3.2 8h2.154l.4-1h3.29l.4 1h2.155L9.399 8zM19 8h-2v2h-1a3 3 0 0 0-.176 5.995L16 16h3V8zm-2 4v2h-1l-.117-.007a1 1 0 0 1 0-1.986L16 12h1zm-8.601-1.115L9.244 13H7.552l.847-2.115z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
