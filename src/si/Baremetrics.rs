#[cfg(feature = "SiBaremetrics")]
use leptos::*;
#[cfg(feature = "SiBaremetrics")]
///This icon requires the feature `SiBaremetrics` to be enabled.
#[component]
pub fn Baremetrics(
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
        "M22.109 7.951l1.485 2.464a3.507 3.507 0 010 3.275l-4.505 7.717a3.333 3.333 0 01-2.94 1.793H7.83a3.335 3.335 0 01-2.94-1.793l-1.555-2.632 6.139-5.695 4.447 2.578a1.093 1.093 0 001.456-.198zm-13.39.628L1.99 16.15.406 13.725a3.495 3.495 0 010-3.27L5.158 2.59A3.338 3.338 0 018.1.8h8.008c1.228 0 2.357.687 2.942 1.79l1.616 2.722-6.017 5.592-4.432-2.574a1.098 1.098 0 00-1.499.248z"
        /> < title > { title } < / title > < / svg >
    }
}
