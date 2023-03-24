#[cfg(feature = "OcLgScreenFull")]
use leptos::*;
#[cfg(feature = "OcLgScreenFull")]
///This icon requires the feature `OcLgScreenFull` to be enabled.
#[component]
pub fn ScreenFull(
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
        "M3.75 15a.75.75 0 0 1 .75.75v3.5c0 .138.112.25.25.25h3.5a.75.75 0 0 1 0 1.5h-3.5A1.75 1.75 0 0 1 3 19.25v-3.5a.75.75 0 0 1 .75-.75Zm16.5 0a.75.75 0 0 1 .75.75v3.5A1.75 1.75 0 0 1 19.25 21h-3.5a.75.75 0 0 1 0-1.5h3.5a.25.25 0 0 0 .25-.25v-3.5a.75.75 0 0 1 .75-.75ZM4.75 4.5a.25.25 0 0 0-.25.25v3.5a.75.75 0 0 1-1.5 0v-3.5C3 3.784 3.784 3 4.75 3h3.5a.75.75 0 0 1 0 1.5ZM15 3.75a.75.75 0 0 1 .75-.75h3.5c.966 0 1.75.784 1.75 1.75v3.5a.75.75 0 0 1-1.5 0v-3.5a.25.25 0 0 0-.25-.25h-3.5a.75.75 0 0 1-.75-.75Z"
        /> < title > { title } < / title > < / svg >
    }
}
