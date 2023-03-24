#[cfg(feature = "TbBone")]
use leptos::*;
#[cfg(feature = "TbBone")]
///This icon requires the feature `TbBone` to be enabled.
#[component]
pub fn Bone(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-bone"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M15 3a3 3 0 0 1 3 3a3 3 0 1 1 -2.12 5.122l-4.758 4.758a3 3 0 1 1 -5.117 2.297l0 -.177l-.176 0a3 3 0 1 1 2.298 -5.115l4.758 -4.758a3 3 0 0 1 2.12 -5.122z"
        /> < title > { title } < / title > < / svg >
    }
}
