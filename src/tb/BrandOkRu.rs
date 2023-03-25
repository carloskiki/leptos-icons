#[cfg(feature = "TbBrandOkRu")]
use leptos::*;
#[cfg(feature = "TbBrandOkRu")]
///This icon requires the feature `TbBrandOkRu` to be enabled.
#[component]
pub fn BrandOkRu(
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
        "icon icon-tabler icon-tabler-brand-ok-ru" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M12 9m-2 0a2 2 0 1 0 4 0a2 2 0 1 0 -4 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M20 12c0 8 0 8 -8 8s-8 0 -8 -8s0 -8 8 -8s8 0 8 8z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9.5 13c1.333 .667 3.667 .667 5 0" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M9.5 17l2.5 -3l2.5 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 13.5v.5" /> < title > { title } < / title >
        < / svg >
    }
}
