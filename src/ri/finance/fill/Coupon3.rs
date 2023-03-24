#[cfg(feature = "RiFinanceFillCoupon3")]
use leptos::*;
#[cfg(feature = "RiFinanceFillCoupon3")]
///This icon requires the feature `RiFinanceFillCoupon3` to be enabled.
#[component]
pub fn Coupon3(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = { size.clone() }
        height = { size } > < g xmlns = "http://www.w3.org/2000/svg" >< path fill =
        "none" d = "M0 0h24v24H0z" />< path d =
        "M11 21a1.5 1.5 0 0 0-3 0H3a1 1 0 0 1-1-1V4a1 1 0 0 1 1-1h5a1.5 1.5 0 0 0 3 0h10a1 1 0 0 1 1 1v5.5a2.5 2.5 0 1 0 0 5V20a1 1 0 0 1-1 1H11zM9.5 10.5a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3zm0 6a1.5 1.5 0 1 0 0-3 1.5 1.5 0 0 0 0 3z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
