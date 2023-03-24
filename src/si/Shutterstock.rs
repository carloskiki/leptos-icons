#[cfg(feature = "SiShutterstock")]
use leptos::*;
#[cfg(feature = "SiShutterstock")]
///This icon requires the feature `SiShutterstock` to be enabled.
#[component]
pub fn Shutterstock(
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
        "M9.839 18.761h5.313a1.53 1.53 0 0 0 1.527-1.528v-5.76h5.237v5.76A6.767 6.767 0 0 1 15.152 24H9.839v-5.239M14.16 5.237H8.85a1.53 1.53 0 0 0-1.53 1.527v5.761H2.085V6.764A6.763 6.763 0 0 1 8.85 0h5.31v5.237Z"
        /> < title > { title } < / title > < / svg >
    }
}
