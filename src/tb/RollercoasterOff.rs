#[cfg(feature = "TbRollercoasterOff")]
use leptos::*;
#[cfg(feature = "TbRollercoasterOff")]
///This icon requires the feature `TbRollercoasterOff` to be enabled.
#[component]
pub fn RollercoasterOff(
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
        "icon icon-tabler icon-tabler-rollercoaster-off" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M3 21a5.55 5.55 0 0 0 5.265 -3.795l.735 -2.205a8.759 8.759 0 0 1 2.35 -3.652m2.403 -1.589a8.76 8.76 0 0 1 3.572 -.759h3.675"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M20 9v7m0 4v1" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M8 21v-3" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M12 21v-9" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 9.5v2.5m0 4v5" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 3h5v3h-5z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M9.446 5.415l.554 -.415l2 2.5l-.285 .213m-2.268 1.702l-1.447 1.085l-1.8 -.5l-.2 -2l1.139 -.854"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M3 3l18 18" /> < title > {
        title } < / title > < / svg >
    }
}
