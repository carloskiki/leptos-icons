#[cfg(feature = "BiRegularVerticalTop")]
use leptos::*;
#[cfg(feature = "BiRegularVerticalTop")]
///This icon requires the feature `BiRegularVerticalTop` to be enabled.
#[component]
pub fn VerticalTop(
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
        "m12 7-4 4h3v8h2v-8h3l-4-4zM3 3h3v2H3zM8 3h3v2H8zM13 3h3v2h-3zM18 3h3v2h-3z" /> <
        title > { title } < / title > < / svg >
    }
}
