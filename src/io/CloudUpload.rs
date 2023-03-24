#[cfg(feature = "IoCloudUpload")]
use leptos::*;
#[cfg(feature = "IoCloudUpload")]
///This icon requires the feature `IoCloudUpload` to be enabled.
#[component]
pub fn CloudUpload(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M473.66,210c-14-10.38-31.2-18-49.36-22.11a16.11,16.11,0,0,1-12.19-12.22c-7.8-34.75-24.59-64.55-49.27-87.13C334.15,62.25,296.21,47.79,256,47.79c-35.35,0-68,11.08-94.37,32.05a150.07,150.07,0,0,0-42.06,53,16,16,0,0,1-11.31,8.87c-26.75,5.4-50.9,16.87-69.34,33.12C13.46,197.33,0,227.24,0,261.39c0,34.52,14.49,66,40.79,88.76,25.12,21.69,58.94,33.64,95.21,33.64H240V230.42l-36.69,36.69a16,16,0,0,1-23.16-.56c-5.8-6.37-5.24-16.3.85-22.39l63.69-63.68a16,16,0,0,1,22.62,0L331,244.14c6.28,6.29,6.64,16.6.39,22.91a16,16,0,0,1-22.68.06L272,230.42V383.79H396c31.34,0,59.91-8.8,80.45-24.77,23.26-18.1,35.55-44,35.55-74.83C512,254.25,498.74,228.58,473.66,210Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M240,448.21a16,16,0,1,0,32,0V383.79H240Z" /> < title > { title } < / title > < /
        svg >
    }
}
