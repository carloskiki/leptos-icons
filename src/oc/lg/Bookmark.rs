#[cfg(feature = "OcLgBookmark")]
use leptos::*;
#[cfg(feature = "OcLgBookmark")]
///This icon requires the feature `OcLgBookmark` to be enabled.
#[component]
pub fn Bookmark(
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
        "M5 3.75C5 2.784 5.784 2 6.75 2h10.5c.966 0 1.75.784 1.75 1.75v17.5a.75.75 0 0 1-1.218.586L12 17.21l-5.781 4.625A.75.75 0 0 1 5 21.25Zm1.75-.25a.25.25 0 0 0-.25.25v15.94l5.031-4.026a.749.749 0 0 1 .938 0L17.5 19.69V3.75a.25.25 0 0 0-.25-.25Z"
        /> < title > { title } < / title > < / svg >
    }
}
