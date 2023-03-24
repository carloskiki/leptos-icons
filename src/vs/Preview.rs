#[cfg(feature = "VsPreview")]
use leptos::*;
#[cfg(feature = "VsPreview")]
///This icon requires the feature `VsPreview` to be enabled.
#[component]
pub fn Preview(
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
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M2 2h12l1 1v10l-1 1H2l-1-1V3l1-1zm0 11h12V3H2v10zm11-9H3v3h10V4zm-1 2H4V5h8v1zm-3 6h4V8H9v4zm1-3h2v2h-2V9zM7 8H3v1h4V8zm-4 3h4v1H3v-1z"
        /> < title > { title } < / title > < / svg >
    }
}
