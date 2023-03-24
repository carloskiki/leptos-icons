#[cfg(feature = "IoCloudySharp")]
use leptos::*;
#[cfg(feature = "IoCloudySharp")]
///This icon requires the feature `IoCloudySharp` to be enabled.
#[component]
pub fn CloudySharp(
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
        "M376,432H116c-32.37,0-60.23-8.57-80.59-24.77C12.24,388.78,0,361.39,0,328c0-61.85,48.44-95.34,97.75-102.64,6.52-41.18,24-76.4,51.11-102.46A153.57,153.57,0,0,1,256,80c35.5,0,68.24,11.69,94.68,33.8a156.42,156.42,0,0,1,45.22,63.61c30.26,4.81,57.45,17.18,77.38,35.36C498.61,235.88,512,267.42,512,304c0,36-14.38,68.88-40.49,92.59C446.36,419.43,412.44,432,376,432Z"
        /> < title > { title } < / title > < / svg >
    }
}
