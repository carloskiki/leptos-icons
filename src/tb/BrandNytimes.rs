#[cfg(feature = "TbBrandNytimes")]
use leptos::*;
#[cfg(feature = "TbBrandNytimes")]
///This icon requires the feature `TbBrandNytimes` to be enabled.
#[component]
pub fn BrandNytimes(
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
        "icon icon-tabler icon-tabler-brand-nytimes" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M11.036 5.058a8 8 0 1 0 8.706 9.965" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 21v-11l-7.5 4" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M17.5 3a2.5 2.5 0 1 1 0 5l-11 -5a2.5 2.5 0 0 0 -.67 4.91" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 12v8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 13h-.01" /> < title > { title } < / title >
        < / svg >
    }
}
