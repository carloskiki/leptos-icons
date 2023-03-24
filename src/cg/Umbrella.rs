#[cfg(feature = "CgUmbrella")]
use leptos::*;
#[cfg(feature = "CgUmbrella")]
///This icon requires the feature `CgUmbrella` to be enabled.
#[component]
pub fn Umbrella(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M4 9C4 4.58172 7.58172 1 12 1C16.4183 1 20 4.58172 20 9V11H13.0187V20.5C13.0187 21.8807 11.8994 23 10.5187 23C9.13797 23 8.01868 21.8807 8.01868 20.5V17.8571H10.0187V20.5C10.0187 20.7761 10.2425 21 10.5187 21C10.7948 21 11.0187 20.7761 11.0187 20.5V11H4V9ZM12 3C15.3137 3 18 5.68629 18 9H6C6 5.68629 8.68629 3 12 3Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
