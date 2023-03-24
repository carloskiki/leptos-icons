#[cfg(feature = "BiLogosMarkdown")]
use leptos::*;
#[cfg(feature = "BiLogosMarkdown")]
///This icon requires the feature `BiLogosMarkdown` to be enabled.
#[component]
pub fn Markdown(
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
        "M20.553 18.15H3.447a1.443 1.443 0 0 1-1.442-1.441V7.291c0-.795.647-1.441 1.442-1.441h17.105c.795 0 1.442.646 1.442 1.441v9.418a1.441 1.441 0 0 1-1.441 1.441zM6.811 15.268V11.52l1.922 2.402 1.922-2.402v3.748h1.922V8.732h-1.922l-1.922 2.403-1.922-2.403H4.889v6.535h1.922zM19.688 12h-1.922V8.732h-1.923V12h-1.922l2.884 3.364L19.688 12z"
        /> < title > { title } < / title > < / svg >
    }
}
