#[cfg(feature = "FaSolidFilePowerpoint")]
use leptos::*;
#[cfg(feature = "FaSolidFilePowerpoint")]
///This icon requires the feature `FaSolidFilePowerpoint` to be enabled.
#[component]
pub fn FilePowerpoint(
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
        stroke_witdh = "0" style = style viewBox = "0 0 384 512" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M64 0C28.7 0 0 28.7 0 64V448c0 35.3 28.7 64 64 64H320c35.3 0 64-28.7 64-64V160H256c-17.7 0-32-14.3-32-32V0H64zM256 0V128H384L256 0zM136 240h68c42 0 76 34 76 76s-34 76-76 76H160v32c0 13.3-10.7 24-24 24s-24-10.7-24-24V368 264c0-13.3 10.7-24 24-24zm68 104c15.5 0 28-12.5 28-28s-12.5-28-28-28H160v56h44z"
        /> < title > { title } < / title > < / svg >
    }
}
