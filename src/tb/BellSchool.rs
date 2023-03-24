#[cfg(feature = "TbBellSchool")]
use leptos::*;
#[cfg(feature = "TbBellSchool")]
///This icon requires the feature `TbBellSchool` to be enabled.
#[component]
pub fn BellSchool(
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
        "icon icon-tabler icon-tabler-bell-school" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M10 10m-6 0a6 6 0 1 0 12 0a6 6 0 1 0 -12 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M13.5 15h.5a2 2 0 0 1 2 2v1a2 2 0 0 1 -2 2h-8a2 2 0 0 1 -2 -2v-1a2 2 0 0 1 2 -2h.5"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M16 17a5.698 5.698 0 0 0 4.467 -7.932l-.467 -1.068" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M10 10v.01" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M20 8m-1 0a1 1 0 1 0 2 0a1 1 0 1 0 -2 0" /> <
        title > { title } < / title > < / svg >
    }
}
