#[cfg(feature = "SiMagento")]
use leptos::*;
#[cfg(feature = "SiMagento")]
///This icon requires the feature `SiMagento` to be enabled.
#[component]
pub fn Magento(
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
        stroke_witdh = "0" style = style role = "img" viewBox = "0 0 24 24" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 24l-4.455-2.572v-12l2.97-1.715v12.001l1.485.902 1.485-.902V7.713l2.971 1.715v12L12 24zM22.391 6v12l-2.969 1.714V7.713L12 3.43 4.574 7.713v12.001L1.609 18V6L12 0l10.391 6z"
        /> < title > { title } < / title > < / svg >
    }
}
