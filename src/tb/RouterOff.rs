#[cfg(feature = "TbRouterOff")]
use leptos::*;
#[cfg(feature = "TbRouterOff")]
///This icon requires the feature `TbRouterOff` to be enabled.
#[component]
pub fn RouterOff(
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
        "icon icon-tabler icon-tabler-router-off" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M17 13h2a2 2 0 0 1 2 2v2m-.588 3.417c-.362 .36 -.861 .583 -1.412 .583h-14a2 2 0 0 1 -2 -2v-4a2 2 0 0 1 2 -2h8"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M17 17v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 17v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12.226 8.2a4 4 0 0 1 6.024 .55" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M9.445 5.407a8 8 0 0 1 12.055 1.093" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < /
        title > < / svg >
    }
}
