#[cfg(feature = "TbSunHigh")]
use leptos::*;
#[cfg(feature = "TbSunHigh")]
///This icon requires the feature `TbSunHigh` to be enabled.
#[component]
pub fn SunHigh(
    cx: Scope,
    /// The size of the icon (The side length of the square surrounding the icon).
    /// Defaults to "1em".
    #[prop(into)]
    #[prop(optional)]
    size: String,
    /// HTML class attribute.
    #[prop(into)]
    #[prop(optional)]
    class: String,
    /// Color of the icon.
    /// For twotone icons, the secondary color has an opacity (alpha value) of 0.4.
    #[prop(into)]
    #[prop(optional)]
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
    view! {
        cx, < svg class = class stroke = "currentColor" fill = "currentColor"
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-sun-high"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14.828 14.828a4 4 0 1 0 -5.656 -5.656a4 4 0 0 0 5.656 5.656z" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6.343 17.657l-1.414 1.414" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6.343 6.343l-1.414 -1.414" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17.657 6.343l1.414 -1.414" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17.657 17.657l1.414 1.414" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 12h-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 4v-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 12h2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 20v2" /> < title > { title } < / title > <
        / svg >
    }
}
