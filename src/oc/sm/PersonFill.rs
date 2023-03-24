#[cfg(feature = "OcSmPersonFill")]
use leptos::*;
#[cfg(feature = "OcSmPersonFill")]
///This icon requires the feature `OcSmPersonFill` to be enabled.
#[component]
pub fn PersonFill(
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
        "M4.243 4.757a3.757 3.757 0 1 1 5.851 3.119 6.006 6.006 0 0 1 3.9 5.339.75.75 0 0 1-.715.784H2.721a.75.75 0 0 1-.714-.784 6.006 6.006 0 0 1 3.9-5.34 3.753 3.753 0 0 1-1.664-3.118Z"
        /> < title > { title } < / title > < / svg >
    }
}
