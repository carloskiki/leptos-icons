#[cfg(feature = "RiEditorInsertColumnLeft")]
use leptos::*;
#[cfg(feature = "RiEditorInsertColumnLeft")]
///This icon requires the feature `RiEditorInsertColumnLeft` to be enabled.
#[component]
pub fn InsertColumnLeft(
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
        "none" d = "M0 0H24V24H0z" />< path d =
        "M20 3c.552 0 1 .448 1 1v16c0 .552-.448 1-1 1h-6c-.552 0-1-.448-1-1V4c0-.552.448-1 1-1h6zm-1 2h-4v14h4V5zM6 7c2.761 0 5 2.239 5 5s-2.239 5-5 5-5-2.239-5-5 2.239-5 5-5zm1 2H5v1.999L3 11v2l2-.001V15h2v-2.001L9 13v-2l-2-.001V9z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
