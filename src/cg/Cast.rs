#[cfg(feature = "CgCast")]
use leptos::*;
#[cfg(feature = "CgCast")]
///This icon requires the feature `CgCast` to be enabled.
#[component]
pub fn Cast(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        fill = "none" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M20 6H4V8H2V6C2 4.89543 2.89543 4 4 4H20C21.1046 4 22 4.89543 22 6V18C22 19.1046 21.1046 20 20 20H15V18H20V6Z"
        fill = "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2 13C5.86599 13 9 16.134 9 20H7C7 17.2386 4.76142 15 2 15V13Z" fill =
        "currentColor" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2 17C3.65685 17 5 18.3431 5 20H2V17Z" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M2 9C8.07513 9 13 13.9249 13 20H11C11 15.0294 6.97056 11 2 11V9Z" fill =
        "currentColor" /> < title > { title } < / title > < / svg >
    }
}
