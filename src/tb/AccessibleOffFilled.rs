#[cfg(feature = "TbAccessibleOffFilled")]
use leptos::*;
#[cfg(feature = "TbAccessibleOffFilled")]
///This icon requires the feature `TbAccessibleOffFilled` to be enabled.
#[component]
pub fn AccessibleOffFilled(
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
    let style = format!("{} color: {};", style, color);
    let size = if size == "" { "1em" } else { &size };
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-accessible-off-filled" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 3.34a10 10 0 1 1 -14.995 8.984l-.005 -.324l.005 -.324a10 10 0 0 1 14.995 -8.336zm-1.051 6.844a1 1 0 0 0 -1.152 -.663l-.113 .03l-2.684 .895l-2.684 -.895l-.113 -.03a1 1 0 0 0 -.628 1.884l.109 .044l2.316 .771v.976l-1.832 2.75l-.06 .1a1 1 0 0 0 .237 1.21l.1 .076l.101 .06a1 1 0 0 0 1.21 -.237l.076 -.1l1.168 -1.752l1.168 1.752l.07 .093a1 1 0 0 0 1.653 -1.102l-.059 -.1l-1.832 -2.75v-.977l2.316 -.771l.109 -.044a1 1 0 0 0 .524 -1.221zm-3.949 -4.184a1.5 1.5 0 1 0 0 3a1.5 1.5 0 0 0 0 -3z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
