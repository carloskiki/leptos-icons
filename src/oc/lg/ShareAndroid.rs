#[cfg(feature = "OcLgShareAndroid")]
use leptos::*;
#[cfg(feature = "OcLgShareAndroid")]
///This icon requires the feature `OcLgShareAndroid` to be enabled.
#[component]
pub fn ShareAndroid(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M20 5.5a3.498 3.498 0 0 1-6.062 2.385l-5.112 3.021a3.498 3.498 0 0 1 0 2.188l5.112 3.021a3.5 3.5 0 1 1-.764 1.29l-5.112-3.02a3.499 3.499 0 1 1-3.843-5.642 3.499 3.499 0 0 1 3.843.872l5.112-3.021A3.5 3.5 0 1 1 20 5.5Zm-1.5 13a2 2 0 1 0-3.999-.001 2 2 0 0 0 3.999.001Zm0-13a2 2 0 1 0-3.999-.001A2 2 0 0 0 18.5 5.5ZM5.5 14a2 2 0 1 0 .001-3.999A2 2 0 0 0 5.5 14Z"
        /> < title > { title } < / title > < / svg >
    }
}
