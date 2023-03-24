#[cfg(feature = "SiZeromq")]
use leptos::*;
#[cfg(feature = "SiZeromq")]
///This icon requires the feature `SiZeromq` to be enabled.
#[component]
pub fn Zeromq(
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
        "M22.088,5.499l1.894-1.894l-3.587-3.587l-1.894,1.894C16.627,0.702,14.396,0,12,0C5.373,0,0,5.373,0,12c0,2.396,0.702,4.627,1.912,6.501l-1.894,1.894l3.587,3.587l1.894-1.894C7.373,23.298,9.604,24,12,24c6.627,0,12-5.373,12-12C24,9.604,23.298,7.373,22.088,5.499z M4.569,12c0-4.104,3.327-7.431,7.431-7.431c1.125,0,2.191,0.25,3.146,0.698l-9.88,9.88C4.819,14.191,4.569,13.125,4.569,12z M12,19.431c-1.125,0-2.191-0.25-3.146-0.698l9.88-9.88c0.447,0.956,0.698,2.022,0.698,3.146C19.431,16.104,16.104,19.431,12,19.431z"
        /> < title > { title } < / title > < / svg >
    }
}
