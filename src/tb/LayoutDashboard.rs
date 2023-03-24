#[cfg(feature = "TbLayoutDashboard")]
use leptos::*;
#[cfg(feature = "TbLayoutDashboard")]
///This icon requires the feature `TbLayoutDashboard` to be enabled.
#[component]
pub fn LayoutDashboard(
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
        "icon icon-tabler icon-tabler-layout-dashboard" width = "24" height = "24"
        viewBox = "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none"
        stroke - linecap = "round" stroke - linejoin = "round" width = { size.clone() }
        height = { size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d
        = "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M4 4h6v8h-6z" />< path xmlns = "http://www.w3.org/2000/svg" d = "M4 16h6v4h-6z"
        />< path xmlns = "http://www.w3.org/2000/svg" d = "M14 12h6v8h-6z" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M14 4h6v4h-6z" /> < title > { title } < /
        title > < / svg >
    }
}
