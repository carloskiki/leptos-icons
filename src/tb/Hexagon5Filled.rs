#[cfg(feature = "TbHexagon5Filled")]
use leptos::*;
#[cfg(feature = "TbHexagon5Filled")]
///This icon requires the feature `TbHexagon5Filled` to be enabled.
#[component]
pub fn Hexagon5Filled(
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
        "icon icon-tabler icon-tabler-hexagon-5-filled" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.543 2.426c.862 -.48 1.9 -.512 2.785 -.096l.187 .096l6.026 3.588l.095 .063l.084 .07l.113 .083a3 3 0 0 1 1.143 1.992l.019 .198l.005 .2v6.536c0 1.022 -.52 1.968 -1.326 2.492l-.165 .099l-6.053 3.864c-.845 .47 -1.86 .501 -2.772 .069l-.193 -.1l-5.947 -3.802a3 3 0 0 1 -1.537 -2.418l-.007 -.203v-6.537c0 -1.022 .52 -1.968 1.348 -2.505l6.195 -3.689zm3.457 4.574h-4a1 1 0 0 0 -.993 .883l-.007 .117v4a1 1 0 0 0 .883 .993l.117 .007h3v2h-2l-.007 -.117a1 1 0 0 0 -1.993 .117a2 2 0 0 0 1.85 1.995l.15 .005h2a2 2 0 0 0 1.995 -1.85l.005 -.15v-2a2 2 0 0 0 -1.85 -1.995l-.15 -.005h-2v-2h3a1 1 0 0 0 .993 -.883l.007 -.117a1 1 0 0 0 -.883 -.993l-.117 -.007z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
