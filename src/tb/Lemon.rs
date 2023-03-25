#[cfg(feature = "TbLemon")]
use leptos::*;
#[cfg(feature = "TbLemon")]
///This icon requires the feature `TbLemon` to be enabled.
#[component]
pub fn Lemon(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-lemon"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17.536 3.393c3.905 3.906 3.905 10.237 0 14.143c-3.906 3.905 -10.237 3.905 -14.143 0l14.143 -14.143"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.868 15.06a6.5 6.5 0 0 0 9.193 -9.192" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10.464 10.464l4.597 4.597" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10.464 10.464v6.364" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10.464 10.464h6.364" /> < title > { title } <
        / title > < / svg >
    }
}
