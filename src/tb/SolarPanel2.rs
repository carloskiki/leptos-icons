#[cfg(feature = "TbSolarPanel2")]
use leptos::*;
#[cfg(feature = "TbSolarPanel2")]
///This icon requires the feature `TbSolarPanel2` to be enabled.
#[component]
pub fn SolarPanel2(
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
        stroke_witdh = "0" style = style class =
        "icon icon-tabler icon-tabler-solar-panel-2" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 2a4 4 0 1 0 8 0" />< path xmlns = "http://www.w3.org/2000/svg" d = "M4 3h1"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M19 3h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 9v1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17.2 7.2l.707 .707" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M6.8 7.2l-.7 .7" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M4.28 21h15.44a1 1 0 0 0 .97 -1.243l-1.5 -6a1 1 0 0 0 -.97 -.757h-12.44a1 1 0 0 0 -.97 .757l-1.5 6a1 1 0 0 0 .97 1.243z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M4 17h16" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 13l-1 8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 13l1 8" /> < title > { title } < / title >
        < / svg >
    }
}
