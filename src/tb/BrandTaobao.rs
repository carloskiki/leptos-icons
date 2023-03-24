#[cfg(feature = "TbBrandTaobao")]
use leptos::*;
#[cfg(feature = "TbBrandTaobao")]
///This icon requires the feature `TbBrandTaobao` to be enabled.
#[component]
pub fn BrandTaobao(
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
        "icon icon-tabler icon-tabler-brand-taobao" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M2 5c.968 .555 1.335 1.104 2 2" />< path xmlns = "http://www.w3.org/2000/svg" d
        = "M2 10c5.007 3.674 2.85 6.544 0 10" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 4c-.137 4.137 -2.258 5.286 -3.709 6.684"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 6c2.194 -.8 3.736 -.852 6.056 -.993c4.206 -.158 5.523 2.264 5.803 5.153c.428 4.396 -.077 7.186 -2.117 9.298c-1.188 1.23 -3.238 2.62 -7.207 .259"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M11 10h6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 10v6.493" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 13h10" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 15.512l.853 1.72" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16.5 17c-1.145 .361 -7 3 -8.5 -.5" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M11.765 8.539l-1.765 2.461" /> < title
        > { title } < / title > < / svg >
    }
}
