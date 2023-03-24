#[cfg(feature = "OcSmShare")]
use leptos::*;
#[cfg(feature = "OcSmShare")]
///This icon requires the feature `OcSmShare` to be enabled.
#[component]
pub fn Share(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M3.75 6.5a.25.25 0 0 0-.25.25v6.5c0 .138.112.25.25.25h8.5a.25.25 0 0 0 .25-.25v-6.5a.25.25 0 0 0-.25-.25h-1a.75.75 0 0 1 0-1.5h1c.966 0 1.75.784 1.75 1.75v6.5A1.75 1.75 0 0 1 12.25 15h-8.5A1.75 1.75 0 0 1 2 13.25v-6.5C2 5.784 2.784 5 3.75 5h1a.75.75 0 0 1 0 1.5ZM7.823.177a.25.25 0 0 1 .354 0l2.896 2.896a.25.25 0 0 1-.177.427H8.75v5.75a.75.75 0 0 1-1.5 0V3.5H5.104a.25.25 0 0 1-.177-.427Z"
        /> < title > { title } < / title > < / svg >
    }
}
