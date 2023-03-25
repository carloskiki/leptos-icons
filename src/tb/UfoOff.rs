#[cfg(feature = "TbUfoOff")]
use leptos::*;
#[cfg(feature = "TbUfoOff")]
///This icon requires the feature `TbUfoOff` to be enabled.
#[component]
pub fn UfoOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-ufo-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = size.clone() height = size xmlns = "http://www.w3.org/2000/svg" >
        < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z"
        fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.95 9.01c3.02 .739 5.05 2.123 5.05 3.714c0 1.08 -.931 2.063 -2.468 2.814m-3 1c-1.36 .295 -2.9 .462 -4.531 .462c-5.52 0 -10 -1.909 -10 -4.276c0 -1.59 2.04 -2.985 5.07 -3.724"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.69 10.686c1.388 -.355 2.31 -.976 2.31 -1.686v-.035c0 -2.742 -2.239 -4.965 -5 -4.965c-1.125 0 -2.164 .37 -3 .992m-1.707 2.297a4.925 4.925 0 0 0 -.293 1.676v.035c0 .961 1.696 1.764 3.956 1.956"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M15 17l2 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8.5 17l-1.5 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 14h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 13h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 13h.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
