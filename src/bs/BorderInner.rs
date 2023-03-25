#[cfg(feature = "BsBorderInner")]
use leptos::*;
#[cfg(feature = "BsBorderInner")]
///This icon requires the feature `BsBorderInner` to be enabled.
#[component]
pub fn BorderInner(
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
        stroke_witdh = "0" style = style width = "16" height = "16" fill = "currentColor"
        class = "bi bi-border-inner" viewBox = "0 0 16 16" width = size.clone() height =
        size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M.969 0H0v.969h.5V1h.469V.969H1V.5H.969V0zm.937 1h.938V0h-.938v1zm1.875 0h.938V0H3.78v1zm1.875 0h.938V0h-.938v1z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.5 7.5H16v1H8.5V16h-1V8.5H0v-1h7.5V0h1v7.5z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M9.406 1h.938V0h-.938v1zm1.875 0h.938V0h-.938v1zm1.875 0h.938V0h-.938v1zm1.875 0h.469V.969h.5V0h-.969v.5H15v.469h.031V1zM1 2.844v-.938H0v.938h1zm14-.938v.938h1v-.938h-1zM1 4.719V3.78H0v.938h1zm14-.938v.938h1V3.78h-1zM1 6.594v-.938H0v.938h1zm14-.938v.938h1v-.938h-1zM0 9.406v.938h1v-.938H0zm16 .938v-.938h-1v.938h1zm-16 .937v.938h1v-.938H0zm16 .938v-.938h-1v.938h1zm-16 .937v.938h1v-.938H0zm16 .938v-.938h-1v.938h1zM0 16h.969v-.5H1v-.469H.969V15H.5v.031H0V16zm1.906 0h.938v-1h-.938v1zm1.875 0h.938v-1H3.78v1zm1.875 0h.938v-1h-.938v1zm3.75 0h.938v-1h-.938v1zm1.875 0h.938v-1h-.938v1zm1.875 0h.938v-1h-.938v1zm1.875-.5v.5H16v-.969h-.5V15h-.469v.031H15v.469h.031z"
        /> < title > { title } < / title > < / svg >
    }
}
