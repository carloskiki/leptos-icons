#[cfg(feature = "IoScanCircle")]
use leptos::*;
#[cfg(feature = "IoScanCircle")]
///This icon requires the feature `IoScanCircle` to be enabled.
#[component]
pub fn ScanCircle(
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
        "M256,48C141.31,48,48,141.31,48,256s93.31,208,208,208,208-93.31,208-208S370.69,48,256,48ZM216,368H188a44.05,44.05,0,0,1-44-44V296a16,16,0,0,1,32,0v28a12,12,0,0,0,12,12h28a16,16,0,0,1,0,32Zm0-192H188a12,12,0,0,0-12,12v28a16,16,0,0,1-32,0V188a44.05,44.05,0,0,1,44-44h28a16,16,0,0,1,0,32ZM368,324a44.05,44.05,0,0,1-44,44H296a16,16,0,0,1,0-32h28a12,12,0,0,0,12-12V296a16,16,0,0,1,32,0Zm0-108a16,16,0,0,1-32,0V188a12,12,0,0,0-12-12H296a16,16,0,0,1,0-32h28a44.05,44.05,0,0,1,44,44Z"
        /> < title > { title } < / title > < / svg >
    }
}
