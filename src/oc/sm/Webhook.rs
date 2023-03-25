#[cfg(feature = "OcSmWebhook")]
use leptos::*;
#[cfg(feature = "OcSmWebhook")]
///This icon requires the feature `OcSmWebhook` to be enabled.
#[component]
pub fn Webhook(
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
        stroke_witdh = "0" style = style width = "16" height = "16" viewBox = "0 0 16 16"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M5.5 4.25a2.25 2.25 0 0 1 4.5 0 .75.75 0 0 0 1.5 0 3.75 3.75 0 1 0-6.14 2.889l-2.272 4.258a.75.75 0 0 0 1.324.706L7 7.25a.75.75 0 0 0-.309-1.015A2.25 2.25 0 0 1 5.5 4.25Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.364 3.607a.75.75 0 0 1 1.03.257l2.608 4.349a3.75 3.75 0 1 1-.628 6.785.75.75 0 0 1 .752-1.299 2.25 2.25 0 1 0-.033-3.88.75.75 0 0 1-1.03-.256L7.107 4.636a.75.75 0 0 1 .257-1.03Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2.9 8.776A.75.75 0 0 1 2.625 9.8 2.25 2.25 0 1 0 6 11.75a.75.75 0 0 1 .75-.751h5.5a.75.75 0 0 1 0 1.5H7.425a3.751 3.751 0 1 1-5.55-3.998.75.75 0 0 1 1.024.274Z"
        /> < title > { title } < / title > < / svg >
    }
}
