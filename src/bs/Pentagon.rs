#[cfg(feature = "BsPentagon")]
use leptos::*;
#[cfg(feature = "BsPentagon")]
///This icon requires the feature `BsPentagon` to be enabled.
#[component]
pub fn Pentagon(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-pentagon" viewBox = "0 0 16 16" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M7.685 1.545a.5.5 0 0 1 .63 0l6.263 5.088a.5.5 0 0 1 .161.539l-2.362 7.479a.5.5 0 0 1-.476.349H4.099a.5.5 0 0 1-.476-.35L1.26 7.173a.5.5 0 0 1 .161-.54l6.263-5.087Zm8.213 5.28a.5.5 0 0 0-.162-.54L8.316.257a.5.5 0 0 0-.631 0L.264 6.286a.5.5 0 0 0-.162.538l2.788 8.827a.5.5 0 0 0 .476.349h9.268a.5.5 0 0 0 .476-.35l2.788-8.826Z"
        /> < title > { title } < / title > < / svg >
    }
}
