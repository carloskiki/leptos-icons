#[cfg(feature = "IoShieldHalf")]
use leptos::*;
#[cfg(feature = "IoShieldHalf")]
///This icon requires the feature `IoShieldHalf` to be enabled.
#[component]
pub fn ShieldHalf(
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
        stroke_witdh = "0" style = style id = "icons" viewBox = "0 0 512 512" width =
        size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M48.9,112.37C138.32,96.33,175.29,84.45,256,48c80.71,36.45,117.68,48.33,207.1,64.37C479.3,369.13,271.42,457.79,256,464,240.58,457.79,32.7,369.13,48.9,112.37Z"
        fill = "none" stroke = "#000" stroke - linecap = "round" stroke - linejoin =
        "round" stroke - width = "32" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M256,48c80.71,36.45,117.68,48.33,207.1,64.37C479.3,369.13,271.42,457.79,256,464Z"
        /> < title > { title } < / title > < / svg >
    }
}
