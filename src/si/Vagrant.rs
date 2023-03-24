#[cfg(feature = "SiVagrant")]
use leptos::*;
#[cfg(feature = "SiVagrant")]
///This icon requires the feature `SiVagrant` to be enabled.
#[component]
pub fn Vagrant(
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
        "M3.556 0L.392 1.846V4.11l7.124 17.3L11.998 24l4.523-2.611 7.083-17.345V1.848l.004-.002L20.44 0l-5.274 3.087v2.111l-3.168 7.384-3.164-7.384V3.109l-.017-.008.017-.01z"
        /> < title > { title } < / title > < / svg >
    }
}
