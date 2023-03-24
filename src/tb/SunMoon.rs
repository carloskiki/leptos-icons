#[cfg(feature = "TbSunMoon")]
use leptos::*;
#[cfg(feature = "TbSunMoon")]
///This icon requires the feature `TbSunMoon` to be enabled.
#[component]
pub fn SunMoon(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-sun-moon"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M9.173 14.83a4 4 0 1 1 5.657 -5.657" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M11.294 12.707l.174 .247a7.5 7.5 0 0 0 8.845 2.492a9 9 0 0 1 -14.671 2.914" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M3 12h1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 3v1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M5.6 5.6l.7 .7" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 21l18 -18" /> < title > { title } < / title
        > < / svg >
    }
}
