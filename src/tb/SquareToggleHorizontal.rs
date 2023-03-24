#[cfg(feature = "TbSquareToggleHorizontal")]
use leptos::*;
#[cfg(feature = "TbSquareToggleHorizontal")]
///This icon requires the feature `TbSquareToggleHorizontal` to be enabled.
#[component]
pub fn SquareToggleHorizontal(
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
        "icon icon-tabler icon-tabler-square-toggle-horizontal" width = "24" height =
        "24" viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill =
        "none" stroke - linecap = "round" stroke - linejoin = "round" width = { size
        .clone() } height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke
        = "none" d = "M0 0h24v24H0z" fill = "none" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M22 12h-20" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M4 14v-8a2 2 0 0 1 2 -2h12a2 2 0 0 1 2 2v8" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M18 20a2 2 0 0 0 2 -2" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M4 18a2 2 0 0 0 2 2" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M14 20l-4 0" /> < title > { title } < / title >
        < / svg >
    }
}
