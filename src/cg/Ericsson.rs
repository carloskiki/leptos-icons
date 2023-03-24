#[cfg(feature = "CgEricsson")]
use leptos::*;
#[cfg(feature = "CgEricsson")]
///This icon requires the feature `CgEricsson` to be enabled.
#[component]
pub fn Ericsson(
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
        "M7.71689 5.72315C6.71581 6.18996 6.2827 7.37992 6.74951 8.381C7.21632 9.38208 8.40628 9.81519 9.40736 9.34838L20.2831 4.27696C21.2841 3.81015 21.7172 2.62019 21.2504 1.61911C20.7836 0.618028 19.5937 0.184918 18.5926 0.651729L7.71689 5.72315Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4.74951 15.381C4.2827 14.3799 4.71581 13.19 5.71689 12.7231L16.5926 7.65173C17.5937 7.18492 18.7836 7.61803 19.2504 8.61911C19.7172 9.62019 19.2841 10.8101 18.2831 11.277L7.40736 16.3484C6.40628 16.8152 5.21632 16.3821 4.74951 15.381Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2.74951 22.381C2.2827 21.3799 2.71581 20.19 3.71689 19.7231L14.5926 14.6517C15.5937 14.1849 16.7836 14.618 17.2504 15.6191C17.7172 16.6202 17.2841 17.8101 16.2831 18.277L5.40736 23.3484C4.40628 23.8152 3.21632 23.3821 2.74951 22.381Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
