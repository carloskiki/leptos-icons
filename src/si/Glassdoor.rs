#[cfg(feature = "SiGlassdoor")]
use leptos::*;
#[cfg(feature = "SiGlassdoor")]
///This icon requires the feature `SiGlassdoor` to be enabled.
#[component]
pub fn Glassdoor(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M17.144 20.572H3.43C3.43 22.465 4.963 24 6.856 24h10.286c1.893 0 3.428-1.535 3.428-3.428V6.492c0-.07-.054-.125-.124-.125h-3.18c-.067 0-.123.056-.123.126v14.08zm0-20.572c1.892 0 3.427 1.535 3.427 3.43H6.858v14.078c0 .068-.056.125-.125.125H3.554c-.07 0-.125-.057-.125-.125V3.428C3.43 1.536 4.963 0 6.856 0h10.287"
        /> < title > { title } < / title > < / svg >
    }
}
