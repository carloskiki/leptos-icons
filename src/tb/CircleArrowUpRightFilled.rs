#[cfg(feature = "TbCircleArrowUpRightFilled")]
use leptos::*;
#[cfg(feature = "TbCircleArrowUpRightFilled")]
///This icon requires the feature `TbCircleArrowUpRightFilled` to be enabled.
#[component]
pub fn CircleArrowUpRightFilled(
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
        "icon icon-tabler icon-tabler-circle-arrow-up-right-filled" width = "24" height =
        "24" viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill =
        "none" stroke - linecap = "round" stroke - linejoin = "round" width = size
        .clone() height = size xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 3.34a10 10 0 1 1 -14.995 8.984l-.005 -.324l.005 -.324a10 10 0 0 1 14.995 -8.336zm-2 4.66h-6l-.117 .007a1 1 0 0 0 -.883 .993l.007 .117a1 1 0 0 0 .993 .883h3.584l-4.291 4.293l-.083 .094a1 1 0 0 0 1.497 1.32l4.293 -4.293v3.586l.007 .117a1 1 0 0 0 1.993 -.117v-6l-.007 -.117l-.029 -.149l-.035 -.105l-.054 -.113l-.071 -.111a1.01 1.01 0 0 0 -.097 -.112l-.09 -.08l-.096 -.067l-.098 -.052l-.11 -.044l-.112 -.03l-.126 -.017l-.075 -.003z"
        stroke - width = "0" fill = "currentColor" /> < title > { title } < / title > < /
        svg >
    }
}
