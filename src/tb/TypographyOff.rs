#[cfg(feature = "TbTypographyOff")]
use leptos::*;
#[cfg(feature = "TbTypographyOff")]
///This icon requires the feature `TbTypographyOff` to be enabled.
#[component]
pub fn TypographyOff(
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
        "icon icon-tabler icon-tabler-typography-off" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M4 20h3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 20h6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6.9 15h6.9" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 13l3 7" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5 20l4.09 -10.906" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10.181 6.183l.819 -2.183h2l3.904 8.924" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title }
        < / title > < / svg >
    }
}
