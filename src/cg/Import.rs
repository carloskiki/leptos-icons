#[cfg(feature = "CgImport")]
use leptos::*;
#[cfg(feature = "CgImport")]
///This icon requires the feature `CgImport` to be enabled.
#[component]
pub fn Import(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 9.98193V19.9819H19V9.98193H15V7.98193H21V21.9819H3V7.98193H9V9.98193H5Z" fill
        = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M13.0001 2H11.0001V14.0531L8.46451 11.5175L7.05029 12.9317L12 17.8815L16.9498 12.9317L15.5356 11.5175L13.0001 14.053V2Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
