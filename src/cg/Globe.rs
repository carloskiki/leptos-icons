#[cfg(feature = "CgGlobe")]
use leptos::*;
#[cfg(feature = "CgGlobe")]
///This icon requires the feature `CgGlobe` to be enabled.
#[component]
pub fn Globe(
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
        "M6.85327 8C6.85327 5.23858 9.09185 3 11.8533 3C14.6147 3 16.8533 5.23858 16.8533 8C16.8533 10.7614 14.6147 13 11.8533 13C9.09185 13 6.85327 10.7614 6.85327 8ZM11.8533 11C10.1964 11 8.85327 9.65685 8.85327 8C8.85327 6.34315 10.1964 5 11.8533 5C13.5101 5 14.8533 6.34315 14.8533 8C14.8533 9.65685 13.5101 11 11.8533 11Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 12.1294C6.25216 14.2031 8.4189 15.6624 10.9414 15.9486V18H8.85327V20H14.8533V18H12.9414V15.9266C16.8449 15.3958 19.8532 12.0492 19.8532 8.00001C19.8532 6.43638 19.4046 4.97752 18.6291 3.74512L16.9253 4.79326C17.513 5.72084 17.8532 6.82069 17.8532 8.00001C17.8532 11.3137 15.167 14 11.8532 14C9.66611 14 7.75231 12.8298 6.70381 11.0813L5 12.1294Z"
        fill = "currentColor" /> < title > { title } < / title > < / svg >
    }
}
