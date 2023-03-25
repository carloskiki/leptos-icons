#[cfg(feature = "BiRegularBitcoin")]
use leptos::*;
#[cfg(feature = "BiRegularBitcoin")]
///This icon requires the feature `BiRegularBitcoin` to be enabled.
#[component]
pub fn Bitcoin(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M8 13v4H6v2h3v2h2v-2h2v2h2v-2.051c1.968-.249 3.5-1.915 3.5-3.949 0-1.32-.65-2.484-1.64-3.213A3.982 3.982 0 0 0 18 9c0-1.858-1.279-3.411-3-3.858V3h-2v2h-2V3H9v2H6v2h2v6zm6.5 4H10v-4h4.5c1.103 0 2 .897 2 2s-.897 2-2 2zM10 7h4c1.103 0 2 .897 2 2s-.897 2-2 2h-4V7z"
        /> < title > { title } < / title > < / svg >
    }
}
