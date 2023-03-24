#[cfg(feature = "SiHp")]
use leptos::*;
#[cfg(feature = "SiHp")]
///This icon requires the feature `SiHp` to be enabled.
#[component]
pub fn Hp(
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
        "M12 24h-.354l2.46-6.745h3.372c.597 0 1.249-.448 1.454-1.007l2.664-7.304c.429-1.192-.242-2.18-1.528-2.18h-4.695l-6.15 16.92C3.933 22.415 0 17.663 0 12 0 6.503 3.708 1.863 8.758.447L2.646 17.255H5.18l3.242-8.926h1.92l-3.243 8.926h2.535l3.037-8.33c.428-1.192-.242-2.18-1.528-2.18H9L11.46.02c.186 0 .354-.019.54-.019 6.634 0 12 5.366 12 12s-5.366 12-12 12zm7.267-15.67h-1.92l-2.682 7.34h1.919z"
        /> < title > { title } < / title > < / svg >
    }
}
