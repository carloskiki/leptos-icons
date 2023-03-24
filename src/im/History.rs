#[cfg(feature = "ImHistory")]
use leptos::*;
#[cfg(feature = "ImHistory")]
///This icon requires the feature `ImHistory` to be enabled.
#[component]
pub fn History(
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
        stroke_witdh = "0" style = style version = "1.1" width = "17" height = "16"
        viewBox = "0 0 17 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M10 1c3.866 0 7 3.134 7 7s-3.134 7-7 7v-1.5c1.469 0 2.85-0.572 3.889-1.611s1.611-2.42 1.611-3.889c0-1.469-0.572-2.85-1.611-3.889s-2.42-1.611-3.889-1.611c-1.469 0-2.85 0.572-3.889 1.611-0.799 0.799-1.322 1.801-1.52 2.889h2.909l-3.5 4-3.5-4h2.571c0.485-3.392 3.402-6 6.929-6zM13 7v2h-4v-5h2v3z"
        /> < title > { title } < / title > < / svg >
    }
}
