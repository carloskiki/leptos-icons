#[cfg(feature = "TbRating12Plus")]
use leptos::*;
#[cfg(feature = "TbRating12Plus")]
///This icon requires the feature `TbRating12Plus` to be enabled.
#[component]
pub fn Rating12Plus(
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
        "icon icon-tabler icon-tabler-rating-12-plus" width = "24" height = "24" viewBox
        = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 12m-9 0a9 9 0 1 0 18 0a9 9 0 1 0 -18 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 15v-6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15.5 12h3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 10.5v3" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M10 10.5a1.5 1.5 0 0 1 3 0c0 .443 -.313 .989 -.612 1.393l-2.388 3.107h3" /> <
        title > { title } < / title > < / svg >
    }
}
