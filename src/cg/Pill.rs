#[cfg(feature = "CgPill")]
use leptos::*;
#[cfg(feature = "CgPill")]
///This icon requires the feature `CgPill` to be enabled.
#[component]
pub fn Pill(
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
        fill = "none" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" fill -
        rule = "evenodd" clip - rule = "evenodd" d =
        "M12.6569 2.75736C15 0.414213 18.799 0.414214 21.1421 2.75736C23.4853 5.1005 23.4853 8.8995 21.1421 11.2426L11.2426 21.1421C8.89949 23.4853 5.1005 23.4853 2.75736 21.1421C0.414214 18.799 0.414213 15 2.75736 12.6569L12.6569 2.75736ZM19.7279 9.82843L15.4853 14.0711L9.82843 8.41421L14.0711 4.17157C15.6332 2.60948 18.1658 2.60948 19.7279 4.17157C21.29 5.73367 21.29 8.26633 19.7279 9.82843Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
