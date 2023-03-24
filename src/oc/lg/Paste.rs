#[cfg(feature = "OcLgPaste")]
use leptos::*;
#[cfg(feature = "OcLgPaste")]
///This icon requires the feature `OcLgPaste` to be enabled.
#[component]
pub fn Paste(
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
        "M5.962 2.513a.75.75 0 0 1-.475.949l-.816.272a.25.25 0 0 0-.171.237V21.25c0 .138.112.25.25.25h14.5a.25.25 0 0 0 .25-.25V3.97a.25.25 0 0 0-.17-.236l-.817-.272a.75.75 0 0 1 .474-1.424l.816.273A1.751 1.751 0 0 1 21 3.97v17.28A1.75 1.75 0 0 1 19.25 23H4.75A1.75 1.75 0 0 1 3 21.25V3.97a1.75 1.75 0 0 1 1.197-1.66l.816-.272a.75.75 0 0 1 .949.475Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7 1.75C7 .784 7.784 0 8.75 0h6.5C16.216 0 17 .784 17 1.75v1.5A1.75 1.75 0 0 1 15.25 5h-6.5A1.75 1.75 0 0 1 7 3.25Zm1.75-.25a.25.25 0 0 0-.25.25v1.5c0 .138.112.25.25.25h6.5a.25.25 0 0 0 .25-.25v-1.5a.25.25 0 0 0-.25-.25Z"
        /> < title > { title } < / title > < / svg >
    }
}
