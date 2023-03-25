#[cfg(feature = "SiNec")]
use leptos::*;
#[cfg(feature = "SiNec")]
///This icon requires the feature `SiNec` to be enabled.
#[component]
pub fn Nec(
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
        "M21.496 8.8a13.892 13.892 0 0 0-2.453.198c-.6.11-1.165.223-1.693.57-.807.526-1.076 1.49-1.086 2.394-.012.98.283 2.06 1.21 2.54.735.384 1.537.552 2.355.624 1.397.118 2.792.1 4.171-.09v-.98c-1.04.19-2.107.31-3.184.152-1.652-.24-1.93-1.473-1.93-2.286 0-.451.06-.987.482-1.526.676-.865 2.667-.956 4.55-.59V8.95a22.777 22.777 0 0 0-2.422-.15zm-12.812.197v4.446c.03.744.295 1.518 1.834 1.581 1.814.075 3.54.045 5.334-.024v-.896c-.89.014-3.487.001-3.65 0-.704-.042-.973-.256-.998-.837a32.12 32.12 0 0 1-.02-1.055h4.177v-.834h-4.176V9.851h4.582v-.854zM0 8.997v5.999h1.146v-4.254l3.383 3.764c.41.404.763.478 1.122.49h2.015V8.997h-1.15v4.397l-3.39-3.757c-.232-.227-.753-.64-1.194-.64z"
        /> < title > { title } < / title > < / svg >
    }
}
