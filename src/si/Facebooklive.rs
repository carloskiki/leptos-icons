#[cfg(feature = "SiFacebooklive")]
use leptos::*;
#[cfg(feature = "SiFacebooklive")]
///This icon requires the feature `SiFacebooklive` to be enabled.
#[component]
pub fn Facebooklive(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" d
        =
        "M9.4777 7.8108a.4611.4611 0 0 0-.462.462v7.452a.461.461 0 0 0 .462.462H23.538v.0024a.4611.4611 0 0 0 .462-.462V8.2728a.4611.4611 0 0 0-.462-.462zm-5.2862.0072C1.879 7.818 0 9.6922 0 12.007c0 2.0922 1.533 3.8267 3.5376 4.1394V13.218h-1.071v-1.211h1.071v-.924c0-1.0497.6208-1.6326 1.578-1.6326.4573 0 .9336.0877.9336.0877v1.0236h-.5237c-.5213 0-.6871.327-.6871.6563v.7866h1.1634l-.1872 1.2108H4.836v2.9286c2.0093-.3104 3.5447-2.0448 3.5447-4.137 0-2.315-1.8766-4.1891-4.1892-4.1891zm7.1676 2.4073h.635v2.9926h1.6278v.5544H11.359zm2.9452 0h.635v3.547h-.635zm1.2439 0h.7014l.8932 2.8078h.0427l.8862-2.8078h.6752l-1.2273 3.547h-.7322zm3.81.0024h2.296v.5473h-1.6609v.9407h1.5709v.5165h-1.5709v.9928h1.661v.5497h-2.296Z"
        /> < title > { title } < / title > < / svg >
    }
}
