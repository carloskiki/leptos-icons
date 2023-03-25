#[cfg(feature = "BiLogosMagento")]
use leptos::*;
#[cfg(feature = "BiLogosMagento")]
///This icon requires the feature `BiLogosMagento` to be enabled.
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
        stroke_witdh = "0" style = style width = "24" height = "24" viewBox = "0 0 24 24"
        width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path
        xmlns = "http://www.w3.org/2000/svg" d =
        "m12 22.019-3.717-2.146V9.863l2.479-1.43v10.01l1.238.753 1.238-.753V8.434l2.479 1.43v10.01L12 22.019zm8.666-15.014v10.009l-2.475 1.43V8.434L12 4.861 5.807 8.434v10.01l-2.473-1.43V7.005L12 2l8.666 5.005z"
        /> < title > { title } < / title > < / svg >
    }
}
