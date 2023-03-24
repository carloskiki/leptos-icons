#[cfg(feature = "CgQuoteO")]
use leptos::*;
#[cfg(feature = "CgQuoteO")]
///This icon requires the feature `CgQuoteO` to be enabled.
#[component]
pub fn QuoteO(
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
        "M20 5H4V19H20V5ZM4 3C2.89543 3 2 3.89543 2 5V19C2 20.1046 2.89543 21 4 21H20C21.1046 21 22 20.1046 22 19V5C22 3.89543 21.1046 3 20 3H4Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.06723 9.19629H12.0672L9.93267 14.8038H6.93267L9.06723 9.19629Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.0672 9.19629H17.0672L14.9327 14.8038H11.9327L14.0672 9.19629Z" fill =
        "currentColor" /> < title > { title } < / title > < / svg >
    }
}
