#[cfg(feature = "SiRoblox")]
use leptos::*;
#[cfg(feature = "SiRoblox")]
///This icon requires the feature `SiRoblox` to be enabled.
#[component]
pub fn Roblox(
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
        "m13.383 14.341-3.726-.958.959-3.726 3.726.959-.96 3.726zM4.913 0 0 19.088 19.088 24 24 4.912 4.912 0z"
        /> < title > { title } < / title > < / svg >
    }
}
