#[cfg(feature = "SiVox")]
use leptos::*;
#[cfg(feature = "SiVox")]
///This icon requires the feature `SiVox` to be enabled.
#[component]
pub fn Vox(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M0 8.198l4.182 7.604h2.442L8.15 13.07a4.276 4.276 0 0 1 2.054-4.872H7.112l-1.677 3.216-1.706-3.216zm14.342 0a4.24 4.24 0 0 1 1.923 2.206c.784 2.081-.098 4.415-2.145 5.398h2.767l1.564-1.754 1.42 1.754H24l-3.505-4.032 3.088-3.572H19.41l-.952 1.249-.931-1.249zm-2.09 1.596c-.949 0-1.913.69-2.074 1.775a2.132 2.132 0 0 0 2.064 2.483c1.268.01 2.192-1.126 2.156-2.18-.013-1.015-.877-2.08-2.146-2.078z"
        /> < title > { title } < / title > < / svg >
    }
}
