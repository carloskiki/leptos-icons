#[cfg(feature = "TbScreenShare")]
use leptos::*;
#[cfg(feature = "TbScreenShare")]
///This icon requires the feature `TbScreenShare` to be enabled.
#[component]
pub fn ScreenShare(
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
        "icon icon-tabler icon-tabler-screen-share" width = "24" height = "24" viewBox =
        "0 0 24 24" stroke - width = "2" stroke = "currentColor" fill = "none" stroke -
        linecap = "round" stroke - linejoin = "round" width = { size.clone() } height = {
        size } > < path xmlns = "http://www.w3.org/2000/svg" stroke = "none" d =
        "M0 0h24v24H0z" fill = "none" />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M21 12v3a1 1 0 0 1 -1 1h-16a1 1 0 0 1 -1 -1v-10a1 1 0 0 1 1 -1h9" />< path xmlns
        = "http://www.w3.org/2000/svg" d = "M7 20l10 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M9 16l0 4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M15 16l0 4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M17 4h4v4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M16 9l5 -5" /> < title > { title } < / title >
        < / svg >
    }
}
