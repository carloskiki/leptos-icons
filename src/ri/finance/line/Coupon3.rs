#[cfg(feature = "RiFinanceLineCoupon3")]
use leptos::*;
#[cfg(feature = "RiFinanceLineCoupon3")]
///This icon requires the feature `RiFinanceLineCoupon3` to be enabled.
#[component]
pub fn Coupon3(
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
        stroke_witdh = "0" style = style viewBox = "0 0 24 24" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < g xmlns =
        "http://www.w3.org/2000/svg" >< path fill = "none" d = "M0 0h24v24H0z" />< path d
        =
        "M2 4a1 1 0 0 1 1-1h18a1 1 0 0 1 1 1v5.5a2.5 2.5 0 1 0 0 5V20a1 1 0 0 1-1 1H3a1 1 0 0 1-1-1V4zm6.085 15a1.5 1.5 0 0 1 2.83 0H20v-2.968a4.5 4.5 0 0 1 0-8.064V5h-9.085a1.5 1.5 0 0 1-2.83 0H4v14h4.085zM9.5 11a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3zm0 5a1.5 1.5 0 1 1 0-3 1.5 1.5 0 0 1 0 3z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
