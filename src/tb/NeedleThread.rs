#[cfg(feature = "TbNeedleThread")]
use leptos::*;
#[cfg(feature = "TbNeedleThread")]
///This icon requires the feature `TbNeedleThread` to be enabled.
#[component]
pub fn NeedleThread(
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
        "icon icon-tabler icon-tabler-needle-thread" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 21c-.667 -.667 3.262 -6.236 11.785 -16.709a3.5 3.5 0 1 1 5.078 4.791c-10.575 8.612 -16.196 12.585 -16.863 11.918z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M17.5 6.5l-1 1" />< path xmlns
        = "http://www.w3.org/2000/svg" d =
        "M17 7c-2.333 -2.667 -3.5 -4 -5 -4s-2 1 -2 2c0 4 8.161 8.406 6 11c-1.056 1.268 -3.363 1.285 -5.75 .808"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.739 15.425c-1.393 -.565 -3.739 -1.925 -3.739 -3.425" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19.5 9.5l1.5 1.5" /> < title > { title } < /
        title > < / svg >
    }
}
