#[cfg(feature = "CgCornerUpRight")]
use leptos::*;
#[cfg(feature = "CgCornerUpRight")]
///This icon requires the feature `CgCornerUpRight` to be enabled.
#[component]
pub fn CornerUpRight(
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
        "M13.3724 14.7219L14.7835 16.1391L21.1612 9.78892L14.811 3.41125L13.3937 4.82242L17.1702 8.61526L6.86461 8.59304C4.65547 8.58828 2.86076 10.3753 2.85599 12.5844L2.83875 20.5844L4.83874 20.5887L4.85599 12.5887C4.85837 11.4841 5.75573 10.5907 6.8603 10.593L17.496 10.616L13.3724 14.7219Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
