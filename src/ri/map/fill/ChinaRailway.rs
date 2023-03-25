#[cfg(feature = "RiMapFillChinaRailway")]
use leptos::*;
#[cfg(feature = "RiMapFillChinaRailway")]
///This icon requires the feature `RiMapFillChinaRailway` to be enabled.
#[component]
pub fn ChinaRailway(
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
        "M11 19v-6l-2-1V9h6v3l-2 1v6l5 1v2H6v-2l5-1zM10 2.223V1h4v1.223a9.003 9.003 0 0 1 2.993 16.266l-1.11-1.664a7 7 0 1 0-7.767 0l-1.109 1.664A9.003 9.003 0 0 1 10 2.223z"
        /></ g > < title > { title } < / title > < / svg >
    }
}
