#[cfg(feature = "CgCompressV")]
use leptos::*;
#[cfg(feature = "CgCompressV")]
///This icon requires the feature `CgCompressV` to be enabled.
#[component]
pub fn CompressV(
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
        "M13.0338 7.3559L15.4999 4.85376L16.9244 6.25768L12.0107 11.2432L7.02515 6.32948L8.42907 4.90505L11.0329 7.47139L11.0834 0.843506L13.0833 0.858735L13.0338 7.3559Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.5627 18.532L16.9749 17.1159L12.0184 12.1729L7.07543 17.1295L8.49159 18.5418L11.0762 15.95L11.1019 23.1566L13.1019 23.1495L13.0765 16.0528L15.5627 18.532Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
