#[cfg(feature = "TbSnowman")]
use leptos::*;
#[cfg(feature = "TbSnowman")]
///This icon requires the feature `TbSnowman` to be enabled.
#[component]
pub fn Snowman(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-snowman"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 3a4 4 0 0 1 2.906 6.75a6 6 0 1 1 -5.81 0a4 4 0 0 1 2.904 -6.75z" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M17.5 11.5l2.5 -1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6.5 11.5l-2.5 -1.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 13h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 16h.01" /> < title > { title } < / title >
        < / svg >
    }
}
