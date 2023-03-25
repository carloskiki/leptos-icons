#[cfg(feature = "TbToolsOff")]
use leptos::*;
#[cfg(feature = "TbToolsOff")]
///This icon requires the feature `TbToolsOff` to be enabled.
#[component]
pub fn ToolsOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-tools-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16 12l4 -4a2.828 2.828 0 1 0 -4 -4l-4 4m-2 2l-7 7v4h4l7 -7" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14.5 5.5l4 4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 8l-5 -5m-2 2l-2 2l5 5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 8l-1.5 1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 12l5 5m-2 2l-2 2l-5 -5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 17l-1.5 1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
