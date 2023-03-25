#[cfg(feature = "RiLogosFillWechatPay")]
use leptos::*;
#[cfg(feature = "RiLogosFillWechatPay")]
///This icon requires the feature `RiLogosFillWechatPay` to be enabled.
#[component]
pub fn WechatPay(
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
        "M9.27 14.669a.662.662 0 0 1-.88-.269l-.043-.095-1.818-3.998a.473.473 0 0 1 0-.145.327.327 0 0 1 .335-.328.305.305 0 0 1 .196.066l2.18 1.527a.989.989 0 0 0 .546.167.894.894 0 0 0 .342-.066l10.047-4.5a10.73 10.73 0 0 0-8.171-3.526C6.478 3.502 2 7.232 2 11.87a7.83 7.83 0 0 0 3.46 6.296.662.662 0 0 1 .24.727l-.45 1.701a.945.945 0 0 0-.051.24.327.327 0 0 0 .334.334.414.414 0 0 0 .19-.058l2.18-1.265c.16-.098.343-.151.531-.152.099 0 .197.014.29.043 1.063.3 2.161.452 3.265.45 5.525 0 10.01-3.729 10.01-8.33a7.226 7.226 0 0 0-1.097-3.883L9.35 14.625l-.08.044z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
