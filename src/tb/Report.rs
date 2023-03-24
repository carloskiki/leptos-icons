#[cfg(feature = "TbReport")]
use leptos::*;
#[cfg(feature = "TbReport")]
///This icon requires the feature `TbReport` to be enabled.
#[component]
pub fn Report(
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
        stroke_witdh = "0" style = style class = "icon icon-tabler icon-tabler-report"
        width = "24" height = "24" viewBox = "0 0 24 24" stroke - width = "2" stroke =
        "currentColor" fill = "none" stroke - linecap = "round" stroke - linejoin =
        "round" width = { size.clone() } height = { size } > < path xmlns =
        "http://www.w3.org/2000/svg" stroke = "none" d = "M0 0h24v24H0z" fill = "none"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M8 5h-2a2 2 0 0 0 -2 2v12a2 2 0 0 0 2 2h5.697" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 14v4h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M18 11v-4a2 2 0 0 0 -2 -2h-2" />< path xmlns =
        "http://www.w3.org/2000/svg" d =
        "M8 3m0 2a2 2 0 0 1 2 -2h2a2 2 0 0 1 2 2v0a2 2 0 0 1 -2 2h-2a2 2 0 0 1 -2 -2z"
        />< path xmlns = "http://www.w3.org/2000/svg" d =
        "M18 18m-4 0a4 4 0 1 0 8 0a4 4 0 1 0 -8 0" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 11h4" />< path xmlns =
        "http://www.w3.org/2000/svg" d = "M8 15h3" /> < title > { title } < / title > < /
        svg >
    }
}
