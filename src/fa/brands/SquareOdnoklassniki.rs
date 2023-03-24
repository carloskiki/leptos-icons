#[cfg(feature = "FaBrandsSquareOdnoklassniki")]
use leptos::*;
#[cfg(feature = "FaBrandsSquareOdnoklassniki")]
///This icon requires the feature `FaBrandsSquareOdnoklassniki` to be enabled.
#[component]
pub fn SquareOdnoklassniki(
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
        stroke_witdh = "0" style = style viewBox = "0 0 448 512" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M184.2 177.1c0-22.1 17.9-40 39.8-40s39.8 17.9 39.8 40c0 22-17.9 39.8-39.8 39.8s-39.8-17.9-39.8-39.8zM448 80v352c0 26.5-21.5 48-48 48H48c-26.5 0-48-21.5-48-48V80c0-26.5 21.5-48 48-48h352c26.5 0 48 21.5 48 48zm-305.1 97.1c0 44.6 36.4 80.9 81.1 80.9s81.1-36.2 81.1-80.9c0-44.8-36.4-81.1-81.1-81.1s-81.1 36.2-81.1 81.1zm174.5 90.7c-4.6-9.1-17.3-16.8-34.1-3.6 0 0-22.7 18-59.3 18s-59.3-18-59.3-18c-16.8-13.2-29.5-5.5-34.1 3.6-7.9 16.1 1.1 23.7 21.4 37 17.3 11.1 41.2 15.2 56.6 16.8l-12.9 12.9c-18.2 18-35.5 35.5-47.7 47.7-17.6 17.6 10.7 45.8 28.4 28.6l47.7-47.9c18.2 18.2 35.7 35.7 47.7 47.9 17.6 17.2 46-10.7 28.6-28.6l-47.7-47.7-13-12.9c15.5-1.6 39.1-5.9 56.2-16.8 20.4-13.3 29.3-21 21.5-37z"
        /> < title > { title } < / title > < / svg >
    }
}
