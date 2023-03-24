#[cfg(feature = "IoTrashBin")]
use leptos::*;
#[cfg(feature = "IoTrashBin")]
///This icon requires the feature `IoTrashBin` to be enabled.
#[component]
pub fn TrashBin(
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
        "0 0 512 512" width = { size.clone() } height = { size } > < rect xmlns =
        "http://www.w3.org/2000/svg" x = "32" y = "48" width = "448" height = "80" rx =
        "32" ry = "32" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M74.45,160a8,8,0,0,0-8,8.83L92.76,421.39a1.5,1.5,0,0,0,0,.22A48,48,0,0,0,140.45,464H371.54a48,48,0,0,0,47.67-42.39l0-.21,26.27-252.57a8,8,0,0,0-8-8.83ZM323.31,340.69a16,16,0,1,1-22.63,22.62L256,318.63l-44.69,44.68a16,16,0,0,1-22.63-22.62L233.37,296l-44.69-44.69a16,16,0,0,1,22.63-22.62L256,273.37l44.68-44.68a16,16,0,0,1,22.63,22.62L278.62,296Z"
        /> < title > { title } < / title > < / svg >
    }
}
