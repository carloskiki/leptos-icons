#[cfg(feature = "TbBellRinging2Filled")]
use leptos::*;
#[cfg(feature = "TbBellRinging2Filled")]
///This icon requires the feature `TbBellRinging2Filled` to be enabled.
#[component]
pub fn BellRinging2Filled(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-bell-ringing-2-filled" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.63 17.531c.612 .611 .211 1.658 -.652 1.706a3.992 3.992 0 0 1 -3.05 -1.166a3.992 3.992 0 0 1 -1.165 -3.049c.046 -.826 1.005 -1.228 1.624 -.726l.082 .074l3.161 3.16z"
        stroke - width = "0" fill = "currentColor" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M20.071 3.929c.96 .96 1.134 2.41 .52 3.547l-.09 .153l-.024 .036a8.013 8.013 0 0 1 -1.446 7.137l-.183 .223l-.191 .218l-2.073 2.072l-.08 .112a3 3 0 0 0 -.499 2.113l.035 .201l.045 .185c.264 .952 -.853 1.645 -1.585 1.051l-.086 -.078l-11.313 -11.313c-.727 -.727 -.017 -1.945 .973 -1.671a3 3 0 0 0 2.5 -.418l.116 -.086l2.101 -2.1a8 8 0 0 1 7.265 -1.86l.278 .071l.037 -.023a3.003 3.003 0 0 1 3.432 .192l.14 .117l.128 .12z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
