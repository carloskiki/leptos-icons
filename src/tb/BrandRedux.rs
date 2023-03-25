#[cfg(feature = "TbBrandRedux")]
use leptos::*;
#[cfg(feature = "TbBrandRedux")]
///This icon requires the feature `TbBrandRedux` to be enabled.
#[component]
pub fn BrandRedux(
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
        "icon icon-tabler icon-tabler-brand-redux" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = size.clone() height = size
        xmlns = "http://www.w3.org/2000/svg" > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16.54 7c-.805 -2.365 -2.536 -4 -4.54 -4c-2.774 0 -5.023 2.632 -5.023 6.496c0 1.956 1.582 4.727 2.512 6"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4.711 11.979c-1.656 1.877 -2.214 4.185 -1.211 5.911c1.387 2.39 5.138 2.831 8.501 .9c1.703 -.979 2.875 -3.362 3.516 -4.798"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15.014 19.99c2.511 0 4.523 -.438 5.487 -2.1c1.387 -2.39 -.215 -5.893 -3.579 -7.824c-1.702 -.979 -4.357 -1.235 -5.927 -1.07"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10.493 9.862c.48 .276 1.095 .112 1.372 -.366a1 1 0 0 0 -.367 -1.365a1.007 1.007 0 0 0 -1.373 .366a1 1 0 0 0 .368 1.365z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.5 15.5m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15.5 14m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" />
        < title > { title } < / title > < / svg >
    }
}
