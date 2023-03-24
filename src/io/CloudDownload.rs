#[cfg(feature = "IoCloudDownload")]
use leptos::*;
#[cfg(feature = "IoCloudDownload")]
///This icon requires the feature `IoCloudDownload` to be enabled.
#[component]
pub fn CloudDownload(
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
        "M472.7,189.5c-13.26-8.43-29.83-14.56-48.08-17.93A16,16,0,0,1,412,159.28c-7.86-34.51-24.6-64.13-49.15-86.58C334.15,46.45,296.21,32,256,32c-35.35,0-68,11.08-94.37,32a150.13,150.13,0,0,0-41.95,52.83A16.05,16.05,0,0,1,108,125.8c-27.13,4.9-50.53,14.68-68.41,28.7C13.7,174.83,0,203.56,0,237.6,0,305,55.93,352,136,352H240V224.45c0-8.61,6.62-16,15.23-16.43A16,16,0,0,1,272,224V352H396c72.64,0,116-34.24,116-91.6C512,230.35,498.41,205.83,472.7,189.5Z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M240,425.42l-36.7-36.64a16,16,0,0,0-22.6,22.65l64,63.89a16,16,0,0,0,22.6,0l64-63.89a16,16,0,0,0-22.6-22.65L272,425.42V352H240Z"
        /> < title > { title } < / title > < / svg >
    }
}
