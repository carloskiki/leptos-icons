#[cfg(feature = "TbBrandDingtalk")]
use leptos::*;
#[cfg(feature = "TbBrandDingtalk")]
///This icon requires the feature `TbBrandDingtalk` to be enabled.
#[component]
pub fn BrandDingtalk(
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
        "icon icon-tabler icon-tabler-brand-dingtalk" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 12a9 9 0 1 1 -18 0a9 9 0 0 1 18 0z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M8 7.5l7.02 2.632a1 1 0 0 1 .567 1.33l-1.087 2.538h1.5l-5 4l1 -4c-3.1 .03 -3.114 -3.139 -4 -6.5z"
        /> < title > { title } < / title > < / svg >
    }
}
