#[cfg(feature = "CgTag")]
use leptos::*;
#[cfg(feature = "CgTag")]
///This icon requires the feature `CgTag` to be enabled.
#[component]
pub fn Tag(
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
        "M2 8V16C2 16.5523 2.44772 17 3 17H16.6202C16.9121 17 17.1895 16.8724 17.3795 16.6508L20.808 12.6508C21.129 12.2763 21.129 11.7237 20.808 11.3492L17.3795 7.34921C17.1895 7.12756 16.9121 7 16.6202 7H3C2.44772 7 2 7.44772 2 8ZM0 8V16C0 17.6569 1.34315 19 3 19H16.6202C17.496 19 18.328 18.6173 18.898 17.9524L22.3265 13.9524C23.2895 12.8289 23.2895 11.1711 22.3265 10.0476L18.898 6.04763C18.328 5.38269 17.496 5 16.6202 5H3C1.34315 5 0 6.34315 0 8Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" fill - rule =
        "evenodd" clip - rule = "evenodd" d =
        "M15 13C15.5523 13 16 12.5523 16 12C16 11.4477 15.5523 11 15 11C14.4477 11 14 11.4477 14 12C14 12.5523 14.4477 13 15 13ZM15 15C16.6569 15 18 13.6569 18 12C18 10.3431 16.6569 9 15 9C13.3431 9 12 10.3431 12 12C12 13.6569 13.3431 15 15 15Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
