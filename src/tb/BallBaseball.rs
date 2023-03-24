#[cfg(feature = "TbBallBaseball")]
use leptos::*;
#[cfg(feature = "TbBallBaseball")]
///This icon requires the feature `TbBallBaseball` to be enabled.
#[component]
pub fn BallBaseball(
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
        "icon icon-tabler icon-tabler-ball-baseball" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M5.636 18.364a9 9 0 1 0 12.728 -12.728a9 9 0 0 0 -12.728 12.728z" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M12.495 3.02a9 9 0 0 1 -9.475 9.475" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M20.98 11.505a9 9 0 0 0 -9.475 9.475"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M9 9l2 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 13l2 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 7l2 1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M7 11l1 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 11l1 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M11 16l2 1" /> < title > { title } < / title >
        < / svg >
    }
}
