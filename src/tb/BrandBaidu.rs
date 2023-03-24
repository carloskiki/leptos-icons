#[cfg(feature = "TbBrandBaidu")]
use leptos::*;
#[cfg(feature = "TbBrandBaidu")]
///This icon requires the feature `TbBrandBaidu` to be enabled.
#[component]
pub fn BrandBaidu(
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
        "icon icon-tabler icon-tabler-brand-baidu" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5 9.5m-1 0a1 1.5 0 1 0 2 0a1 1.5 0 1 0 -2 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M14.463 11.596c1.282 1.774 3.476 3.416 3.476 3.416s1.921 1.574 .593 3.636c-1.328 2.063 -4.892 1.152 -4.892 1.152s-1.416 -.44 -3.06 -.088c-1.644 .356 -3.06 .22 -3.06 .22s-2.055 -.22 -2.47 -2.304c-.416 -2.084 1.918 -3.638 2.102 -3.858c.182 -.222 1.409 -.966 2.284 -2.394c.875 -1.428 3.337 -2.287 5.027 .221z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9 4.5m-1 0a1 1.5 0 1 0 2 0a1 1.5 0 1 0 -2 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 4.5m-1 0a1 1.5 0 1 0 2 0a1 1.5 0 1 0 -2 0"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M19 9.5m-1 0a1 1.5 0 1 0 2 0a1 1.5 0 1 0 -2 0" /> < title > { title } < / title
        > < / svg >
    }
}
