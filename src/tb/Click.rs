#[cfg(feature = "TbClick")]
use leptos::*;
#[cfg(feature = "TbClick")]
///This icon requires the feature `TbClick` to be enabled.
#[component]
pub fn Click(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-click"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 12l3 0" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M12 3l0 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7.8 7.8l-2.2 -2.2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16.2 7.8l2.2 -2.2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7.8 16.2l-2.2 2.2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 12l9 3l-4 2l-2 4l-3 -9" /> < title > {
        title } < / title > < / svg >
    }
}
