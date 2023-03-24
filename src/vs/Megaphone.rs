#[cfg(feature = "VsMegaphone")]
use leptos::*;
#[cfg(feature = "VsMegaphone")]
///This icon requires the feature `VsMegaphone` to be enabled.
#[component]
pub fn Megaphone(
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
        "M2 6.77l12.33-3.43.67.53v8.6l-.67.53-6.089-1.595a2.16 2.16 0 1 1-4.178-1.095L2 9.77l-.42-.53V7.3L2 6.77zm3.006 3.787a1.13 1.13 0 0 0-.04.242 1.17 1.17 0 0 0 2.288.347l-2.248-.589zM2.58 8.82L14 11.83V4.5L2.58 7.72v1.1z"
        /> < title > { title } < / title > < / svg >
    }
}
