#[cfg(feature = "HiMdSolidCloud")]
use leptos::*;
#[cfg(feature = "HiMdSolidCloud")]
///This icon requires the feature `HiMdSolidCloud` to be enabled.
#[component]
pub fn Cloud(
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
        stroke_witdh = "0" style = style width = "20" height = "20" viewBox = "0 0 20 20"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M1 12.5C1 14.9853 3.01472 17 5.5 17H15C17.2091 17 19 15.2091 19 13C19 11.4649 18.1353 10.1318 16.8664 9.4612C16.9534 9.15579 17 8.83334 17 8.5C17 6.567 15.433 5 13.5 5C13.1017 5 12.7189 5.06655 12.3621 5.18913C11.5758 3.87771 10.1404 3 8.5 3C6.01472 3 4 5.01472 4 7.5C4 7.75044 4.02046 7.9961 4.05979 8.2354C2.28084 8.83595 1 10.5184 1 12.5Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
