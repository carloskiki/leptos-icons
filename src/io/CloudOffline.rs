#[cfg(feature = "IoCloudOffline")]
use leptos::*;
#[cfg(feature = "IoCloudOffline")]
///This icon requires the feature `IoCloudOffline` to be enabled.
#[component]
pub fn CloudOffline(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M448,464a15.92,15.92,0,0,1-11.31-4.69l-384-384A16,16,0,0,1,75.31,52.69l384,384A16,16,0,0,1,448,464Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M38.72,196.78C13.39,219.88,0,251.42,0,288c0,36,14.38,68.88,40.49,92.59C65.64,403.43,99.56,416,136,416H328.8a8,8,0,0,0,5.66-13.66L100.88,168.76a8,8,0,0,0-8-2C72,173.15,53.4,183.38,38.72,196.78Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M476.59,391.23C499.76,372.78,512,345.39,512,312c0-57.57-42-90.58-87.56-100.75a16,16,0,0,1-12.12-12.39c-7.68-36.68-24.45-68.15-49.18-92A153.57,153.57,0,0,0,256,64c-31.12,0-60.12,9-84.62,26.1a8,8,0,0,0-1.14,12.26L461.68,393.8a8,8,0,0,0,10.2.93Q474.31,393.05,476.59,391.23Z"
        /> < title > { title } < / title > < / svg >
    }
}
