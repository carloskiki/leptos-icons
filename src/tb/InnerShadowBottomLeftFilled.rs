#[cfg(feature = "TbInnerShadowBottomLeftFilled")]
use leptos::*;
#[cfg(feature = "TbInnerShadowBottomLeftFilled")]
///This icon requires the feature `TbInnerShadowBottomLeftFilled` to be enabled.
#[component]
pub fn InnerShadowBottomLeftFilled(
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
        "icon icon-tabler icon-tabler-inner-shadow-bottom-left-filled" width = "24"
        height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor"
        fill = "none" stroke - linecap = "round" stroke - linejoin = "round" width = {
        size.clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg"
        stroke = "none" d = "M0 0h24v24H0z" fill = "none" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 2c5.523 0 10 4.477 10 10s-4.477 10 -10 10s-10 -4.477 -10 -10s4.477 -10 10 -10zm-6 9a1 1 0 0 0 -1 1a7 7 0 0 0 7 7a1 1 0 0 0 0 -2a5 5 0 0 1 -5 -5a1 1 0 0 0 -1 -1z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
