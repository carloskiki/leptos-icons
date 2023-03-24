#[cfg(feature = "TbMessageCircle2Filled")]
use leptos::*;
#[cfg(feature = "TbMessageCircle2Filled")]
///This icon requires the feature `TbMessageCircle2Filled` to be enabled.
#[component]
pub fn MessageCircle2Filled(
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
        "icon icon-tabler icon-tabler-message-circle-2-filled" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.821 4.91c3.898 -2.765 9.469 -2.539 13.073 .536c3.667 3.127 4.168 8.238 1.152 11.897c-2.842 3.447 -7.965 4.583 -12.231 2.805l-.232 -.101l-4.375 .931l-.075 .013l-.11 .009l-.113 -.004l-.044 -.005l-.11 -.02l-.105 -.034l-.1 -.044l-.076 -.042l-.108 -.077l-.081 -.074l-.073 -.083l-.053 -.075l-.065 -.115l-.042 -.106l-.031 -.113l-.013 -.075l-.009 -.11l.004 -.113l.005 -.044l.02 -.11l.022 -.072l1.15 -3.451l-.022 -.036c-2.21 -3.747 -1.209 -8.392 2.411 -11.118l.23 -.168z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
