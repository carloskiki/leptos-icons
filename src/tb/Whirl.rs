#[cfg(feature = "TbWhirl")]
use leptos::*;
#[cfg(feature = "TbWhirl")]
///This icon requires the feature `TbWhirl` to be enabled.
#[component]
pub fn Whirl(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-whirl"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M14 12a2 2 0 1 0 -4 0a2 2 0 0 0 4 0z" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 21c-3.314 0 -6 -2.462 -6 -5.5s2.686 -5.5 6 -5.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M21 12c0 3.314 -2.462 6 -5.5 6s-5.5 -2.686 -5.5 -6" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M12 14c3.314 0 6 -2.462 6 -5.5s-2.686 -5.5 -6 -5.5" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M14 12c0 -3.314 -2.462 -6 -5.5 -6s-5.5 2.686 -5.5 6" /> < title > { title } < /
        title > < / svg >
    }
}
