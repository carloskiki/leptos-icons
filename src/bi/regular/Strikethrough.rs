#[cfg(feature = "BiRegularStrikethrough")]
use leptos::*;
#[cfg(feature = "BiRegularStrikethrough")]
///This icon requires the feature `BiRegularStrikethrough` to be enabled.
#[component]
pub fn Strikethrough(
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
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M20 11h-8c-4 0-4-1.816-4-2.5C8 7.882 8 6 12 6c2.8 0 2.99 1.678 3 2.014L16 8h1c0-1.384-1.045-4-5-4-5.416 0-6 3.147-6 4.5 0 .728.148 1.667.736 2.5H4v2h16v-2zm-8 7c-3.793 0-3.99-1.815-4-2H6c0 .04.069 4 6 4 5.221 0 6-2.819 6-4.5 0-.146-.009-.317-.028-.5h-2.006c.032.2.034.376.034.5 0 .684 0 2.5-4 2.5z"
        /> < title > { title } < / title > < / svg >
    }
}
