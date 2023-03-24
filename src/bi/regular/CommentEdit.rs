#[cfg(feature = "BiRegularCommentEdit")]
use leptos::*;
#[cfg(feature = "BiRegularCommentEdit")]
///This icon requires the feature `BiRegularCommentEdit` to be enabled.
#[component]
pub fn CommentEdit(
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
        "m13.771 9.123-1.399-1.398-3.869 3.864v1.398h1.398zM14.098 6l1.398 1.398-1.067 1.067-1.398-1.398z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M20 2H4c-1.103 0-2 .897-2 2v18l5.333-4H20c1.103 0 2-.897 2-2V4c0-1.103-.897-2-2-2zm0 14H6.667L4 18V4h16v12z"
        /> < title > { title } < / title > < / svg >
    }
}
