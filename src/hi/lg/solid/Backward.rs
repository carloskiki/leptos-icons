#[cfg(feature = "HiLgSolidBackward")]
use leptos::*;
#[cfg(feature = "HiLgSolidBackward")]
///This icon requires the feature `HiLgSolidBackward` to be enabled.
#[component]
pub fn Backward(
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
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.19474 18.4391C10.4447 19.1534 12 18.2508 12 16.8112V14.4707L18.9447 18.4391C20.1947 19.1534 21.75 18.2508 21.75 16.8112L21.75 8.68832C21.75 7.24865 20.1947 6.34609 18.9447 7.06036L12 11.0288V8.68832C12 7.24865 10.4447 6.34609 9.19474 7.06036L2.08725 11.1218C0.827597 11.8416 0.827596 13.6579 2.08725 14.3777L9.19474 18.4391Z"
        fill = "#0F172A" /> < title > { title } < / title > < / svg >
    }
}
