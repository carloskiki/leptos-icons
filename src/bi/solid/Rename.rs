#[cfg(feature = "BiSolidRename")]
use leptos::*;
#[cfg(feature = "BiSolidRename")]
///This icon requires the feature `BiSolidRename` to be enabled.
#[component]
pub fn Rename(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M20 5.995h-1v12h1c1.103 0 2-.897 2-2v-8c0-1.102-.897-2-2-2z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M17 17.995V4h2.995V2h-8v2H15v1.995H4c-1.103 0-2 .897-2 2v8c0 1.103.897 2 2 2h11V20h-3.005v2h8v-2H17v-2.005zm-11-4v-4h9v4H6z"
        /> < title > { title } < / title > < / svg >
    }
}
