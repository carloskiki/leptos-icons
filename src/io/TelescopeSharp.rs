#[cfg(feature = "IoTelescopeSharp")]
use leptos::*;
#[cfg(feature = "IoTelescopeSharp")]
///This icon requires the feature `IoTelescopeSharp` to be enabled.
#[component]
pub fn TelescopeSharp(
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style viewBox = "0 0 512 512" width = { size.clone() }
        height = { size } > < polygon xmlns = "http://www.w3.org/2000/svg" points =
        "1.41 292.9 46.23 369.87 144.37 313.49 99.64 236.12 1.41 292.9" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M287.87,252.34l23.67-13.81-63.42-110-132.92,77C133.75,237.51,176,310,176,310l15.53-8.32c.24-.13.55,0,.83,0L102.65,496h35.16l99.05-214.25,23.24-13.4L358,464h36Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M262.08,96c24.81,42.23,60.25,104.25,86.4,148.76L510.79,151,424.07,1.41Z" /> <
        title > { title } < / title > < / svg >
    }
}
