#[cfg(feature = "SiShenzhenmetro")]
use leptos::*;
#[cfg(feature = "SiShenzhenmetro")]
///This icon requires the feature `SiShenzhenmetro` to be enabled.
#[component]
pub fn Shenzhenmetro(
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
        "M.27 0v.155c0 4.69 3.033 8.751 7.331 10.434v2.736C3.303 14.99.271 19.019.271 23.768V24H4.36v-.232c0-2.459 1.278-4.623 3.24-5.934V24h3.165v-7.384c.408-.065.82-.098 1.234-.1.423 0 .834.038 1.235.1V24h3.165v-6.148c1.925 1.313 3.163 3.469 3.163 5.916V24h4.168v-.232c0-4.691-3.033-8.751-7.331-10.434V10.6c4.298-1.665 7.33-5.696 7.33-10.446V.001h-4.09v.154c0 2.458-1.277 4.622-3.24 5.934V0h-3.165v7.305c-.408.066-.821.1-1.235.103a8.11 8.11 0 0 1-1.234-.103V.001H7.6V6.07C5.675 4.757 4.438 2.602 4.438.154V.001zm10.495 11.358c.82.084 1.648.084 2.469.001v1.205a12.236 12.236 0 0 0-2.47 0z"
        /> < title > { title } < / title > < / svg >
    }
}
