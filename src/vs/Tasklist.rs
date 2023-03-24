#[cfg(feature = "VsTasklist")]
use leptos::*;
#[cfg(feature = "VsTasklist")]
///This icon requires the feature `VsTasklist` to be enabled.
#[component]
pub fn Tasklist(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" fill - rule = "evenodd" clip - rule = "evenodd" d =
        "M3.57 6.699l5.693-4.936L8.585 1 3.273 5.596l-1.51-1.832L1 4.442l1.85 2.214.72.043zM15 5H6.824l2.307-2H15v2zM6 7h9v2H6V7zm9 4H6v2h9v-2z"
        /> < title > { title } < / title > < / svg >
    }
}
