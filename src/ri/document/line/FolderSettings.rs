#[cfg(feature = "RiDocumentLineFolderSettings")]
use leptos::*;
#[cfg(feature = "RiDocumentLineFolderSettings")]
///This icon requires the feature `RiDocumentLineFolderSettings` to be enabled.
#[component]
pub fn FolderSettings(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path d =
        "M12.414 5H21a1 1 0 0 1 1 1v14a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h7.414l2 2zM4 5v14h16V7h-8.414l-2-2H4zm4.591 8.809a3.508 3.508 0 0 1 0-1.622l-.991-.572 1-1.732.991.573a3.495 3.495 0 0 1 1.404-.812V8.5h2v1.144c.532.159 1.01.44 1.403.812l.992-.573 1 1.731-.991.573a3.508 3.508 0 0 1 0 1.622l.991.572-1 1.731-.991-.572a3.495 3.495 0 0 1-1.404.811v1.145h-2V16.35a3.495 3.495 0 0 1-1.404-.811l-.991.572-1-1.73.991-.573zm3.404.688a1.5 1.5 0 1 0 0-2.998 1.5 1.5 0 0 0 0 2.998z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
