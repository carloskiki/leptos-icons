#[cfg(feature = "TbTrain")]
use leptos::*;
#[cfg(feature = "TbTrain")]
///This icon requires the feature `TbTrain` to be enabled.
#[component]
pub fn Train(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-train"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 13c0 -3.87 -3.37 -7 -10 -7h-8" />< path xmlns = "http://www.w3.org/2000/svg"
        d = "M3 15h16a2 2 0 0 0 2 -2" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 6v5h17.5" />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 10l0 4" /><
        path xmlns = "http://www.w3.org/2000/svg" d = "M8 11l0 -5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M13 11l0 -4.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M3 19l18 0" /> < title > { title } < / title >
        < / svg >
    }
}
