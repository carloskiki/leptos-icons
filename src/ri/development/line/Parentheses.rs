#[cfg(feature = "RiDevelopmentLineParentheses")]
use leptos::*;
#[cfg(feature = "RiDevelopmentLineParentheses")]
///This icon requires the feature `RiDevelopmentLineParentheses` to be enabled.
#[component]
pub fn Parentheses(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path d =
        "M6.923 21C5.113 18.664 4 15.493 4 12c0-3.493 1.113-6.664 2.923-9h2.014C7.235 5.388 6.2 8.542 6.2 12s1.035 6.612 2.737 9H6.923zm10.151 0H15.06c1.702-2.388 2.737-5.542 2.737-9s-1.035-6.612-2.737-9h2.014c1.81 2.336 2.923 5.507 2.923 9 0 3.493-1.112 6.664-2.923 9z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
