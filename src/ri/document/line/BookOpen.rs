#[cfg(feature = "RiDocumentLineBookOpen")]
use leptos::*;
#[cfg(feature = "RiDocumentLineBookOpen")]
///This icon requires the feature `RiDocumentLineBookOpen` to be enabled.
#[component]
pub fn BookOpen(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M13 21v2h-2v-2H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h6a3.99 3.99 0 0 1 3 1.354A3.99 3.99 0 0 1 15 3h6a1 1 0 0 1 1 1v16a1 1 0 0 1-1 1h-8zm7-2V5h-5a2 2 0 0 0-2 2v12h7zm-9 0V7a2 2 0 0 0-2-2H4v14h7z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
