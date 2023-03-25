#[cfg(feature = "BiRegularReceipt")]
use leptos::*;
#[cfg(feature = "BiRegularReceipt")]
///This icon requires the feature `BiRegularReceipt` to be enabled.
#[component]
pub fn Receipt(
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
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M21 11h-3V4a1 1 0 0 0-1-1H3a1 1 0 0 0-1 1v14c0 1.654 1.346 3 3 3h14c1.654 0 3-1.346 3-3v-6a1 1 0 0 0-1-1zM5 19a1 1 0 0 1-1-1V5h12v13c0 .351.061.688.171 1H5zm15-1a1 1 0 0 1-2 0v-5h2v5z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M6 7h8v2H6zm0 4h8v2H6zm5 4h3v2h-3z" /> < title > { title } < / title > < / svg >
    }
}
