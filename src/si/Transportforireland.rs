#[cfg(feature = "SiTransportforireland")]
use leptos::*;
#[cfg(feature = "SiTransportforireland")]
///This icon requires the feature `SiTransportforireland` to be enabled.
#[component]
pub fn Transportforireland(
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
        "M0 0v12c0 6.62 5.38 12 12 12h12V11.978h-.022c0-6.62-5.38-11.978-12-11.978zm3.376 8.145h6.337v1.546h-2.33v6.12H5.706v-6.12h-2.33zm8.014 0h5.837V9.67h-4.138v1.633h3.659v1.546h-3.659v2.962H11.39zm7.535 0h1.678v7.666h-1.678Z"
        /> < title > { title } < / title > < / svg >
    }
}
