#[cfg(feature = "TbCircleDotted")]
use leptos::*;
#[cfg(feature = "TbCircleDotted")]
///This icon requires the feature `TbCircleDotted` to be enabled.
#[component]
pub fn CircleDotted(
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
        "icon icon-tabler icon-tabler-circle-dotted" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M7.5 4.21l0 .01" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M4.21 7.5l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 12l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4.21 16.5l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7.5 19.79l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 21l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16.5 19.79l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19.79 16.5l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M21 12l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M19.79 7.5l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16.5 4.21l0 .01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 3l0 .01" /> < title > { title } < / title >
        < / svg >
    }
}
