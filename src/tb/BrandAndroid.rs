#[cfg(feature = "TbBrandAndroid")]
use leptos::*;
#[cfg(feature = "TbBrandAndroid")]
///This icon requires the feature `TbBrandAndroid` to be enabled.
#[component]
pub fn BrandAndroid(
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
        "icon icon-tabler icon-tabler-brand-android" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M4 10l0 6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 10l0 6" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M7 9h10v8a1 1 0 0 1 -1 1h-8a1 1 0 0 1 -1 -1v-8a5 5 0 0 1 10 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 3l1 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 3l-1 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 18l0 3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 18l0 3" /> < title > { title } < / title >
        < / svg >
    }
}
