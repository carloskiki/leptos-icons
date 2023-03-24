#[cfg(feature = "BiSolidCabinet")]
use leptos::*;
#[cfg(feature = "BiSolidCabinet")]
///This icon requires the feature `BiSolidCabinet` to be enabled.
#[component]
pub fn Cabinet(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M21 4c0-1.103-.897-2-2-2H5c-1.103 0-2 .897-2 2v7h18V4zm-5 4H8V5h2v1h4V5h2v3zM5 22h14c1.103 0 2-.897 2-2v-7H3v7c0 1.103.897 2 2 2zm3-6h2v1h4v-1h2v3H8v-3z"
        /> < title > { title } < / title > < / svg >
    }
}
