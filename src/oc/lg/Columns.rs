#[cfg(feature = "OcLgColumns")]
use leptos::*;
#[cfg(feature = "OcLgColumns")]
///This icon requires the feature `OcLgColumns` to be enabled.
#[component]
pub fn Columns(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M3.75 2h5.5c.966 0 1.75.784 1.75 1.75v16.5A1.75 1.75 0 0 1 9.25 22h-5.5A1.75 1.75 0 0 1 2 20.25V3.75C2 2.784 2.784 2 3.75 2Zm11 0h5.5c.966 0 1.75.784 1.75 1.75v16.5A1.75 1.75 0 0 1 20.25 22h-5.5A1.75 1.75 0 0 1 13 20.25V3.75c0-.966.784-1.75 1.75-1.75ZM3.5 3.75v16.5c0 .138.112.25.25.25h5.5a.25.25 0 0 0 .25-.25V3.75a.25.25 0 0 0-.25-.25h-5.5a.25.25 0 0 0-.25.25Zm11 0v16.5c0 .138.112.25.25.25h5.5a.25.25 0 0 0 .25-.25V3.75a.25.25 0 0 0-.25-.25h-5.5a.25.25 0 0 0-.25.25Z"
        /> < title > { title } < / title > < / svg >
    }
}
