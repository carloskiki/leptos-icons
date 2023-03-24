#[cfg(feature = "OcLgGift")]
use leptos::*;
#[cfg(feature = "OcLgGift")]
///This icon requires the feature `OcLgGift` to be enabled.
#[component]
pub fn Gift(
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
        "M3.75 3.75A3.75 3.75 0 0 1 7.5 0c1.455 0 3.436.901 4.5 3.11C13.064.901 15.044 0 16.5 0a3.75 3.75 0 0 1 3 6h1.75c.966 0 1.75.784 1.75 1.75v2.5c0 .698-.409 1.301-1 1.582v8.418A1.75 1.75 0 0 1 20.25 22H3.75A1.75 1.75 0 0 1 2 20.25v-8.418c-.591-.282-1-.884-1-1.582v-2.5C1 6.784 1.784 6 2.75 6H4.5a3.733 3.733 0 0 1-.75-2.25ZM20.5 12h-7.75v8.5h7.5a.25.25 0 0 0 .25-.25Zm-9.25 8.5V12H3.5v8.25c0 .138.112.25.25.25Zm10-10a.25.25 0 0 0 .25-.25v-2.5a.25.25 0 0 0-.25-.25h-8.5v3Zm-18.5 0h8.5v-3h-8.5a.25.25 0 0 0-.25.25v2.5c0 .138.112.25.25.25Zm16-6.75A2.25 2.25 0 0 0 16.5 1.5c-1.15 0-3.433 1.007-3.72 4.5h3.72a2.25 2.25 0 0 0 2.25-2.25ZM11.22 6c-.287-3.493-2.57-4.5-3.72-4.5a2.25 2.25 0 1 0 0 4.5Z"
        /> < title > { title } < / title > < / svg >
    }
}
