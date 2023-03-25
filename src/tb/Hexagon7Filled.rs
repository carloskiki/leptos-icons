#[cfg(feature = "TbHexagon7Filled")]
use leptos::*;
#[cfg(feature = "TbHexagon7Filled")]
///This icon requires the feature `TbHexagon7Filled` to be enabled.
#[component]
pub fn Hexagon7Filled(
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
        "icon icon-tabler icon-tabler-hexagon-7-filled" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = size.clone()
        height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.54 2.401a3.062 3.062 0 0 1 2.79 -.096l.188 .096l6.043 3.599l.095 .063l.085 .07l.113 .084a3.005 3.005 0 0 1 1.146 1.993l.019 .199l.005 .2v6.555c0 1.024 -.52 1.971 -1.328 2.497l-.165 .099l-6.07 3.875a3.016 3.016 0 0 1 -2.778 .069l-.194 -.1l-5.965 -3.813a3.006 3.006 0 0 1 -1.54 -2.422l-.006 -.204v-6.555a3 3 0 0 1 1.35 -2.51l.17 -.102l6.042 -3.598zm3.46 4.599h-4l-.117 .007a1 1 0 0 0 -.876 .876l-.007 .117l.007 .117a1 1 0 0 0 .876 .876l.117 .007h2.718l-1.688 6.757l-.022 .115a1 1 0 0 0 1.927 .482l.035 -.111l2 -8l.021 -.112a1 1 0 0 0 -.878 -1.125l-.113 -.006z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
