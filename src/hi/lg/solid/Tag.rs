#[cfg(feature = "HiLgSolidTag")]
use leptos::*;
#[cfg(feature = "HiLgSolidTag")]
///This icon requires the feature `HiLgSolidTag` to be enabled.
#[component]
pub fn Tag(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M5.25 2.25C3.59315 2.25 2.25 3.59315 2.25 5.25V9.56802C2.25 10.3637 2.56607 11.1267 3.12868 11.6893L12.7098 21.2705C13.6291 22.1898 15.0989 22.4564 16.2573 21.698C18.4242 20.2793 20.2793 18.4242 21.698 16.2573C22.4564 15.0989 22.1898 13.6291 21.2705 12.7098L11.6893 3.12868C11.1267 2.56607 10.3637 2.25 9.56802 2.25H5.25ZM6.375 7.5C6.99632 7.5 7.5 6.99632 7.5 6.375C7.5 5.75368 6.99632 5.25 6.375 5.25C5.75368 5.25 5.25 5.75368 5.25 6.375C5.25 6.99632 5.75368 7.5 6.375 7.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
