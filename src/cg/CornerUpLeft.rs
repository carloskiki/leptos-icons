#[cfg(feature = "CgCornerUpLeft")]
use leptos::*;
#[cfg(feature = "CgCornerUpLeft")]
///This icon requires the feature `CgCornerUpLeft` to be enabled.
#[component]
pub fn CornerUpLeft(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.6276 14.7219L9.21641 16.1391L2.83875 9.78892L9.18897 3.41125L10.6062 4.82242L6.82971 8.61526L17.1353 8.59304C19.3445 8.58828 21.1392 10.3753 21.144 12.5844L21.1612 20.5844L19.1612 20.5887L19.144 12.5887C19.1416 11.4841 18.2442 10.5907 17.1396 10.593L6.50391 10.616L10.6276 14.7219Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
