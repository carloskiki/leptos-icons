#[cfg(feature = "SiHugo")]
use leptos::*;
#[cfg(feature = "SiHugo")]
///This icon requires the feature `SiHugo` to be enabled.
#[component]
pub fn Hugo(
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
        "M11.754 0a3.998 3.998 0 00-2.049.596L3.33 4.532a4.252 4.252 0 00-2.017 3.615v8.03c0 1.473.79 2.838 2.067 3.574l6.486 3.733a3.88 3.88 0 003.835.018l7.043-3.966a3.817 3.817 0 001.943-3.323V7.752a3.57 3.57 0 00-1.774-3.084L13.817.541a3.998 3.998 0 00-2.063-.54zm.022 1.674c.413-.006.828.1 1.2.315l7.095 4.127c.584.34.941.96.94 1.635v8.462c0 .774-.414 1.484-1.089 1.864l-7.042 3.966a2.199 2.199 0 01-2.179-.01l-6.485-3.734a2.447 2.447 0 01-1.228-2.123v-8.03c0-.893.461-1.72 1.221-2.19l6.376-3.935a2.323 2.323 0 011.19-.347zm-4.7 3.844V18.37h2.69v-5.62h4.46v5.62h2.696V5.518h-2.696v4.681h-4.46V5.518Z"
        /> < title > { title } < / title > < / svg >
    }
}
