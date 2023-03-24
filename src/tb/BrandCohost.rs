#[cfg(feature = "TbBrandCohost")]
use leptos::*;
#[cfg(feature = "TbBrandCohost")]
///This icon requires the feature `TbBrandCohost` to be enabled.
#[component]
pub fn BrandCohost(
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
        "icon icon-tabler icon-tabler-brand-cohost" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 14m-3 0a3 2 0 1 0 6 0a3 2 0 1 0 -6 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M4.526 17.666c-1.133 -.772 -1.897 -1.924 -2.291 -3.456c-.398 -1.54 -.29 -2.937 .32 -4.19c.61 -1.255 1.59 -2.34 2.938 -3.254c1.348 -.914 2.93 -1.625 4.749 -2.132c1.81 -.504 3.516 -.708 5.12 -.61c1.608 .1 2.979 .537 4.112 1.31s1.897 1.924 2.291 3.456c.398 1.541 .29 2.938 -.32 4.192c-.61 1.253 -1.59 2.337 -2.938 3.252c-1.348 .915 -2.93 1.626 -4.749 2.133c-1.81 .503 -3.516 .707 -5.12 .61c-1.608 -.102 -2.979 -.538 -4.112 -1.31z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11 12.508c-.53 -.316 -1.23 -.508 -2 -.508c-1.657 0 -3 .895 -3 2s1.343 2 3 2c.767 0 1.467 -.192 2 -.508"
        /> < title > { title } < / title > < / svg >
    }
}
