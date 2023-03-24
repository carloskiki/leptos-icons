#[cfg(feature = "ImSigma")]
use leptos::*;
#[cfg(feature = "ImSigma")]
///This icon requires the feature `ImSigma` to be enabled.
#[component]
pub fn Sigma(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M14.713 11.48l0.694-1.48h0.594l-1 6h-15v-1.16l5.18-6.113-5.18-5.18v-3.547h15.313l0.688 4h-0.537l-0.293-0.607c-0.552-1.146-0.967-1.393-2.17-1.393h-10.344l5.517 5.516-4.647 5.483h8.474c1.813 0 2.291-0.65 2.713-1.52z"
        /> < title > { title } < / title > < / svg >
    }
}
