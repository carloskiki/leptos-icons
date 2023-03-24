#[cfg(feature = "OcLgBug")]
use leptos::*;
#[cfg(feature = "OcLgBug")]
///This icon requires the feature `OcLgBug` to be enabled.
#[component]
pub fn Bug(
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
        "M7.72.22a.75.75 0 0 1 1.06 0l1.204 1.203A4.98 4.98 0 0 1 12 1c.717 0 1.4.151 2.016.423L15.22.22a.751.751 0 0 1 1.042.018.751.751 0 0 1 .018 1.042l-.971.972A4.991 4.991 0 0 1 17 6v1.104a2.755 2.755 0 0 1 1.917 1.974l1.998-.999a.75.75 0 0 1 .67 1.342L19 10.714V13.5l3.25.003a.75.75 0 0 1 0 1.5L19 15.001V16c0 .568-.068 1.134-.204 1.686l.04.018 2.75 1.375a.75.75 0 1 1-.671 1.342l-2.638-1.319A6.998 6.998 0 0 1 12 23a6.998 6.998 0 0 1-6.197-3.742l-2.758 1.181a.752.752 0 0 1-1.064-.776.752.752 0 0 1 .474-.602l2.795-1.199A6.976 6.976 0 0 1 5 16v-.996H1.75a.75.75 0 0 1 0-1.5H5v-2.79L2.415 9.42a.75.75 0 0 1 .67-1.342l1.998.999A2.756 2.756 0 0 1 7 7.104V6a4.99 4.99 0 0 1 1.69-3.748l-.97-.972a.75.75 0 0 1 0-1.06ZM6.5 9.75V16a5.5 5.5 0 1 0 11 0V9.75c0-.69-.56-1.25-1.25-1.25h-8.5c-.69 0-1.25.56-1.25 1.25ZM8.5 7h7V6a3.5 3.5 0 1 0-7 0Z"
        /> < title > { title } < / title > < / svg >
    }
}
