#[cfg(feature = "TbRecycle")]
use leptos::*;
#[cfg(feature = "TbRecycle")]
///This icon requires the feature `TbRecycle` to be enabled.
#[component]
pub fn Recycle(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-recycle"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M12 17l-2 2l2 2" />< path
        xmlns = "http://www.w3.org/2000/svg" d = "M10 19h9a2 2 0 0 0 1.75 -2.75l-.55 -1"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8.536 11l-.732 -2.732l-2.732 .732" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M7.804 8.268l-4.5 7.794a2 2 0 0 0 1.506 2.89l1.141 .024" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15.464 11l2.732 .732l.732 -2.732" />< path
        xmlns = "http://www.w3.org/2000/svg" d =
        "M18.196 11.732l-4.5 -7.794a2 2 0 0 0 -3.256 -.14l-.591 .976" /> < title > {
        title } < / title > < / svg >
    }
}
