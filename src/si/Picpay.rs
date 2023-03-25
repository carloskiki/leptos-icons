#[cfg(feature = "SiPicpay")]
use leptos::*;
#[cfg(feature = "SiPicpay")]
///This icon requires the feature `SiPicpay` to be enabled.
#[component]
pub fn Picpay(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M16.463 1.587v7.537H24V1.587zm1.256 1.256h5.025v5.025h-5.025zm1.256 1.256v2.513h2.513V4.099zM3.77 5.355V8.53h3.376c2.142 0 3.358 1.04 3.358 2.939 0 1.947-1.216 3.011-3.358 3.011H3.769V8.53H0v13.884h3.769v-4.76h3.57c4.333 0 6.815-2.352 6.815-6.32 0-3.771-2.482-5.978-6.814-5.978Z"
        /> < title > { title } < / title > < / svg >
    }
}
