#[cfg(feature = "BiRegularBarcodeReader")]
use leptos::*;
#[cfg(feature = "BiRegularBarcodeReader")]
///This icon requires the feature `BiRegularBarcodeReader` to be enabled.
#[component]
pub fn BarcodeReader(
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M5 7h2v10H5zm9 0h1v10h-1zm-4 0h3v10h-3zM8 7h1v10H8zm8 0h3v10h-3z" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M4 5h4V3H4c-1.103 0-2 .897-2 2v4h2V5zm0 16h4v-2H4v-4H2v4c0 1.103.897 2 2 2zM20 3h-4v2h4v4h2V5c0-1.103-.897-2-2-2zm0 16h-4v2h4c1.103 0 2-.897 2-2v-4h-2v4z"
        /> < title > { title } < / title > < / svg >
    }
}
