#[cfg(feature = "IoLogoStencil")]
use leptos::*;
#[cfg(feature = "IoLogoStencil")]
///This icon requires the feature `IoLogoStencil` to be enabled.
#[component]
pub fn LogoStencil(
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
        stroke_witdh = "0" style = style width = "512" height = "512" viewBox =
        "0 0 512 512" width = size.clone() height = size xmlns =
        "http://www.w3.org/2000/svg" > < path xmlns = "http://www.w3.org/2000/svg" d =
        "M188.8,334.07H386.13L279.47,448H83.2Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M512,199H106.61L0,313H405.39Z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M232.2,64H428.8L322.62,177.93H125.87Z" /> <
        title > { title } < / title > < / svg >
    }
}
