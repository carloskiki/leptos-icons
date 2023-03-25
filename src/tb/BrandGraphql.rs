#[cfg(feature = "TbBrandGraphql")]
use leptos::*;
#[cfg(feature = "TbBrandGraphql")]
///This icon requires the feature `TbBrandGraphql` to be enabled.
#[component]
pub fn BrandGraphql(
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
        "icon icon-tabler icon-tabler-brand-graphql" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M5.308 7.265l5.385 -3.029" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M13.308 4.235l5.384 3.03" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M20 9.5v5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18.693 16.736l-5.385 3.029" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10.692 19.765l-5.384 -3.03" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 14.5v-5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12.772 4.786l6.121 10.202" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18.5 16h-13" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5.107 14.988l6.122 -10.201" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 3.5m-1.5 0a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 20.5m-1.5 0a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M4 8m-1.5 0a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M4 16m-1.5 0a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M20 16m-1.5 0a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M20 8m-1.5 0a1.5 1.5 0 1 0 3 0a1.5 1.5 0 1 0 -3 0" /> < title > { title } < /
        title > < / svg >
    }
}
