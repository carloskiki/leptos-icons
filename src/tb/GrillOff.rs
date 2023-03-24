#[cfg(feature = "TbGrillOff")]
use leptos::*;
#[cfg(feature = "TbGrillOff")]
///This icon requires the feature `TbGrillOff` to be enabled.
#[component]
pub fn GrillOff(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-grill-off"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 8h-3a6 6 0 0 0 6 6h2c.315 0 .624 -.024 .926 -.071m2.786 -1.214a5.99 5.99 0 0 0 2.284 -4.49l0 -.225h-7"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18.827 18.815a2 2 0 1 1 -2.663 -2.633" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 14l-3 6" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 18h-8" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 5v-1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 5v-1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 5v-1" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > { title } < / title >
        < / svg >
    }
}
