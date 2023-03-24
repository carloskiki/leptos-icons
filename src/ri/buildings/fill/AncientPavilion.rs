#[cfg(feature = "RiBuildingsFillAncientPavilion")]
use leptos::*;
#[cfg(feature = "RiBuildingsFillAncientPavilion")]
///This icon requires the feature `RiBuildingsFillAncientPavilion` to be enabled.
#[component]
pub fn AncientPavilion(
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
        "M12.513 2.001a9.004 9.004 0 0 0 9.97 5.877A4.501 4.501 0 0 1 19 11.888V19l2 .001v2H3v-2h2v-7.113a4.503 4.503 0 0 1-3.484-4.01 9.004 9.004 0 0 0 9.972-5.876h1.025zM17 12H7V19h10v-7z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
