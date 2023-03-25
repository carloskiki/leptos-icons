#[cfg(feature = "BiRegularBorderLeft")]
use leptos::*;
#[cfg(feature = "BiRegularBorderLeft")]
///This icon requires the feature `BiRegularBorderLeft` to be enabled.
#[component]
pub fn BorderLeft(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("1em"))]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
    #[prop(default = String::from("currentColor"))]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M11 3h2v2h-2zm4 0h2v2h-2zM7 3h2v2H7zm12 0h2v2h-2zm0 8h2v2h-2zm0 4h2v2h-2zm0-8h2v2h-2zm0 12h2v2h-2zm-4 0h2v2h-2zm-8 0h2v2H7zm4 0h2v2h-2zm0-4h2v2h-2zm0-8h2v2h-2zm4 4h2v2h-2zm-8 0h2v2H7zm4 0h2v2h-2zM3 5v16h2V3H3z"
        /> < title > { title } < / title > < / svg >
    }
}
