#[cfg(feature = "SiKoyeb")]
use leptos::*;
#[cfg(feature = "SiKoyeb")]
///This icon requires the feature `SiKoyeb` to be enabled.
#[component]
pub fn Koyeb(
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
        "M0 12.822V7.044L11.985.115 24 7.037v5.77L11.992 5.892Zm11.985 1.114L1.92 19.759 0 18.645v-3.557l11.985-6.93L24 15.089v3.542l-1.92 1.13Zm-3.028 9.949L3.95 21.004l8.036-4.656 8.066 4.656-5.009 2.88-3.05-1.759Z"
        /> < title > { title } < / title > < / svg >
    }
}
