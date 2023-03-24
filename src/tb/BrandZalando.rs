#[cfg(feature = "TbBrandZalando")]
use leptos::*;
#[cfg(feature = "TbBrandZalando")]
///This icon requires the feature `TbBrandZalando` to be enabled.
#[component]
pub fn BrandZalando(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-brand-zalando" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M7.531 21c-.65 0 -1 -.15 -1.196 -.27c-.266 -.157 -.753 -.563 -1.197 -1.747a20.583 20.583 0 0 1 -1.137 -6.983c.015 -2.745 .436 -5.07 1.137 -6.975c.444 -1.2 .93 -1.605 1.197 -1.763c.192 -.103 .545 -.262 1.195 -.262c.244 0 .532 .022 .871 .075a19.093 19.093 0 0 1 6.425 2.475h.007a19.572 19.572 0 0 1 5.287 4.508c.783 .99 .879 1.627 .879 1.942c0 .315 -.096 .953 -.879 1.943a19.571 19.571 0 0 1 -5.287 4.5h-.007a19.041 19.041 0 0 1 -6.425 2.474a5.01 5.01 0 0 1 -.871 .083z"
        /> < title > { title } < / title > < / svg >
    }
}
