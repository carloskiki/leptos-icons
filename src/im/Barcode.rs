#[cfg(feature = "ImBarcode")]
use leptos::*;
#[cfg(feature = "ImBarcode")]
///This icon requires the feature `ImBarcode` to be enabled.
#[component]
pub fn Barcode(
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
        stroke_witdh = "0" style = style version = "1.1" width = "16" height = "16"
        viewBox = "0 0 16 16" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" xmlns : xlink = "http://www.w3.org/1999/xlink" fill
        = "#000000" d =
        "M0 2h2v10h-2zM3 2h1v10h-1zM5 2h1v10h-1zM8 2h1v10h-1zM12 2h1v10h-1zM15 2h1v10h-1zM10 2h0.5v10h-0.5zM7 2h0.5v10h-0.5zM13.5 2h0.5v10h-0.5zM0 13h1v1h-1zM3 13h1v1h-1zM5 13h1v1h-1zM10 13h1v1h-1zM15 13h1v1h-1zM12 13h2v1h-2zM7 13h2v1h-2z"
        /> < title > { title } < / title > < / svg >
    }
}
