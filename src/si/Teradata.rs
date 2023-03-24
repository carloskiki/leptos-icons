#[cfg(feature = "SiTeradata")]
use leptos::*;
#[cfg(feature = "SiTeradata")]
///This icon requires the feature `SiTeradata` to be enabled.
#[component]
pub fn Teradata(
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
        "M12 0C5 0 0 5.65 0 12.08C0 18.83 5 24 12 24S24 18.83 24 12.08C24 5.65 19 0 12 0M8.47 3.44H11.97V6.7H15.55V9.56H11.97V14.78C11.97 16.36 12.74 17.05 13.9 17.05C14.32 17.05 14.88 16.93 15.41 16.73C15.79 17.73 16.46 18.63 17.18 19.35A7 7 0 0 1 13.66 20.32C10.54 20.32 8.47 18.67 8.47 15.04V3.45Z"
        /> < title > { title } < / title > < / svg >
    }
}
