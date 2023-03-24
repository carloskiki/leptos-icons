#[cfg(feature = "SiServerless")]
use leptos::*;
#[cfg(feature = "SiServerless")]
///This icon requires the feature `SiServerless` to be enabled.
#[component]
pub fn Serverless(
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
        "M0 17.57h4.248l-1.316 3.974H0zm0-7.557h6.75l-1.316 3.974H0zm0-7.557h9.252L7.936 6.431H0zm13.44 0H24v3.975H12.123zM9.62 13.987l1.317-3.974H24v3.974zM8.436 17.57H24v3.975H7.119Z"
        /> < title > { title } < / title > < / svg >
    }
}
