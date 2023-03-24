#[cfg(feature = "CgMaximizeAlt")]
use leptos::*;
#[cfg(feature = "CgMaximizeAlt")]
///This icon requires the feature `CgMaximizeAlt` to be enabled.
#[component]
pub fn MaximizeAlt(
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
        "http://www.w3.org/2000/svg" d =
        "M3 3H9V5H6.46173L11.3047 9.84298L9.8905 11.2572L5 6.3667V9H3V3Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 21H9V19H6.3764L11.3046 14.0718L9.89038 12.6576L5 17.548V15H3V21Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 21H21V15H19V17.5244L14.1332 12.6576L12.719 14.0718L17.6472 19H15V21Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 3H15V5H17.5619L12.7189 9.84301L14.1331 11.2572L19 6.39032V9H21V3Z" fill =
        "currentColor" /> < title > { title } < / title > < / svg >
    }
}
