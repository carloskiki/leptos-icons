#[cfg(feature = "TbCircleArrowDownLeftFilled")]
use leptos::*;
#[cfg(feature = "TbCircleArrowDownLeftFilled")]
///This icon requires the feature `TbCircleArrowDownLeftFilled` to be enabled.
#[component]
pub fn CircleArrowDownLeftFilled(
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
        "icon icon-tabler icon-tabler-circle-arrow-down-left-filled" width = "24" height
        = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill =
        "none" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 3.34a10 10 0 1 1 -14.995 8.984l-.005 -.324l.005 -.324a10 10 0 0 1 14.995 -8.336zm-8 4.66a1 1 0 0 0 -1 1v6l.007 .117l.029 .149l.035 .105l.054 .113l.071 .111c.03 .04 .061 .077 .097 .112l.09 .08l.096 .067l.098 .052l.11 .044l.112 .03l.126 .017l6.075 .003l.117 -.007a1 1 0 0 0 .883 -.993l-.007 -.117a1 1 0 0 0 -.993 -.883h-3.586l4.293 -4.293l.083 -.094a1 1 0 0 0 -1.497 -1.32l-4.293 4.291v-3.584l-.007 -.117a1 1 0 0 0 -.993 -.883z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
