#[cfg(feature = "VsLink")]
use leptos::*;
#[cfg(feature = "VsLink")]
///This icon requires the feature `VsLink` to be enabled.
#[component]
pub fn Link(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        fill = "currentColor" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M4.4 3h3.085a3.4 3.4 0 0 1 3.4 3.4v.205A3.4 3.4 0 0 1 7.485 10H7V9h.485A2.4 2.4 0 0 0 9.88 6.61V6.4A2.4 2.4 0 0 0 7.49 4H4.4A2.4 2.4 0 0 0 2 6.4v.205A2.394 2.394 0 0 0 4 8.96v1a3.4 3.4 0 0 1-3-3.35V6.4A3.405 3.405 0 0 1 4.4 3zM12 7.04v-1a3.4 3.4 0 0 1 3 3.36v.205A3.405 3.405 0 0 1 11.605 13h-3.09A3.4 3.4 0 0 1 5.12 9.61V9.4A3.4 3.4 0 0 1 8.515 6H9v1h-.485A2.4 2.4 0 0 0 6.12 9.4v.205A2.4 2.4 0 0 0 8.515 12h3.09A2.4 2.4 0 0 0 14 9.61V9.4a2.394 2.394 0 0 0-2-2.36z"
        /> < title > { title } < / title > < / svg >
    }
}
